//! Swept sphere detection and collision response, preserving source pair order.
use crate::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SweepResult {
    pub hit: bool,
    pub u0: f64,
}
pub fn sphere_sphere_sweep(
    ra: f64,
    a0: Vec3,
    a1: Vec3,
    rb: f64,
    b0: Vec3,
    b1: Vec3,
) -> SweepResult {
    let va = a1.minus(a0);
    let vb = b1.minus(b0);
    let ab = b0.minus(a0);
    let vab = vb.minus(va);
    let rab = ra + rb;
    let a = vab.dot(vab);
    let b = 2. * vab.dot(ab);
    let c = ab.dot(ab) - rab * rab;
    let no = SweepResult { hit: false, u0: 0. };
    if c <= 0. {
        return SweepResult { hit: true, u0: 0. };
    }
    if a == 0. {
        return no;
    }
    let d = b * b - 4. * a * c;
    if d < 0. {
        return no;
    }
    let sq = d.sqrt();
    let mut u0 = (-b - sq) / (2. * a);
    let mut u1 = (-b + sq) / (2. * a);
    if u0 > u1 {
        std::mem::swap(&mut u0, &mut u1);
    }
    if u0 > 1. || u1 < 0. {
        return no;
    }
    if u0 < 0. {
        return if (0. ..=1.).contains(&u1) {
            SweepResult { hit: true, u0: 0. }
        } else {
            no
        };
    }
    SweepResult { hit: true, u0 }
}
pub fn test_entity_pair(a: &Entity, b: &Entity) -> SweepResult {
    for sa in &a.collision_spheres {
        for sb in &b.collision_spheres {
            let r = sphere_sphere_sweep(
                sa.radius,
                a.previous_position.plus(sa.offset),
                a.position.plus(sa.offset),
                sb.radius,
                b.previous_position.plus(sb.offset),
                b.position.plus(sb.offset),
            );
            if r.hit {
                return r;
            }
        }
    }
    SweepResult { hit: false, u0: 0. }
}
pub fn enforce_minimum_distance(a: &mut Entity, b: &mut Entity) {
    let ra = a.radius();
    let rb = b.radius();
    if ra == 0. && rb == 0. {
        return;
    }
    let min = ra + rb;
    let buffer = min + 8.;
    let dx = b.position.x - a.position.x;
    let dy = b.position.y - a.position.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist >= buffer {
        return;
    }
    let (nx, ny) = if dist < 0.001 {
        (1., 0.)
    } else {
        (dx / dist, dy / dist)
    };
    if dist < min {
        let push = (buffer - dist) / 2.;
        a.position.x -= nx * push;
        a.position.y -= ny * push;
        b.position.x += nx * push;
        b.position.y += ny * push;
    }
    let ad = a.accel_vec.x * nx + a.accel_vec.y * ny;
    if ad > 0. {
        a.accel_vec.x -= ad * nx;
        a.accel_vec.y -= ad * ny;
    }
    let bd = b.accel_vec.x * nx + b.accel_vec.y * ny;
    if bd < 0. {
        b.accel_vec.x -= bd * nx;
        b.accel_vec.y -= bd * ny;
    }
}
/// Returns entities which died during this event, in source callback order.
pub fn ship_collision(a: &mut Entity, b: &mut Entity) -> Vec<Entity> {
    if a.entity_type == EntityType::Asteroid || b.entity_type == EntityType::Asteroid {
        let dx = b.position.x - a.position.x;
        let dy = b.position.y - a.position.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let (nx, ny) = if dist > 0.001 {
            (dx / dist, dy / dist)
        } else {
            (1., 0.)
        };
        let va = (a.accel_vec.x * a.accel_vec.x + a.accel_vec.y * a.accel_vec.y).sqrt();
        let vb = (b.accel_vec.x * b.accel_vec.x + b.accel_vec.y * b.accel_vec.y).sqrt();
        a.accel_vec = Vec3::new(-nx * va, -ny * va, 0.);
        b.accel_vec = Vec3::new(nx * vb, ny * vb, 0.);
        return vec![];
    }
    let speed = a.accel_vec.minus(b.accel_vec).len();
    let damage = (speed * 0.2).min(0.3);
    a.health -= damage;
    b.health -= damage;
    let mut deaths = vec![];
    if a.health <= 0. {
        a.alive = false;
        if a.entity_type == EntityType::Player {
            deaths.push(a.clone());
        }
    }
    if b.health <= 0. {
        b.alive = false;
        if b.entity_type == EntityType::Player {
            deaths.push(b.clone());
        }
    }
    let mut nx = b.position.x - a.position.x;
    let mut ny = b.position.y - a.position.y;
    let mut dist = (nx * nx + ny * ny).sqrt();
    if dist < 0.001 {
        nx = (a.position.x - a.previous_position.x) - (b.position.x - b.previous_position.x);
        ny = (a.position.y - a.previous_position.y) - (b.position.y - b.previous_position.y);
        let len = (nx * nx + ny * ny).sqrt();
        if len > 0.001 {
            nx = -nx / len;
            ny = -ny / len;
        } else {
            nx = 1.;
            ny = 0.;
        }
        dist = 0.;
    } else {
        nx /= dist;
        ny /= dist;
    }
    let target = a.radius() + b.radius() + 8.;
    if dist < target {
        let push = (target - dist) / 2.;
        a.position.x -= nx * push;
        a.position.y -= ny * push;
        b.position.x += nx * push;
        b.position.y += ny * push;
    }
    a.previous_position = a.position;
    b.previous_position = b.position;
    let va = a.accel_vec.len().max(0.4);
    let vb = b.accel_vec.len().max(0.4);
    a.accel_vec = Vec3::new(-nx * va, -ny * va, 0.);
    b.accel_vec = Vec3::new(nx * vb, ny * vb, 0.);
    deaths
}
pub fn sun_collision(sun: &Entity, other: &mut Entity) -> Option<Entity> {
    other.health -= 0.02;
    let death = if other.health <= 0. {
        other.alive = false;
        (other.entity_type == EntityType::Player).then(|| other.clone())
    } else {
        None
    };
    let dx = other.position.x - sun.position.x;
    let dy = other.position.y - sun.position.y;
    let dist = (dx * dx + dy * dy).sqrt();
    let (nx, ny, dist) = if dist < 0.001 {
        (1., 0., 0.)
    } else {
        (dx / dist, dy / dist, dist)
    };
    let target = sun.radius() + other.radius() + 8.;
    if dist < target {
        other.position.x = sun.position.x + nx * target;
        other.position.y = sun.position.y + ny * target;
    }
    other.previous_position = other.position;
    let speed = other.accel_vec.len().max(0.4);
    other.accel_vec = Vec3::new(nx * speed, ny * speed, 0.);
    death
}
