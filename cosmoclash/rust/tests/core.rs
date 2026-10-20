use cosmoclash_core::collision::*;
use cosmoclash_core::*;
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-10, "{a} != {b}");
}
fn no_input() -> [InputState; 2] {
    [InputState::default(); 2]
}
fn world(ships: [ShipId; 2]) -> World {
    let mut w = World::new(42, ships, None);
    w.asteroids.clear();
    w
}
#[test]
fn entity_motion_and_wrap_reset_sweep() {
    let mut e = Entity::new(0, EntityType::Player, 14.);
    e.position = Vec3::new(1499., 1049., 0.);
    e.accel_vec = Vec3::new(1., 1., 0.);
    e.update(20.);
    assert_eq!(e.previous_position, Vec3::new(1499., 1049., 0.));
    e.handle_wrap(Vec3::default());
    assert_eq!(e.position, Vec3::new(-1494., -1044., 0.));
    assert_eq!(e.previous_position, e.position);
}
#[test]
fn sweep_detects_fast_tunneling() {
    let r = sphere_sphere_sweep(
        2.,
        Vec3::new(-100., 0., 0.),
        Vec3::new(100., 0., 0.),
        2.,
        Vec3::default(),
        Vec3::default(),
    );
    assert!(r.hit);
    near(r.u0, 0.48);
    assert!(
        !sphere_sphere_sweep(
            2.,
            Vec3::new(-100., 5., 0.),
            Vec3::new(100., 5., 0.),
            2.,
            Vec3::default(),
            Vec3::default()
        )
        .hit
    );
}
#[test]
fn swept_sphere_offset_is_not_rotated() {
    let mut a = Player::new(0, ShipId::MichiGichi, Vec3::default());
    a.angle = 90.;
    let b = Entity {
        position: Vec3::new(0., 30., 0.),
        previous_position: Vec3::new(0., 30., 0.),
        ..Entity::new(1, EntityType::Player, 8.)
    };
    assert!(test_entity_pair(&a, &b).hit);
}
#[test]
fn cooldown_energy_and_strict_cost() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    w.players[0].energy = 0.12 - 0.0002 * 10.;
    let mut i = no_input();
    i[0].fire_primary = true;
    w.step(10., i).unwrap();
    assert!(w.bullets.is_empty());
    w.step(10., i).unwrap();
    assert_eq!(w.bullets.len(), 1);
    near(w.players[0].primary_cooldown, 100.);
    for _ in 0..9 {
        w.step(10., i).unwrap();
    }
    assert_eq!(w.bullets.len(), 1);
}
#[test]
fn sholev_radial_burst_is_world_aligned() {
    let mut w = world([ShipId::Sholev, ShipId::Gondi]);
    w.players[0].angle = 27.;
    let mut i = no_input();
    i[0].fire_primary = true;
    w.step(10., i).unwrap();
    assert_eq!(w.bullets.len(), 8);
    for (idx, b) in w.bullets.iter().enumerate() {
        near(b.angle, idx as f64 * 45.);
        assert_eq!(b.kind, BulletKind::Ice);
        near(b.anim_frame, 2.);
    }
    near(w.players[0].energy, 0.93);
}
#[test]
fn shield_toggle_is_edge_triggered_and_cooled() {
    let mut w = world([ShipId::Sholev, ShipId::Gondi]);
    let mut i = no_input();
    i[0].fire_secondary = true;
    w.step(20., i).unwrap();
    assert!(w.players[0].frost_shield_active);
    assert_eq!(w.bullets.len(), 1);
    for _ in 0..104 {
        w.step(20., i).unwrap();
    }
    assert!(w.players[0].frost_shield_active);
    w.step(20., no_input()).unwrap();
    w.step(20., i).unwrap();
    assert!(!w.players[0].frost_shield_active);
    assert!(w.bullets.is_empty());
    near(w.players[0].stats.speed_max_scaler, 0.8);
}
#[test]
fn shield_depletes_and_restores_stats() {
    let mut w = world([ShipId::Sholev, ShipId::Gondi]);
    w.players[0].energy = 0.25;
    let mut i = no_input();
    i[0].fire_secondary = true;
    w.step(100., i).unwrap();
    for _ in 0..30 {
        w.step(100., no_input()).unwrap();
    }
    assert!(!w.players[0].frost_shield_active);
    near(w.players[0].stats.rotation_speed, 0.5);
}
#[test]
fn shield_cannot_toggle_early() {
    let mut w = world([ShipId::Sholev, ShipId::Gondi]);
    let mut i = no_input();
    i[0].fire_secondary = true;
    w.step(20., i).unwrap();
    w.step(20., no_input()).unwrap();
    w.step(20., i).unwrap();
    assert!(w.players[0].frost_shield_active);
}
#[test]
fn morph_retains_current_weapon_until_completed() {
    let mut w = world([ShipId::MichiGichi, ShipId::Gondi]);
    let mut i = no_input();
    i[0].fire_secondary = true;
    w.step(20., i).unwrap();
    assert_eq!(w.players[0].morph_state, 1);
    assert_eq!(w.players[0].bullet_kind(), BulletKind::Plasma);
    for _ in 0..39 {
        w.step(20., no_input()).unwrap();
    }
    assert_eq!(w.players[0].morph_state, 2);
    near(w.players[0].stats.speed_max_scaler, 1.);
    w.step(20., i).unwrap();
    assert_eq!(w.players[0].bullet_kind(), BulletKind::Sun);
    for _ in 0..39 {
        w.step(20., no_input()).unwrap();
    }
    assert_eq!(w.players[0].morph_state, 0);
}
#[test]
fn attached_flame_follows_rotation_without_sweep() {
    let mut origin = Entity::new(1, EntityType::Player, 14.);
    origin.position = Vec3::new(100., 100., 0.);
    origin.angle = 90.;
    let mut b = Bullet::new(2, BulletKind::Flame, Some(1));
    b.local_offset_distance = 25.;
    b.attached_speed = 0.3;
    b.update(20., Some(&origin), None);
    near(b.position.x, 72.9);
    near(b.position.y, 100.);
    assert_eq!(b.position, b.previous_position);
}
#[test]
fn plasma_expands_decays_and_tracks() {
    let mut b = Bullet::new(2, BulletKind::Plasma, Some(0));
    b.accel_vec = Vec3::new(0., 0.3, 0.);
    let mut target = Entity::new(1, EntityType::Player, 14.);
    target.position = Vec3::new(100., 0., 0.);
    b.update(100., None, Some(&target));
    assert!(b.accel_vec.x > 0.);
    near(b.accel_vec.len(), 0.3);
    near(b.radius(), 0.7);
    near(b.current_damage(), 0.297);
}
#[test]
fn snot_attachment_penalty_and_expiration() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    let mut s = Snot::new(20, 0, 1);
    s.position = w.players[1].position;
    s.previous_position = s.position;
    w.snots.push(s);
    w.step(20., no_input()).unwrap();
    assert_eq!(w.players[1].snot_count, 1);
    assert!(w.snots[0].attached);
    near(w.players[1].get_effective_max_speed(), 0.6);
    w.snots[0].ttl = 10.;
    w.step(20., no_input()).unwrap();
    assert_eq!(w.players[1].snot_count, 0);
    assert!(w.snots.is_empty());
}
#[test]
fn snot_secondary_uses_actual_primary_cost() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    let mut i = no_input();
    i[0].fire_secondary = true;
    w.step(20., i).unwrap();
    near(w.players[0].energy, 0.88);
    assert_eq!(w.snots.len(), 1);
    near(w.players[0].secondary_cooldown, 200.);
}
#[test]
fn collision_damage_and_minimum_bounce() {
    let mut a = Entity::new(0, EntityType::Player, 14.);
    let mut b = Entity::new(1, EntityType::Player, 14.);
    a.position = Vec3::new(0., 0., 0.);
    b.position = Vec3::new(20., 0., 0.);
    a.accel_vec.x = 0.1;
    b.accel_vec.x = -0.1;
    ship_collision(&mut a, &mut b);
    near(a.health, 0.96);
    near(b.health, 0.96);
    near(b.position.x - a.position.x, 36.);
    near(a.accel_vec.x, -0.4);
    near(b.accel_vec.x, 0.4);
}
#[test]
fn asteroid_collision_bounces_without_damage_or_push() {
    let mut a = Entity::new(0, EntityType::Player, 14.);
    let mut b = Entity::new(1, EntityType::Asteroid, 18.);
    b.position.x = 20.;
    a.accel_vec.x = 0.2;
    b.accel_vec.x = -0.1;
    ship_collision(&mut a, &mut b);
    near(a.health, 1.);
    near(b.health, 1.);
    near(a.position.x, 0.);
    near(a.accel_vec.x, -0.2);
    near(b.accel_vec.x, 0.1);
}
#[test]
fn sun_damage_and_bounce() {
    let sun = Entity::new(0, EntityType::Sun, 65.);
    let mut p = Entity::new(1, EntityType::Player, 14.);
    p.position.x = 50.;
    sun_collision(&sun, &mut p);
    near(p.health, 0.98);
    near(p.position.x, 87.);
    near(p.accel_vec.x, 0.4);
}
#[test]
fn minimum_distance_cancels_approaching_components() {
    let mut a = Entity::new(0, EntityType::Player, 14.);
    let mut b = Entity::new(1, EntityType::Player, 14.);
    b.position.x = 30.;
    a.accel_vec = Vec3::new(0.1, 0.2, 0.);
    b.accel_vec = Vec3::new(-0.1, -0.2, 0.);
    enforce_minimum_distance(&mut a, &mut b);
    near(a.accel_vec.x, 0.);
    near(a.accel_vec.y, 0.2);
    near(b.position.x, 30.);
}
#[test]
fn gravity_boundary_and_center() {
    let mut e = Entity::new(0, EntityType::Player, 14.);
    apply_sun_gravity(&mut e, Vec3::default(), 20.);
    assert_eq!(e.accel_vec, Vec3::default());
    e.position.x = 450.;
    apply_sun_gravity(&mut e, Vec3::default(), 20.);
    near(e.accel_vec.x, -0.007);
    e.accel_vec = Vec3::default();
    e.position.x = 450.00001;
    apply_sun_gravity(&mut e, Vec3::default(), 20.);
    near(e.accel_vec.x, 0.);
}
#[test]
fn player_wrap_advances_both_and_resets_previous() {
    let mut p = [
        Player::new(0, ShipId::Gondi, Vec3::new(0., 0., 0.)),
        Player::new(1, ShipId::Gondi, Vec3::new(1600., 0., 0.)),
    ];
    p[0].accel_vec.y = 1.;
    p[1].accel_vec.x = 1.;
    assert!(handle_player_wrap(&mut p));
    near(p[1].position.x, -1360.);
    near(p[0].position.y, 140.);
    assert_eq!(p[0].position, p[0].previous_position);
    assert_eq!(p[1].position, p[1].previous_position);
}
#[test]
fn projectile_does_not_hit_origin_or_projectiles() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    for id in [20, 21] {
        let mut b = Bullet::new(id, BulletKind::Sun, Some(0));
        b.position = w.players[0].position;
        b.previous_position = b.position;
        w.bullets.push(b);
    }
    w.step(20., no_input()).unwrap();
    near(w.players[0].health, 1.);
    assert_eq!(w.bullets.len(), 2);
}
#[test]
fn projectile_destroys_asteroid_and_animates_explosion() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    let mut a = Asteroid::new(20, 0, &mut w.rng);
    a.position = Vec3::new(200., 200., 0.);
    a.previous_position = a.position;
    let mut b = Bullet::new(21, BulletKind::Ice, Some(0));
    b.position = a.position;
    b.previous_position = b.position;
    w.asteroids.push(a);
    w.bullets.push(b);
    w.step(20., no_input()).unwrap();
    assert!(w.asteroids.is_empty());
    assert!(w.bullets.is_empty());
    assert_eq!(w.explosions.len(), 1);
    near(w.explosions[0].current_frame, 0.8);
    assert_eq!(w.explosions[0].source_type, EntityType::Asteroid);
}
#[test]
fn frost_rate_limit_is_global_to_bullet() {
    let mut w = world([ShipId::Sholev, ShipId::Gondi]);
    w.players[1].health = 1.;
    let mut b = Bullet::new(20, BulletKind::Frost, Some(0));
    b.position = w.players[1].position;
    b.previous_position = b.position;
    w.bullets.push(b);
    w.step(20., no_input()).unwrap();
    near(w.players[1].health, 0.5);
    w.bullets[0].position = w.players[1].position;
    w.bullets[0].previous_position = w.players[1].position;
    w.step(20., no_input()).unwrap();
    near(w.players[1].health, 0.5);
    near(w.bullets[0].damage_cooldown, 460.);
}
#[test]
fn winner_freezes_battle_but_explosion_keeps_playing() {
    let mut w = world([ShipId::Gondi, ShipId::Gondi]);
    w.players[1].health = 0.01;
    let mut b = Bullet::new(20, BulletKind::Sun, Some(0));
    b.position = w.players[1].position;
    b.previous_position = b.position;
    w.bullets.push(b);
    w.step(20., no_input()).unwrap();
    assert_eq!(w.winner, Some(1));
    assert_eq!(w.explosions.len(), 1);
    let pos = w.players[0].position;
    let old = w.explosions[0].current_frame;
    let mut input = no_input();
    input[0].thrust = true;
    w.step(20., input).unwrap();
    assert_eq!(pos, w.players[0].position);
    assert!(w.explosions[0].current_frame > old);
}
#[test]
fn malformed_requests_are_transactional() {
    let mut e = abi::Engine::default();
    assert_eq!(e.request(br#"{"op":"snapshot"}"#)["ok"], false);
    assert_eq!(
        e.request(br#"{"op":"init","seed":1,"ships":["gondi","sholev"]}"#)["ok"],
        true
    );
    let before = serde_json::to_string(&e.world).unwrap();
    for request in [
        r#"{"op":"step","dt":0,"inputs":[{},{}]}"#,
        r#"{"op":"step","dt":101,"inputs":[{},{}]}"#,
        r#"{"op":"step","dt":1e-320,"inputs":[{},{}]}"#,
        r#"{"op":"step","dt":16,"inputs":[{"bogus":true},{}]}"#,
        r#"{"op":"init","seed":1,"ships":["fake","gondi"]}"#,
        r#"{"op":"init","seed":1,"ships":["gondi","gondi"],"aiDifficulty":"invalid"}"#,
    ] {
        assert_eq!(e.request(request.as_bytes())["ok"], false);
        assert_eq!(serde_json::to_string(&e.world).unwrap(), before);
    }
}
#[test]
fn abi_rejects_untracked_allocations() {
    assert_eq!(abi::cosmo_alloc(0), 0);
    assert_eq!(abi::cosmo_alloc(abi::MAX_REQUEST_BYTES + 1), 0);
    let p = abi::cosmo_alloc(12);
    assert_ne!(p, 0);
    abi::cosmo_dealloc(p, 13);
    abi::cosmo_request(usize::MAX, usize::MAX);
    assert!(abi::cosmo_result_len() > 0);
    abi::cosmo_dealloc(p, 12);
    abi::cosmo_dealloc(p, 12);
}
#[test]
fn deterministic_all_matchups_and_ai_difficulties() {
    let ships = [ShipId::Gondi, ShipId::MichiGichi, ShipId::Sholev];
    for &p in &ships {
        for &q in &ships {
            for difficulty in [None, Some("easy"), Some("medium"), Some("hard")] {
                let mut a = World::new(123, [p, q], difficulty.map(str::to_owned));
                let mut b = a.clone();
                for frame in 0..600 {
                    let input = [InputState {
                        thrust: true,
                        rotate_left: frame % 270 < 135,
                        rotate_right: frame % 270 >= 135,
                        fire_primary: true,
                        fire_secondary: frame % 180 == 0,
                    }; 2];
                    a.step(1000. / 60., input).unwrap();
                    b.step(1000. / 60., input).unwrap();
                }
                assert_eq!(
                    serde_json::to_string(&a).unwrap(),
                    serde_json::to_string(&b).unwrap()
                );
                assert!(a
                    .players
                    .iter()
                    .all(|p| p.position.x.is_finite() && p.energy.is_finite()));
            }
        }
    }
}

#[test]
fn all_request_variants_reject_unknown_fields_without_mutation() {
    let mut engine = abi::Engine::default();
    assert_eq!(
        engine.request(br#"{"op":"init","seed":1,"ships":["gondi","sholev"]}"#)["ok"],
        true
    );
    let initial = engine.request(br#"{"op":"snapshot"}"#);
    for request in [
        r#"null"#,
        r#"{}"#,
        r#"{"op":"snapshot","unexpected":true}"#,
        r#"{"op":"step","dt":16,"inputs":[{},{}],"unexpected":true}"#,
        r#"{"op":"init","seed":1,"ships":["gondi","sholev"],"unexpected":true}"#,
        r#"{"op":"step","dt":16,"inputs":[{"thrust":1},{}]}"#,
        r#"{"op":"init","seed":-1,"ships":["gondi","sholev"]}"#,
        r#"{"op":"unknown"}"#,
        r#"{"op":"snapshot","op":"init"}"#,
    ] {
        let result = engine.request(request.as_bytes());
        assert_eq!(result["ok"], false, "request was accepted: {request}");
        assert_eq!(engine.request(br#"{"op":"snapshot"}"#), initial);
    }
}
