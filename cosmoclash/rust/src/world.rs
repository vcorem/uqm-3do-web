use crate::collision::*;
use crate::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub position: Vec3,
    pub zoom_dist: f64,
}
impl Camera {
    fn new() -> Self {
        Self {
            position: Vec3::new(0., -70., -5100.),
            zoom_dist: -1.,
        }
    }
    fn update(&mut self, p1: Vec3, p2: Vec3, dt: f64, immediate: bool) {
        let mx = (p1.x + p2.x) / 2.;
        let my = (p1.y + p2.y) / 2.;
        if immediate {
            self.position.x = mx;
            self.position.y = my;
        } else {
            let dx = mx - self.position.x;
            let dy = my - self.position.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 0.01 {
                let move_len = (dt * dist * 0.005).min(dist);
                self.position.x += (dx / dist) * move_len;
                self.position.y += (dy / dist) * move_len;
            }
        }
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let pd = (dx * dx + dy * dy).sqrt();
        if self.zoom_dist < 0. {
            self.zoom_dist = pd;
        } else {
            let avgr = 400. / dt;
            self.zoom_dist = (self.zoom_dist * (avgr - 1.) / avgr) + (pd / avgr);
        }
        self.position.z = (-100. - self.zoom_dist.max(50.) * 5.).max(-3500.);
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub kind: String,
    pub sound: Option<String>,
    pub entity_id: Option<u32>,
    pub target_id: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub tick: u64,
    pub elapsed_ms: f64,
    pub players: [Player; 2],
    pub bullets: Vec<Bullet>,
    pub snots: Vec<Snot>,
    pub asteroids: Vec<Asteroid>,
    pub explosions: Vec<Explosion>,
    pub sun: Sun,
    pub camera: Camera,
    pub winner: Option<u8>,
    pub events: Vec<Event>,
    pub rng: Rng,
    pub next_id: u32,
    #[serde(default)]
    pub ai_difficulty: Option<String>,
    #[serde(default)]
    pub ai_controller: Option<crate::ai::AiController>,
}
#[derive(Clone, Copy)]
enum Ref {
    Player(usize),
    Sun,
    Bullet(usize),
    Snot(usize),
    Asteroid(usize),
}
impl World {
    pub fn new(seed: u32, ships: [ShipId; 2], ai_difficulty: Option<String>) -> Self {
        let mut world = Self {
            tick: 0,
            elapsed_ms: 0.,
            players: [
                Player::new(0, ships[0], Vec3::new(500., -70., 0.)),
                Player::new(1, ships[1], Vec3::new(-500., -70., 0.)),
            ],
            bullets: vec![],
            snots: vec![],
            asteroids: vec![],
            explosions: vec![],
            sun: Sun {
                entity: Entity::new(2, EntityType::Sun, 65.),
                current_frame: 0.,
            },
            camera: Camera::new(),
            winner: None,
            events: vec![],
            rng: Rng::new(seed),
            next_id: 3,
            ai_difficulty,
            ai_controller: None,
        };
        // The host's preallocated render pools are deliberately absent: only gameplay
        // spawning consumes randomness, identically on both peers.
        for _ in 0..10 {
            let kind = (world.rng.next_f64() * 3.).floor() as u8;
            let id = world.id();
            let mut a = Asteroid::new(id, kind, &mut world.rng);
            a.position = Vec3::new(
                WRAP_X - world.rng.next_f64() * WRAP_X * 2.,
                WRAP_Y - world.rng.next_f64() * WRAP_Y * 2.,
                0.,
            );
            a.previous_position = a.position;
            world.asteroids.push(a);
        }
        if let Some(level) = world.ai_difficulty.as_deref() {
            world.ai_controller = Some(crate::ai::AiController::new(match level {
                "easy" => 1,
                "medium" => 2,
                _ => 3,
            }));
        }
        world
    }
    fn id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    fn sound(&mut self, sound: &str, entity: Option<u32>) {
        self.events.push(Event {
            kind: "sound".into(),
            sound: Some(sound.into()),
            entity_id: entity,
            target_id: None,
        });
    }
    fn explode(&mut self, e: Entity) {
        let id = self.id();
        let mut entity = Entity::new(id, EntityType::Explosion, -1.);
        entity.position = e.position;
        entity.previous_position = e.position;
        self.explosions.push(Explosion {
            entity,
            current_frame: 0.,
            source_type: e.entity_type,
        });
        self.sound(
            if e.entity_type == EntityType::Asteroid {
                "Explosion"
            } else {
                "ExplosionBig"
            },
            Some(e.id),
        );
        self.events.push(Event {
            kind: "explosion".into(),
            sound: None,
            entity_id: Some(e.id),
            target_id: None,
        });
    }
    fn entity(&self, r: Ref) -> &Entity {
        match r {
            Ref::Player(i) => &self.players[i].entity,
            Ref::Sun => &self.sun.entity,
            Ref::Bullet(i) => &self.bullets[i].entity,
            Ref::Snot(i) => &self.snots[i].bullet.entity,
            Ref::Asteroid(i) => &self.asteroids[i].entity,
        }
    }
    fn put_entity(&mut self, r: Ref, e: Entity) {
        match r {
            Ref::Player(i) => self.players[i].entity = e,
            Ref::Sun => self.sun.entity = e,
            Ref::Bullet(i) => self.bullets[i].entity = e,
            Ref::Snot(i) => self.snots[i].bullet.entity = e,
            Ref::Asteroid(i) => self.asteroids[i].entity = e,
        }
    }
    fn projectile_origin(&self, r: Ref) -> Option<u32> {
        match r {
            Ref::Bullet(i) => self.bullets[i].origin_entity,
            Ref::Snot(i) => self.snots[i].origin_entity,
            _ => None,
        }
    }
    fn should_check(&self, a: Ref, b: Ref) -> bool {
        let at = self.entity(a).entity_type;
        let bt = self.entity(b).entity_type;
        let bullet = EntityType::Bullet;
        let snot = EntityType::Snot;
        let ast = EntityType::Asteroid;
        if at == bullet && bt == bullet {
            return false;
        }
        if at == snot && (bt == snot || bt == bullet) || bt == snot && (at == snot || at == bullet)
        {
            return false;
        }
        if matches!(a,Ref::Snot(i) if self.snots[i].attached)
            || matches!(b,Ref::Snot(i) if self.snots[i].attached)
        {
            return false;
        }
        if self.projectile_origin(a) == Some(self.entity(b).id)
            || self.projectile_origin(b) == Some(self.entity(a).id)
        {
            return false;
        }
        if at == ast && (bt == ast || bt == snot) || bt == ast && at == snot {
            return false;
        }
        true
    }
    fn detect_collisions(&mut self) {
        let mut entities = vec![Ref::Player(0), Ref::Player(1), Ref::Sun];
        entities.extend((0..self.bullets.len()).map(Ref::Bullet));
        entities.extend((0..self.snots.len()).map(Ref::Snot));
        entities.extend((0..self.asteroids.len()).map(Ref::Asteroid));
        for (i, &a) in entities.iter().enumerate() {
            if !self.entity(a).alive || self.entity(a).collision_spheres.is_empty() {
                continue;
            }
            for &b in &entities[i + 1..] {
                if !self.entity(b).alive
                    || self.entity(b).collision_spheres.is_empty()
                    || !self.should_check(a, b)
                {
                    continue;
                }
                if test_entity_pair(self.entity(a), self.entity(b)).hit {
                    self.collide(a, b);
                }
            }
        }
    }
    fn collide(&mut self, a: Ref, b: Ref) {
        if let Ref::Snot(i) = a {
            self.snot_hit(i, b);
        } else if let Ref::Snot(i) = b {
            self.snot_hit(i, a);
        } else if let Ref::Bullet(i) = a {
            self.bullet_hit(i, b);
        } else if let Ref::Bullet(i) = b {
            self.bullet_hit(i, a);
        } else if matches!(a, Ref::Sun) {
            self.sun_hit(b);
        } else if matches!(b, Ref::Sun) {
            self.sun_hit(a);
        } else {
            let mut ea = self.entity(a).clone();
            let mut eb = self.entity(b).clone();
            let deaths = ship_collision(&mut ea, &mut eb);
            self.put_entity(a, ea);
            self.put_entity(b, eb);
            for e in deaths {
                self.explode(e);
            }
        }
    }
    fn snot_hit(&mut self, i: usize, target: Ref) {
        let t = self.entity(target).clone();
        if let Ref::Player(p) = target {
            self.snots[i].attach(t.id, &mut self.rng);
            self.players[p].snot_count += 1;
        } else {
            self.snots[i].alive = false;
        }
        self.sound("SnotHit", Some(t.id));
    }
    fn bullet_hit(&mut self, i: usize, target: Ref) {
        if self.bullets[i].config.persistent && self.bullets[i].damage_cooldown > 0. {
            let t = self.entity(target);
            let id = t.id;
            let typ = t.entity_type;
            if typ == EntityType::Asteroid {
                self.sound("AsteroidHit", Some(id));
            } else if typ == EntityType::Player {
                self.sound("Hit", Some(id));
            }
            return;
        }
        let mut t = self.entity(target).clone();
        let mut death = None;
        if t.entity_type == EntityType::Sun {
        } else if t.entity_type == EntityType::Asteroid {
            t.alive = false;
            death = Some(t.clone());
        } else {
            t.health -= self.bullets[i].current_damage();
            if t.health <= 0. {
                t.alive = false;
                if t.entity_type == EntityType::Player {
                    death = Some(t.clone());
                }
            }
        }
        self.put_entity(target, t.clone());
        if self.bullets[i].config.persistent {
            self.bullets[i].damage_cooldown = self.bullets[i].config.hit_rate_limit;
        } else {
            self.bullets[i].alive = false;
        }
        if let Some(d) = death {
            self.explode(d);
        }
        if t.entity_type == EntityType::Asteroid {
            self.sound("AsteroidHit", Some(t.id));
        } else if t.entity_type == EntityType::Player {
            self.sound("Hit", Some(t.id));
        }
    }
    fn sun_hit(&mut self, target: Ref) {
        let mut t = self.entity(target).clone();
        let death = sun_collision(&self.sun.entity, &mut t);
        let id = t.id;
        self.put_entity(target, t);
        if let Some(d) = death {
            self.explode(d);
        }
        self.sound("SunBoom", Some(id));
    }
    fn player_input(&mut self, p: usize, input: InputState, dt: f64) {
        let player = &mut self.players[p];
        if input.rotate_left {
            player.rotate(-1., dt);
        } else if input.rotate_right {
            player.rotate(1., dt);
        } else {
            player.reset_roll(dt);
        }
        if input.thrust {
            player.thrust(dt);
        }
        player.update_energy(dt);
        if input.fire_primary
            && player.primary_cooldown <= 0.
            && player.energy > player.stats.primary_energy_cost
        {
            player.energy -= player.stats.primary_energy_cost;
            player.primary_cooldown = player.stats.primary_weapon_delay;
            let player = player.clone();
            let kind = player.bullet_kind();
            self.sound(kind.config().sound.as_deref().unwrap(), Some(player.id));
            let shards = if player.ship_id == ShipId::Sholev {
                8
            } else {
                1
            };
            for shard in 0..shards {
                let id = self.id();
                let mut bullet = Bullet::new(id, kind, Some(player.id));
                bullet.angle = if shards == 1 {
                    player.angle
                } else {
                    (360. / f64::from(shards)) * f64::from(shard)
                };
                bullet.position = player.position.plus(Vec3::heading(bullet.angle, 25.));
                bullet.previous_position = bullet.position;
                bullet.accel_vec = Vec3::heading(bullet.angle, bullet.config.speed_max_scaler);
                if bullet.config.tracking {
                    bullet.tracking_target = Some(self.players[1 - p].id);
                }
                if bullet.config.attached {
                    bullet.attached_speed = bullet.config.speed_max_scaler;
                    bullet.local_offset_distance = 25.;
                }
                self.bullets.push(bullet);
            }
        }
        if input.fire_secondary {
            match self.players[p].ship_id {
                ShipId::MichiGichi => {
                    let state = self.players[p].morph_state;
                    if state == 0 || state == 2 {
                        self.sound(
                            if state == 0 {
                                "MorphOpen"
                            } else {
                                "MorphClose"
                            },
                            Some(self.players[p].id),
                        );
                    }
                    self.players[p].trigger_morph();
                }
                ShipId::Sholev => {
                    if !self.players[p].previous_secondary {
                        if self.players[p].frost_shield_active {
                            if self.players[p].frost_shield_active_time >= 2000. {
                                if let Some(id) = self.players[p].active_frost_bullet {
                                    if let Some(b) = self.bullets.iter_mut().find(|b| b.id == id) {
                                        b.alive = false;
                                    }
                                }
                                self.players[p].deactivate_shield();
                            }
                        } else if self.players[p].energy >= 0.25 {
                            self.players[p].frost_shield_active = true;
                            self.players[p].frost_shield_active_time = 0.;
                            self.players[p].stats =
                                CharacterStats::for_ship(ShipId::Sholev, false, true);
                            let id = self.id();
                            let mut b =
                                Bullet::new(id, BulletKind::Frost, Some(self.players[p].id));
                            b.position = self.players[p].position;
                            b.previous_position = b.position;
                            self.players[p].active_frost_bullet = Some(id);
                            self.bullets.push(b);
                            self.sound("FrostShot", Some(self.players[p].id));
                        }
                    }
                }
                ShipId::Gondi => {
                    let player = &mut self.players[p];
                    if player.secondary_cooldown <= 0.
                        && player.energy > player.stats.primary_energy_cost
                    {
                        player.energy -= player.stats.primary_energy_cost;
                        player.secondary_cooldown = 200.;
                        let player = player.clone();
                        let id = self.id();
                        let mut s = Snot::new(id, player.id, self.players[1 - p].id);
                        s.angle = player.angle;
                        s.position = player.position.plus(Vec3::heading(player.angle, 25.));
                        s.previous_position = s.position;
                        s.accel_vec = Vec3::heading(player.angle, 0.2);
                        self.snots.push(s);
                        self.sound("SnotShot", Some(player.id));
                    }
                }
            }
        }
        self.players[p].previous_secondary = input.fire_secondary;
    }
    fn update_shield(&mut self, p: usize, dt: f64) {
        if !self.players[p].frost_shield_active {
            return;
        }
        if let Some(id) = self.players[p].active_frost_bullet {
            self.players[p].frost_shield_active_time += dt;
            if let Some(b) = self.bullets.iter_mut().find(|b| b.id == id) {
                if !b.alive {
                    self.players[p].deactivate_shield();
                    return;
                }
                self.players[p].energy -= 0.0003 * dt;
                if self.players[p].energy <= 0. {
                    self.players[p].energy = 0.;
                    b.alive = false;
                    self.players[p].deactivate_shield();
                }
            } else {
                self.players[p].deactivate_shield();
            }
        }
    }
    pub fn step(&mut self, dt: f64, mut inputs: [InputState; 2]) -> Result<(), String> {
        if !dt.is_finite() || !(0.001..=100.).contains(&dt) {
            return Err("dt must be finite and in [0.001,100] milliseconds".into());
        }
        self.events.clear();
        self.tick += 1;
        self.elapsed_ms += dt;
        let cam = self.camera.position;
        let mut wrapped = false;
        if self.winner.is_none() {
            self.detect_collisions();
            let [a, b] = &mut self.players;
            enforce_minimum_distance(&mut a.entity, &mut b.entity);
            self.player_input(0, inputs[0], dt);
            // AI input is generated from the state after player one's input, as in main.ts.
            if self.ai_difficulty.is_some() {
                inputs[1] = self.ai_input(dt);
            }
            self.player_input(1, inputs[1], dt);
            for p in 0..2 {
                if self.players[p].ship_id == ShipId::MichiGichi {
                    self.players[p].update_morph(dt);
                }
            }
            for p in 0..2 {
                if self.players[p].ship_id == ShipId::Sholev {
                    self.update_shield(p, dt);
                }
            }
            for p in &mut self.players {
                if p.alive {
                    p.entity.update(dt);
                }
            }
            self.sun.previous_position = self.sun.position;
            self.sun.handle_wrap(cam);
            for b in &mut self.bullets {
                let origin = self
                    .players
                    .iter()
                    .find(|p| Some(p.id) == b.origin_entity)
                    .map(|p| &p.entity);
                let target = self
                    .players
                    .iter()
                    .find(|p| Some(p.id) == b.tracking_target)
                    .map(|p| &p.entity);
                b.update(dt, origin, target);
                b.handle_wrap(cam);
            }
            let mut detached = vec![];
            for s in &mut self.snots {
                let target = self.players.iter().find(|p| p.id == s.target).unwrap();
                let victim = self
                    .players
                    .iter()
                    .find(|p| Some(p.id) == s.victim)
                    .map(|p| &p.entity);
                if let Some(id) = s.update(dt, &target.entity, victim) {
                    detached.push(id);
                }
                s.handle_wrap(cam);
            }
            for id in detached {
                if let Some(p) = self.players.iter_mut().find(|p| p.id == id) {
                    p.snot_count -= 1;
                }
            }
            for a in &mut self.asteroids {
                a.update(dt);
                a.handle_wrap(cam);
            }
            for p in &mut self.players {
                apply_sun_gravity(&mut p.entity, self.sun.position, dt);
            }
            wrapped = handle_player_wrap(&mut self.players);
            if !self.players[0].alive || !self.players[1].alive {
                self.winner = Some(if self.players[0].alive { 1 } else { 2 });
                self.events.push(Event {
                    kind: "winner".into(),
                    sound: None,
                    entity_id: self.winner.map(u32::from),
                    target_id: None,
                });
            }
        }
        self.sun.current_frame += 0.025 * dt;
        if self.sun.current_frame >= 60. {
            self.sun.current_frame -= 60.;
        }
        self.camera.update(
            self.players[0].position,
            self.players[1].position,
            dt,
            wrapped,
        );
        for b in &mut self.bullets {
            b.animate(dt);
        }
        for e in &mut self.explosions {
            e.current_frame += 0.04 * dt;
            if e.current_frame > 81. {
                e.alive = false;
            }
            e.handle_wrap(cam);
        }
        self.bullets.retain(|b| b.alive);
        self.snots.retain(|s| s.alive);
        self.asteroids.retain(|a| a.alive);
        self.explosions.retain(|e| e.alive);
        Ok(())
    }
    pub(crate) fn ai_player(&self, p: usize) -> crate::ai::AiPlayer {
        use crate::ai::{AiPlayer, AiShot, AiShotConfig, AiSnot, Point};
        let player = &self.players[p];
        AiPlayer {
            position: Point {
                x: player.position.x,
                y: player.position.y,
            },
            angle: player.angle,
            accelVec: Point {
                x: player.accel_vec.x,
                y: player.accel_vec.y,
            },
            health: player.health,
            energy: player.energy,
            maxSpeed: player.get_effective_max_speed(),
            snotCount: f64::from(player.snot_count),
            entityType: match player.ship_id {
                ShipId::Gondi => 4.,
                ShipId::MichiGichi => 1.,
                ShipId::Sholev => 8.,
            },
            morphState: if player.ship_id == ShipId::MichiGichi {
                Some(player.morph_state)
            } else {
                None
            },
            shots: self
                .bullets
                .iter()
                .filter(|b| b.origin_entity == Some(player.id))
                .map(|b| AiShot {
                    position: Point {
                        x: b.position.x,
                        y: b.position.y,
                    },
                    config: AiShotConfig {
                        tracking: b.config.tracking,
                    },
                })
                .collect(),
            snots: self
                .snots
                .iter()
                .filter(|s| s.origin_entity == Some(player.id))
                .map(|s| AiSnot {
                    position: Point {
                        x: s.position.x,
                        y: s.position.y,
                    },
                    attached: s.attached,
                })
                .collect(),
        }
    }
    pub(crate) fn ai_input(&mut self, _dt: f64) -> InputState {
        let ai = self.ai_player(1);
        let user = self.ai_player(0);
        let rng = &mut self.rng;
        let output = self
            .ai_controller
            .as_mut()
            .expect("AI initialized with world")
            .update(&ai, &user, &mut || rng.next_f64());
        InputState {
            thrust: output.thrust,
            rotate_left: output.rotateLeft,
            rotate_right: output.rotateRight,
            fire_primary: output.firePrimary,
            fire_secondary: output.fireSecondary,
        }
    }
}
pub fn handle_player_wrap(players: &mut [Player; 2]) -> bool {
    let [p1, p2] = players;
    let dx = p2.position.x - p1.position.x;
    let dy = p2.position.y - p1.position.y;
    let x_far = p2.position.x * p2.position.x > p1.position.x * p1.position.x;
    let y_far = p2.position.y * p2.position.y > p1.position.y * p1.position.y;
    let mut wrapped = false;
    if dx > WRAP_X {
        if x_far {
            p2.position.x = p1.position.x - WRAP_X;
        } else {
            p1.position.x = p2.position.x + WRAP_X;
        }
        wrapped = true;
    } else if dx < -WRAP_X {
        if x_far {
            p2.position.x = p1.position.x + WRAP_X;
        } else {
            p1.position.x = p2.position.x - WRAP_X;
        }
        wrapped = true;
    }
    if dy > WRAP_Y {
        if y_far {
            p2.position.y = p1.position.y - WRAP_Y;
        } else {
            p1.position.y = p2.position.y + WRAP_Y;
        }
        wrapped = true;
    } else if dy < -WRAP_Y {
        if y_far {
            p2.position.y = p1.position.y + WRAP_Y;
        } else {
            p1.position.y = p2.position.y - WRAP_Y;
        }
        wrapped = true;
    }
    if wrapped {
        let t = 400. * TIME_SCALER;
        p1.position.x += p1.accel_vec.x * t;
        p1.position.y += p1.accel_vec.y * t;
        p2.position.x += p2.accel_vec.x * t;
        p2.position.y += p2.accel_vec.y * t;
        p1.previous_position = p1.position;
        p2.previous_position = p2.position;
    }
    wrapped
}
