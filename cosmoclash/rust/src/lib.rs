//! Deterministic source port of Cosmo Clash's TypeScript gameplay engine.
//! Three.js/audio/DOM stay in the host; all battle decisions and state live here.
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
pub mod abi;
pub mod ai;
pub mod collision;
pub mod world;
pub use world::*;

pub const TIME_SCALER: f64 = 0.35;
pub const WRAP_X: f64 = 1500.;
pub const WRAP_Y: f64 = 1050.;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn plus(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
    pub fn minus(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
    pub fn dot(self, b: Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn heading(angle: f64, speed: f64) -> Self {
        let r = angle * (std::f64::consts::PI / 180.);
        Self::new(-r.sin() * speed, r.cos() * speed, 0.)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionSphere {
    pub offset: Vec3,
    pub radius: f64,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Bullet,
    Sun,
    Dummy,
    Snot,
    Explosion,
    Asteroid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: u32,
    pub position: Vec3,
    pub previous_position: Vec3,
    pub accel_vec: Vec3,
    pub angle: f64,
    pub alive: bool,
    pub auto_wrap: bool,
    pub health: f64,
    pub entity_type: EntityType,
    pub collision_spheres: Vec<CollisionSphere>,
}
impl Entity {
    pub fn new(id: u32, entity_type: EntityType, radius: f64) -> Self {
        Self {
            id,
            position: Vec3::default(),
            previous_position: Vec3::default(),
            accel_vec: Vec3::default(),
            angle: 0.,
            alive: true,
            auto_wrap: true,
            health: 1.,
            entity_type,
            collision_spheres: if radius < 0. {
                vec![]
            } else {
                vec![CollisionSphere {
                    offset: Vec3::default(),
                    radius,
                }]
            },
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.previous_position = self.position;
        let t = dt * TIME_SCALER;
        self.position.x += self.accel_vec.x * t;
        self.position.y += self.accel_vec.y * t;
        self.position.z += self.accel_vec.z * t;
    }
    pub fn correct_angle(&mut self) {
        if self.angle >= 360. {
            self.angle -= 360.;
        }
        if self.angle < 0. {
            self.angle += 360.;
        }
    }
    pub fn handle_wrap(&mut self, center: Vec3) {
        if !self.auto_wrap {
            return;
        }
        let dx = self.position.x - center.x;
        let dy = self.position.y - center.y;
        let mut wrapped = false;
        if dx > WRAP_X {
            self.position.x = center.x - WRAP_X + (dx - WRAP_X);
            wrapped = true;
        } else if dx < -WRAP_X {
            self.position.x = center.x + WRAP_X + (dx + WRAP_X);
            wrapped = true;
        }
        if dy > WRAP_Y {
            self.position.y = center.y - WRAP_Y + (dy - WRAP_Y);
            wrapped = true;
        } else if dy < -WRAP_Y {
            self.position.y = center.y + WRAP_Y + (dy + WRAP_Y);
            wrapped = true;
        }
        if wrapped {
            self.previous_position = self.position;
        }
    }
    pub fn radius(&self) -> f64 {
        self.collision_spheres.first().map_or(0., |s| s.radius)
    }
}
macro_rules! entity_deref {
    ($name:ty,$field:ident,$target:ty) => {
        impl Deref for $name {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ShipId {
    Gondi,
    MichiGichi,
    Sholev,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BulletKind {
    Flame,
    Plasma,
    Sun,
    Ice,
    Frost,
    Snot,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandingRadius {
    pub min: f64,
    pub max: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayingDamage {
    pub initial: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletConfig {
    pub speed_max_scaler: f64,
    pub max_ttl: f64,
    pub damage: f64,
    pub collision_radius: f64,
    pub tracking: bool,
    pub expanding_radius: Option<ExpandingRadius>,
    pub decaying_damage: Option<DecayingDamage>,
    pub sound: Option<String>,
    pub attached: bool,
    pub persistent: bool,
    pub hit_rate_limit: f64,
}
impl BulletKind {
    pub fn config(self) -> BulletConfig {
        let (speed, ttl, damage, radius, sound) = match self {
            Self::Flame => (0.3, 700., 0.05, 10., "FlameShot"),
            Self::Plasma => (0.3, 10000., 0.3, 0., "PlasmaShot"),
            Self::Sun => (1.5, 1000., 0.1, 2., "SunShot"),
            Self::Ice => (1.5, 700., 0.03, 1., "IceShot"),
            Self::Frost => (0., 500000., 0.5, 11., "FrostShot"),
            Self::Snot => (0.2, 10000., 0., 2., "SnotShot"),
        };
        BulletConfig {
            speed_max_scaler: speed,
            max_ttl: ttl,
            damage,
            collision_radius: radius,
            tracking: self == Self::Plasma,
            expanding_radius: (self == Self::Plasma)
                .then_some(ExpandingRadius { min: 0., max: 70. }),
            decaying_damage: (self == Self::Plasma).then_some(DecayingDamage { initial: 0.3 }),
            sound: Some(sound.into()),
            attached: matches!(self, Self::Flame | Self::Frost),
            persistent: self == Self::Frost,
            hit_rate_limit: if self == Self::Frost { 500. } else { 0. },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStats {
    pub speed_max_scaler: f64,
    pub acceleration: f64,
    pub rotation_speed: f64,
    pub primary_energy_cost: f64,
    pub primary_weapon_delay: f64,
    pub primary_weapon_fill_rate: f64,
    pub primary_bullet_config: BulletConfig,
}
impl CharacterStats {
    pub fn for_ship(ship: ShipId, gichi: bool, shield: bool) -> Self {
        let (speed, acceleration, rot, cost, bullet) = match ship {
            ShipId::Gondi => (1.2, 0.004, 0.5, 0.12, BulletKind::Flame),
            ShipId::Sholev => (
                if shield { 1.6 } else { 0.8 },
                0.004,
                if shield { 1. } else { 0.5 },
                0.07,
                BulletKind::Ice,
            ),
            ShipId::MichiGichi => {
                if gichi {
                    (1., 0.002, 0.5, 0.08, BulletKind::Sun)
                } else {
                    (0.2, 0.002, 0.3, 0.24, BulletKind::Plasma)
                }
            }
        };
        Self {
            speed_max_scaler: speed,
            acceleration,
            rotation_speed: rot,
            primary_energy_cost: cost,
            primary_weapon_delay: 100.,
            primary_weapon_fill_rate: 0.0002,
            primary_bullet_config: bullet.config(),
        }
    }
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct InputState {
    pub thrust: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub fire_primary: bool,
    pub fire_secondary: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(flatten)]
    pub entity: Entity,
    pub ship_id: ShipId,
    pub stats: CharacterStats,
    pub roll_angle: f64,
    pub energy: f64,
    pub primary_cooldown: f64,
    pub secondary_cooldown: f64,
    pub snot_count: i32,
    pub need_roll_reset: bool,
    pub morph_state: u8,
    pub current_anim_frame: u32,
    pub morph_frame_timer: f64,
    pub frost_shield_active: bool,
    pub active_frost_bullet: Option<u32>,
    pub frost_shield_active_time: f64,
    pub previous_secondary: bool,
}
entity_deref!(Player, entity, Entity);
impl Player {
    pub fn new(id: u32, ship_id: ShipId, position: Vec3) -> Self {
        let mut entity = Entity::new(
            id,
            EntityType::Player,
            if ship_id == ShipId::Sholev { 10. } else { 14. },
        );
        entity.auto_wrap = false;
        entity.position = position;
        entity.previous_position = position;
        if ship_id == ShipId::MichiGichi {
            entity.collision_spheres = vec![
                CollisionSphere {
                    offset: Vec3::new(0., -10., 0.),
                    radius: 14.,
                },
                CollisionSphere {
                    offset: Vec3::new(0., 10., 0.),
                    radius: 14.,
                },
            ];
        }
        Self {
            entity,
            ship_id,
            stats: CharacterStats::for_ship(ship_id, false, false),
            roll_angle: 0.,
            energy: 1.,
            primary_cooldown: 0.,
            secondary_cooldown: 0.,
            snot_count: 0,
            need_roll_reset: false,
            morph_state: 0,
            current_anim_frame: 1,
            morph_frame_timer: 0.,
            frost_shield_active: false,
            active_frost_bullet: None,
            frost_shield_active_time: 0.,
            previous_secondary: false,
        }
    }
    pub fn get_effective_max_speed(&self) -> f64 {
        self.stats.speed_max_scaler / f64::from(self.snot_count + 1)
    }
    pub fn get_effective_rot_speed(&self) -> f64 {
        self.stats.rotation_speed / ((f64::from(self.snot_count) / 3.).floor() + 1.)
    }
    pub fn rotate(&mut self, direction: f64, dt: f64) {
        let delta = self.get_effective_rot_speed() * (dt * TIME_SCALER);
        self.angle += direction * delta;
        self.roll_angle = (self.roll_angle + direction * 0.6 * delta).clamp(-70., 70.);
        self.need_roll_reset = true;
        self.correct_angle();
    }
    pub fn reset_roll(&mut self, dt: f64) {
        if !self.need_roll_reset {
            self.roll_angle = 0.;
            return;
        }
        let d = 0.6 * self.get_effective_rot_speed() * (dt * TIME_SCALER);
        if self.roll_angle < 0. {
            self.roll_angle += d;
            if self.roll_angle >= 0. {
                self.roll_angle = 0.;
                self.need_roll_reset = false;
            }
        } else {
            self.roll_angle -= d;
            if self.roll_angle <= 0. {
                self.roll_angle = 0.;
                self.need_roll_reset = false;
            }
        }
    }
    pub fn thrust(&mut self, dt: f64) {
        let a = self.stats.acceleration * (dt * TIME_SCALER);
        let v = Vec3::heading(self.angle, self.get_effective_max_speed());
        self.accel_vec.x = move_toward(self.accel_vec.x, v.x, a);
        self.accel_vec.y = move_toward(self.accel_vec.y, v.y, a);
    }
    pub fn update_energy(&mut self, dt: f64) {
        if self.primary_cooldown > 0. {
            self.primary_cooldown = (self.primary_cooldown - dt).max(0.);
        }
        if self.secondary_cooldown > 0. {
            self.secondary_cooldown = (self.secondary_cooldown - dt).max(0.);
        }
        self.energy = (self.energy + self.stats.primary_weapon_fill_rate * dt).min(1.);
    }
    pub fn trigger_morph(&mut self) {
        if self.morph_state == 0 {
            self.morph_state = 1;
            self.morph_frame_timer = 0.;
        } else if self.morph_state == 2 {
            self.morph_state = 3;
            self.morph_frame_timer = 0.;
        }
    }
    pub fn update_morph(&mut self, dt: f64) {
        if self.morph_state != 1 && self.morph_state != 3 {
            return;
        }
        self.morph_frame_timer += dt;
        while self.morph_frame_timer >= 20. {
            self.morph_frame_timer -= 20.;
            if self.morph_state == 1 {
                self.current_anim_frame += 1;
                if self.current_anim_frame >= 41 {
                    self.current_anim_frame = 41;
                    self.morph_state = 2;
                    self.stats = CharacterStats::for_ship(self.ship_id, true, false);
                    break;
                }
            } else {
                self.current_anim_frame -= 1;
                if self.current_anim_frame <= 1 {
                    self.current_anim_frame = 1;
                    self.morph_state = 0;
                    self.stats = CharacterStats::for_ship(self.ship_id, false, false);
                    break;
                }
            }
        }
    }
    pub fn deactivate_shield(&mut self) {
        self.frost_shield_active = false;
        self.active_frost_bullet = None;
        self.frost_shield_active_time = 0.;
        self.stats = CharacterStats::for_ship(self.ship_id, false, false);
    }
    pub fn bullet_kind(&self) -> BulletKind {
        match self.ship_id {
            ShipId::Gondi => BulletKind::Flame,
            ShipId::Sholev => BulletKind::Ice,
            ShipId::MichiGichi => {
                if matches!(self.morph_state, 2 | 3) {
                    BulletKind::Sun
                } else {
                    BulletKind::Plasma
                }
            }
        }
    }
}
fn move_toward(current: f64, target: f64, max_delta: f64) -> f64 {
    let diff = target - current;
    let step = diff.abs().min(max_delta);
    if diff > 0. {
        current + step
    } else if diff < 0. {
        current - step
    } else {
        current
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    #[serde(flatten)]
    pub entity: Entity,
    pub kind: BulletKind,
    pub config: BulletConfig,
    pub origin_entity: Option<u32>,
    pub ttl: f64,
    pub tracking_target: Option<u32>,
    pub anim_frame: f64,
    pub anim_speed: f64,
    pub local_offset_distance: f64,
    pub attached_speed: f64,
    pub damage_cooldown: f64,
}
entity_deref!(Bullet, entity, Entity);
impl Bullet {
    pub fn new(id: u32, kind: BulletKind, origin_entity: Option<u32>) -> Self {
        let config = kind.config();
        Self {
            entity: Entity::new(id, EntityType::Bullet, config.collision_radius),
            kind,
            ttl: config.max_ttl,
            config,
            origin_entity,
            tracking_target: None,
            anim_frame: 0.,
            anim_speed: if kind == BulletKind::Plasma {
                0.4
            } else if kind == BulletKind::Ice {
                0.2
            } else {
                0.
            },
            local_offset_distance: 0.,
            attached_speed: 0.,
            damage_cooldown: 0.,
        }
    }
    pub fn current_damage(&self) -> f64 {
        self.config
            .decaying_damage
            .as_ref()
            .map_or(self.config.damage, |d| {
                (self.ttl.max(0.) / self.config.max_ttl) * d.initial
            })
    }
    pub fn update(&mut self, dt: f64, origin: Option<&Entity>, target: Option<&Entity>) {
        if self.damage_cooldown > 0. {
            self.damage_cooldown = (self.damage_cooldown - dt).max(0.);
        }
        if self.config.attached {
            if let Some(origin) = origin.filter(|o| o.alive) {
                self.local_offset_distance += self.attached_speed * dt * TIME_SCALER;
                let v = Vec3::heading(origin.angle, self.local_offset_distance);
                self.position = origin.position.plus(v);
                self.previous_position = self.position;
                self.ttl -= dt;
                if self.ttl <= 0. {
                    self.alive = false;
                }
                return;
            }
        }
        if self.config.tracking {
            if let Some(target) = target.filter(|t| t.alive) {
                let speed = (self.accel_vec.x * self.accel_vec.x
                    + self.accel_vec.y * self.accel_vec.y)
                    .sqrt();
                if speed > 0.001 {
                    let dx = target.position.x - self.position.x;
                    let dy = target.position.y - self.position.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > 0.001 {
                        let tx = dx / dist;
                        let ty = dy / dist;
                        let cx = self.accel_vec.x / speed;
                        let cy = self.accel_vec.y / speed;
                        let nx = cx + (tx - cx) * 0.02;
                        let ny = cy + (ty - cy) * 0.02;
                        let nl = (nx * nx + ny * ny).sqrt();
                        self.accel_vec.x = (nx / nl) * speed;
                        self.accel_vec.y = (ny / nl) * speed;
                    }
                }
            }
        }
        self.entity.update(dt);
        self.ttl -= dt;
        if let Some(r) = &self.config.expanding_radius {
            self.entity.collision_spheres[0].radius = r.min
                + ((self.config.max_ttl - self.ttl) / self.config.max_ttl).min(1.)
                    * (r.max - r.min);
        }
        if self.ttl <= 0. {
            self.alive = false;
        }
    }
    pub fn animate(&mut self, dt: f64) {
        if self.kind == BulletKind::Plasma {
            self.anim_frame += self.anim_speed * dt;
            if self.anim_frame > 41. {
                self.anim_speed = 0.03;
            }
            if self.anim_frame > 79. {
                self.anim_frame = 41.;
            }
        } else if self.kind == BulletKind::Ice {
            self.anim_frame += self.anim_speed * dt;
            if self.anim_frame > 64. {
                self.anim_frame = 0.;
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snot {
    #[serde(flatten)]
    pub bullet: Bullet,
    pub target: u32,
    pub attached: bool,
    pub victim: Option<u32>,
    pub current_frame: f64,
    pub orbit_offset: Vec3,
}
entity_deref!(Snot, bullet, Bullet);
impl Snot {
    pub fn new(id: u32, origin: u32, target: u32) -> Self {
        let mut bullet = Bullet::new(id, BulletKind::Snot, Some(origin));
        bullet.entity_type = EntityType::Snot;
        Self {
            bullet,
            target,
            attached: false,
            victim: None,
            current_frame: 0.,
            orbit_offset: Vec3::default(),
        }
    }
    pub fn attach(&mut self, victim: u32, rng: &mut Rng) {
        self.attached = true;
        self.victim = Some(victim);
        self.ttl = self.config.max_ttl;
        let xz = rng.next_f64() * 360. * (std::f64::consts::PI / 180.);
        let xy = rng.next_f64() * 360. * (std::f64::consts::PI / 180.);
        let ox = 30. * xz.sin();
        let oy = -30.;
        let oz = -30. * xz.cos();
        self.orbit_offset = Vec3::new(
            ox * xy.cos() - oy * xy.sin(),
            ox * xy.sin() + oy * xy.cos(),
            oz,
        );
    }
    pub fn update(&mut self, dt: f64, target: &Entity, victim: Option<&Entity>) -> Option<u32> {
        self.current_frame += 0.03 * dt;
        if self.ttl > 2000. {
            if self.current_frame >= 55. {
                self.current_frame -= 55.;
            }
        } else if self.current_frame > 116. {
            self.current_frame = 116.;
        }
        if self.attached {
            if let Some(victim) = victim {
                let r = 2. * dt / 1000.;
                let c = r.cos();
                let s = r.sin();
                let ox = self.orbit_offset.x * c - self.orbit_offset.z * s;
                let oz = self.orbit_offset.x * s + self.orbit_offset.z * c;
                self.orbit_offset.x = ox;
                self.orbit_offset.z = oz;
                let ox2 = self.orbit_offset.x * c - self.orbit_offset.y * s;
                let oy2 = self.orbit_offset.x * s + self.orbit_offset.y * c;
                self.orbit_offset.x = ox2;
                self.orbit_offset.y = oy2;
                self.position = victim.position.plus(self.orbit_offset);
                self.ttl -= dt;
                if self.ttl <= 0. {
                    self.alive = false;
                    return self.victim;
                }
                return None;
            }
        }
        let dx = target.position.x - self.position.x;
        let dy = target.position.y - self.position.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len >= 0.001 {
            let nx = dx / len;
            let ny = dy / len;
            let mut a = (ny.clamp(-1., 1.).acos() * 180.) / std::f64::consts::PI;
            if nx > 0. {
                a = 360. - a;
            }
            self.angle = a;
            self.correct_angle();
            self.accel_vec = Vec3::heading(self.angle, self.config.speed_max_scaler);
        }
        self.bullet.update(dt, None, None);
        None
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asteroid {
    #[serde(flatten)]
    pub entity: Entity,
    pub asteroid_type: u8,
    pub cur_x_angle: f64,
    pub cur_y_angle: f64,
}
entity_deref!(Asteroid, entity, Entity);
impl Asteroid {
    pub fn new(id: u32, kind: u8, rng: &mut Rng) -> Self {
        let mut e = Entity::new(id, EntityType::Asteroid, [8., 18., 15.][kind as usize]);
        let x = rng.next_f64() * 360.;
        let y = rng.next_f64() * 360.;
        e.angle = rng.next_f64() * 360.;
        e.accel_vec = Vec3::heading(e.angle, 0.1);
        Self {
            entity: e,
            asteroid_type: kind,
            cur_x_angle: x,
            cur_y_angle: y,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.cur_x_angle += dt * 0.1;
        if self.cur_x_angle >= 360. {
            self.cur_x_angle -= 360.;
        }
        self.cur_y_angle += dt * 0.1;
        if self.cur_y_angle >= 360. {
            self.cur_y_angle -= 360.;
        }
        self.entity.update(dt);
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Explosion {
    #[serde(flatten)]
    pub entity: Entity,
    pub current_frame: f64,
    pub source_type: EntityType,
}
entity_deref!(Explosion, entity, Entity);
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sun {
    #[serde(flatten)]
    pub entity: Entity,
    pub current_frame: f64,
}
entity_deref!(Sun, entity, Entity);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rng {
    pub state: u32,
}
impl Rng {
    pub fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0x6d2b79f5 } else { seed },
        }
    }
    pub fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        f64::from(x) / 4294967296.
    }
}
pub fn apply_sun_gravity(entity: &mut Entity, sun: Vec3, dt: f64) {
    let dx = sun.x - entity.position.x;
    let dy = sun.y - entity.position.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if !(0.001..=450.).contains(&dist) {
        return;
    }
    let force = (0.5 / dist).min(0.001);
    let scale = force * dt * TIME_SCALER;
    entity.accel_vec.x += (dx / dist) * scale;
    entity.accel_vec.y += (dy / dist) * scale;
}
