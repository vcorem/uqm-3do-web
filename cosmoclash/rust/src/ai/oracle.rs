//! Differential-test entrypoint. Never selected by the game's public ABI.
use super::*;
use serde_json::{json, Value};

fn decode<T: serde::de::DeserializeOwned>(v: &Value) -> Result<T, String> {
    serde_json::from_value(v.clone()).map_err(|e| e.to_string())
}
pub fn run(request: &Value) -> Result<Value, String> {
    let level = request["level"].as_u64().unwrap_or(2) as u8;
    if !(1..=3).contains(&level) {
        return Err("invalid difficulty".into());
    }
    let mut state = request["seed"].as_u64().unwrap_or(1234567) as u32;
    let mut calls = 0u64;
    let mut rng = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        calls += 1;
        state as f64 / 4294967296.0
    };
    match request["kind"].as_str().unwrap_or("controller") {
        "worldBridge" => {
            let mut trace = Vec::new();
            for case in request["cases"].as_array().ok_or("cases required")? {
                let ships: [crate::ShipId; 2] = decode(&case["ships"])?;
                let mut world = crate::World::new(
                    case["seed"].as_u64().unwrap_or(1) as u32,
                    ships,
                    Some(
                        match level {
                            1 => "easy",
                            2 => "medium",
                            _ => "hard",
                        }
                        .into(),
                    ),
                );
                let n = case["variant"].as_u64().unwrap_or(0) as u32;
                for (i, p) in world.players.iter_mut().enumerate() {
                    let v = n + i as u32;
                    p.position = crate::Vec3::new(
                        700.0 + f64::from(v) * 13.0,
                        650.0 - f64::from(v) * 5.0,
                        0.0,
                    );
                    p.previous_position = p.position;
                    p.accel_vec =
                        crate::Vec3::new(f64::from(v % 5) * 0.09, f64::from(v % 7) * -0.07, 0.0);
                    p.angle = f64::from(v * 17 % 360);
                    p.health = if i == 0 { 0.9 } else { 0.8 };
                    p.energy = f64::from(v % 9) * 0.1;
                    p.snot_count = (v % 8) as i32;
                    p.morph_state = (v % 4) as u8;
                    p.frost_shield_active = p.ship_id == crate::ShipId::Sholev && v % 2 == 0;
                    p.stats = crate::CharacterStats::for_ship(
                        p.ship_id,
                        p.morph_state >= 2,
                        p.frost_shield_active,
                    );
                }
                for j in 0..16u32 {
                    let owner = match j % 4 {
                        0 => Some(0),
                        1 => Some(1),
                        2 => None,
                        _ => Some(999),
                    };
                    let mut b = crate::Bullet::new(
                        100 + j,
                        if j % 3 == 0 {
                            crate::BulletKind::Plasma
                        } else {
                            crate::BulletKind::Flame
                        },
                        owner,
                    );
                    b.position = crate::Vec3::new(710.0 + f64::from(j), 650.0 + f64::from(j), 0.0);
                    b.alive = j % 3 != 0;
                    world.bullets.push(b);
                    let mut s = crate::Snot::new(200 + j, owner.unwrap_or(998), 0);
                    s.origin_entity = owner;
                    s.position = crate::Vec3::new(720.0 + f64::from(j), 650.0 - f64::from(j), 0.0);
                    s.attached = j % 3 == 0;
                    s.alive = j % 5 != 0;
                    world.snots.push(s);
                }
                let before = serde_json::to_value(&world).map_err(|e| e.to_string())?;
                let players = [world.ai_player(0), world.ai_player(1)];
                let tables = [
                    buildPlayerTable(&players[0], &players[1]),
                    buildPlayerTable(&players[1], &players[0]),
                ];
                let mut updates = Vec::new();
                for _ in 0..12 {
                    let output = world.ai_input(1000.0 / 60.0);
                    updates.push(
                        json!({"output":output,"controller":world.ai_controller,"rng":world.rng}),
                    );
                }
                trace.push(
                    json!({"before":before,"players":players,"tables":tables,"updates":updates}),
                );
            }
            Ok(json!({"trace":trace}))
        }
        "controller" => {
            let mut ai = AiController::new(level);
            let mut trace = Vec::new();
            for frame in request["frames"].as_array().ok_or("frames required")? {
                if frame["reset"].as_bool() == Some(true) {
                    ai.reset();
                }
                if let Some(overrides) = frame["override"].as_object() {
                    let mut value = serde_json::to_value(&ai).map_err(|e| e.to_string())?;
                    for (k, v) in overrides {
                        value[k] = v.clone();
                    }
                    ai = decode(&value)?;
                }
                let own: AiPlayer = decode(&frame["ai"])?;
                let enemy: AiPlayer = decode(&frame["user"])?;
                let a = buildPlayerTable(&own, &enemy);
                let u = buildPlayerTable(&enemy, &own);
                ai.update_tables(&a, &u, &mut rng);
                trace.push(json!({"state":ai,"ai":a,"user":u}));
            }
            Ok(json!({"trace":trace,"randomState":state,"randomCalls":calls}))
        }
        "modes" => {
            let mut trace = Vec::new();
            for case in request["cases"].as_array().ok_or("cases required")? {
                let ai: PlayerTable = decode(&case["ai"])?;
                let user: PlayerTable = decode(&case["user"])?;
                let attack = case["attack"].as_bool().ok_or("attack required")?;
                let mut mode: AIMode = decode(&case["mode"])?;
                let mut ctx = AIContext::new(level);
                if let Some(overrides) = case["ctx"].as_object() {
                    let mut value = serde_json::to_value(&ctx).map_err(|e| e.to_string())?;
                    for (k, v) in overrides {
                        value[k] = v.clone();
                    }
                    ctx = decode(&value)?;
                }
                let mut out = AIOutput::default();
                mode.update(attack, &ai, &user, &mut out, &mut ctx, &mut rng);
                trace.push(json!({"mode":mode,"out":out,"ctx":ctx}));
            }
            Ok(json!({"trace":trace,"randomState":state,"randomCalls":calls}))
        }
        "learning" => {
            let mut learning = AILearning::default();
            let mut trace = Vec::new();
            for event in request["events"].as_array().ok_or("events required")? {
                let tactic = event["tactic"].as_u64().ok_or("tactic required")?;
                if tactic > 20 {
                    return Err("tactic out of range".into());
                }
                learning.updateTacticsTable(
                    event["attack"].as_bool().ok_or("attack required")?,
                    tactic as f64,
                    event["hit"].as_bool().ok_or("hit required")?,
                );
                trace.push(learning.clone());
            }
            Ok(json!({"trace":trace}))
        }
        "tactics" => {
            let mut trace = Vec::new();
            for case in request["cases"].as_array().ok_or("cases required")? {
                let ai: PlayerTable = decode(&case["ai"])?;
                let user: PlayerTable = decode(&case["user"])?;
                let mut ctx = AIContext::new(level);
                let mut out = AIOutput::default();
                if let Some(overrides) = case["ctx"].as_object() {
                    let mut value = serde_json::to_value(&ctx).map_err(|e| e.to_string())?;
                    for (k, v) in overrides {
                        value[k] = v.clone();
                    }
                    ctx = decode(&value)?;
                }
                if !case["out"].is_null() {
                    out = decode(&case["out"])?;
                }
                let name = case["name"].as_str().ok_or("name required")?;
                let p = case["parameter"].as_f64().unwrap_or(150.0);
                let result = match name {
                    "onceMethod" => {
                        onceMethod(&mut out, &mut ctx);
                        Value::Null
                    }
                    "turnToAngle" => json!(turnToAngle(&ai, p, &mut out)),
                    "turnAwayFromOp" => {
                        turnAwayFromOp(&ai, p, &mut out);
                        Value::Null
                    }
                    "playerThrustSpeed" => {
                        playerThrustSpeed(&ai, p, &mut out);
                        Value::Null
                    }
                    "fireWeapon2Once" => {
                        fireWeapon2Once(&mut out, &mut ctx);
                        Value::Null
                    }
                    "playerThrustOnce" => {
                        playerThrustOnce(&mut out, &mut ctx);
                        Value::Null
                    }
                    "fireWeapon1Energy" => {
                        fireWeapon1Energy(&ai, &mut out, &mut ctx);
                        Value::Null
                    }
                    "fireWeapon2Energy" => {
                        fireWeapon2Energy(&ai, &mut out, &mut ctx);
                        Value::Null
                    }
                    "fire" => {
                        fire(
                            &ai,
                            &user,
                            case["weapon"].as_f64().unwrap_or(1.0),
                            case["shootTime"].as_f64().unwrap_or(1000.0),
                            case["shootInterval"].as_f64().unwrap_or(25.0),
                            case["timeToRest"].as_f64().unwrap_or(10.0),
                            case["aim"].as_bool().unwrap_or(true),
                            &mut out,
                            &mut ctx,
                            &mut rng,
                        );
                        Value::Null
                    }
                    "slowDown" => json!(slowDown(&ai, p, &mut out, &mut ctx)),
                    "stayAtADistanceFromSun" => json!(stayAtADistanceFromSun(
                        &ai,
                        ctx.sunTable,
                        p,
                        &mut out,
                        &mut ctx
                    )),
                    "stayAtADistance" => json!(stayAtADistance(&ai, &user, p, &mut out)),
                    "stayAtADistanceFromEntity" => {
                        stayAtADistanceFromEntity(&ai, user.x, user.y, p, &mut out);
                        Value::Null
                    }
                    "flyToPoint" => json!(flyToPoint(&ai, user.x, user.y, &mut out)),
                    "flyToPointNStop" => {
                        json!(flyToPointNStop(&ai, user.x, user.y, &mut out, &mut ctx))
                    }
                    "flyInCircle" => {
                        flyInCircle(&ai, &user, p, &mut out, &mut ctx);
                        Value::Null
                    }
                    "numOfShotsInAir" => json!(numOfShotsInAir(&ai)),
                    "getClosestPlasma" => json!(getClosestPlasma(&ai, &user)),
                    "getClosestSnot" => json!(getClosestSnot(&ai, &user)),
                    "isActiveIceShield" => json!(false),
                    "getNumberOfSnotsStuck" => json!(getNumberOfSnotsStuck(&ai)),
                    "facingAndShooting" => json!(facingAndShooting(&ai, &user, &ctx, &mut rng)),
                    "avoidBluePlasma" => json!(avoidBluePlasma(&ai, &user, &mut out, &mut ctx)),
                    "avoidSnot" => json!(avoidSnot(&ai, &user, &mut out, &mut ctx)),
                    "avoidIceSholev" => {
                        json!(avoidIceSholev(&ai, &user, &mut out, &mut ctx, &mut rng))
                    }
                    "shootWhenSnotsActive" => json!(shootWhenSnotsActive(
                        &ai, &user, &mut out, &mut ctx, &mut rng
                    )),
                    "snotsStuckOnAIAction" => {
                        json!(snotsStuckOnAIAction(&ai, &user, &mut out, &mut ctx))
                    }
                    "superFastAndShoot" => {
                        superFastAndShoot(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "zigzag" => {
                        zigzag(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "turnAndShoot" => {
                        turnAndShoot(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "zigZagShoot" => {
                        zigZagShoot(
                            &ai,
                            &user,
                            case["stop"].as_bool().unwrap_or(false),
                            &mut out,
                            &mut ctx,
                        );
                        Value::Null
                    }
                    "uniqueTactic" => {
                        uniqueTactic(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "cowardFlying" => {
                        cowardFlying(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "lowEnergyAttack" => {
                        lowEnergyAttack(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "closeRangeAttack" => {
                        closeRangeAttack(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "inDisadvantage" => {
                        inDisadvantage(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "inAndOut" => {
                        inAndOut(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "stationaryAttack" => {
                        stationaryAttack(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "kamikazeAttack" => {
                        kamikazeAttack(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    "sitOnTailAttack" => {
                        sitOnTailAttack(&ai, &user, &mut out, &mut ctx, &mut rng);
                        Value::Null
                    }
                    _ => return Err(format!("unknown tactic {name}")),
                };
                trace.push(json!({"out":out,"ctx":ctx,"result":result}));
            }
            Ok(json!({"trace":trace,"randomState":state,"randomCalls":calls}))
        }
        "math" => {
            let mut trace = Vec::new();
            for case in request["cases"].as_array().ok_or("cases required")? {
                let a: PlayerTable = decode(&case["ai"])?;
                let b: PlayerTable = decode(&case["user"])?;
                let n = case["n"].as_f64().unwrap_or(1000.0);
                trace.push(json!({"distance":getDistance(&a,&b),"vectorSize":getVectorSize(a.x,a.y,b.x,b.y),
                    "angle":getAngleToXY(b.x,b.y,a.x,a.y),"velocityAngle":getVVAngle(a.dxVector,a.dyVector),
                    "turn":turnToAngleDecision(a.angle,b.angle),"aim":goodAim(&a,&b,getDifficultyConstants(level).goodAimProbability,&mut rng),
                    "range":realIsInRange(&a,&b),"futureRange":isInFutureRange(&a,&b,n),
                    "speed":getSpeedOfShot(a.entityType,a.morphState),"ttl":getMaxTtlOfShot(a.entityType,a.morphState),
                    "maxDistance":getMaxDistanceOfShot(a.entityType,a.morphState)}));
            }
            Ok(json!({"trace":trace,"randomState":state,"randomCalls":calls}))
        }
        other => Err(format!("unknown AI oracle kind {other}")),
    }
}
