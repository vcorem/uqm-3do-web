//! Tactics and helper routines, in the same order as upstream AITactics.ts.
use super::*;

pub fn onceMethod(out: &mut AIOutput, ctx: &mut AIContext) {
    if ctx.fireWeapon2Flag {
        out.setFireSecondary(false);
        ctx.fireWeapon2Flag = false;
        ctx.smallGitchiFlag = !ctx.smallGitchiFlag;
    }
    if ctx.thrustOnceFlag {
        out.setThrust(false);
        ctx.thrustOnceFlag = false;
    }
}
pub fn turnToAngle(table: &PlayerTable, desiredAngle: f64, out: &mut AIOutput) -> bool {
    let d = turnToAngleDecision(table.angle, desiredAngle);
    if d.reached {
        out.resetTurn();
        return true;
    }
    if d.right {
        out.setTurnLeft(false);
        out.setTurnRight(true);
    } else {
        out.setTurnRight(false);
        out.setTurnLeft(true);
    }
    false
}
pub fn turnAwayFromOp(table: &PlayerTable, toOpponentAngle: f64, out: &mut AIOutput) {
    turnToAngle(table, (toOpponentAngle + 180.0) % 360.0, out);
}
pub fn playerThrustSpeed(table: &PlayerTable, speed: f64, out: &mut AIOutput) {
    let angle = table.angleToOp - table.VVAngle;
    let vVector = (angle * std::f64::consts::PI / 180.0).cos() * table.speed;
    out.setThrust(vVector < speed || table.distance > 250.0);
}
pub fn fireWeapon2Once(out: &mut AIOutput, ctx: &mut AIContext) {
    out.setFireSecondary(true);
    ctx.fireWeapon2Flag = true;
}
pub fn playerThrustOnce(out: &mut AIOutput, ctx: &mut AIContext) {
    out.setThrust(true);
    ctx.thrustOnceFlag = true;
}
pub fn fireWeapon1Energy(table: &PlayerTable, out: &mut AIOutput, ctx: &mut AIContext) {
    if table.energy < ctx.constants.minEnergyLevel && !ctx.energyFillFlag {
        out.setFirePrimary(false);
        ctx.energyFillFlag = true;
    } else if table.energy > ctx.constants.resumeEnergyLevel || !ctx.energyFillFlag {
        out.setFirePrimary(true);
        ctx.energyFillFlag = false;
    }
}
pub fn fireWeapon2Energy(table: &PlayerTable, out: &mut AIOutput, ctx: &mut AIContext) {
    if table.energy < ctx.constants.minEnergyLevel {
        out.setFireSecondary(false);
        ctx.energyFillFlag = true;
    } else if table.energy > ctx.constants.resumeEnergyLevel || !ctx.energyFillFlag {
        out.setFireSecondary(true);
        ctx.energyFillFlag = false;
    }
}
pub fn fireStop(out: &mut AIOutput, weapon: f64) {
    if weapon == 1.0 {
        out.setFirePrimary(false);
    } else {
        out.setFireSecondary(false);
    }
}
#[allow(clippy::too_many_arguments)]
pub fn fire(
    computer: &PlayerTable,
    user: &PlayerTable,
    weapon: f64,
    shootTime: f64,
    shootInterval: f64,
    timeToRest: f64,
    aim: bool,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if aim {
        turnToAngle(
            computer,
            goodAim(computer, user, ctx.constants.goodAimProbability, rng),
            out,
        );
    }
    if ctx.countShootTime <= shootTime && ctx.readyToFire {
        if shootInterval > 0.0 && ctx.countShootTime % shootInterval == 0.0 {
            if weapon == 1.0 {
                if realIsInRange(computer, user) {
                    out.setFirePrimary(true);
                }
            } else {
                out.setFireSecondary(true);
            }
        } else {
            fireStop(out, weapon);
        }
        ctx.countShootTime += 1.0;
    } else {
        fireStop(out, weapon);
        out.resetTurnAndThrust();
        ctx.countShootTime = 0.0;
        ctx.readyToFire = false;
        if timeToRest > 0.0 && ctx.num % timeToRest == 0.0 {
            ctx.readyToFire = true;
        }
    }
}
pub fn slowDown(
    table: &PlayerTable,
    mut howSlow: f64,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    if getDistance(table, &ctx.sunTable) < 250.0 || ctx.slowDownSunFlag {
        ctx.slowDownSunFlag = true;
        stayAtADistanceFromSun(table, ctx.sunTable, 450.0, out, ctx);
        return false;
    }
    if howSlow < 0.02 {
        howSlow = 0.02;
    }
    let dx = table.dxVector;
    let dy = table.dyVector;
    let speedSize = getVectorSize(0.0, 0.0, dx, dy);
    let relativeSpeed = if table.maxSpeed > 0.0 {
        speedSize / table.maxSpeed
    } else {
        0.0
    };
    if relativeSpeed > howSlow {
        let angleOfSpeed = getAngleBetweenTwoVectors(
            Vector {
                x: dx,
                y: dy,
                size: speedSize,
            },
            Vector {
                x: 0.0,
                y: 1.0,
                size: 1.0,
            },
        );
        let reverseAngle = (angleOfSpeed + 180.0) % 360.0;
        if (table.angle - reverseAngle).abs() > 2.0 {
            out.setThrust(false);
            turnToAngle(table, reverseAngle, out);
        } else {
            out.resetTurn();
            out.setThrust(true);
        }
        return false;
    }
    out.resetTurnAndThrust();
    true
}
pub fn stayAtADistanceFromSun(
    computer: &PlayerTable,
    sun: Point,
    distance: f64,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    let currentDistance = getDistance(computer, &sun);
    if currentDistance - distance < 0.0 {
        if computer.morphState == MICHI {
            fireWeapon2Once(out, ctx);
        }
        turnAwayFromOp(computer, computer.angleToSun, out);
        out.setThrust(true);
        return true;
    }
    out.resetTurnAndThrust();
    ctx.slowDownSunFlag = false;
    false
}
pub fn stayAtADistance(
    computer: &PlayerTable,
    user: &PlayerTable,
    distance: f64,
    out: &mut AIOutput,
) -> bool {
    if getDistance(computer, user) < distance {
        turnAwayFromOp(computer, computer.angleToOp, out);
        out.setThrust(true);
        return true;
    }
    out.setThrust(false);
    false
}
pub fn stayAtADistanceFromEntity(
    computer: &PlayerTable,
    entityX: f64,
    entityY: f64,
    distance: f64,
    out: &mut AIOutput,
) {
    let currentDistance = getVectorSize(computer.x, computer.y, entityX, entityY);
    if currentDistance < distance {
        let angle = getAngleToXY(entityX, entityY, computer.x, computer.y);
        turnAwayFromOp(computer, angle, out);
        out.setThrust(true);
    } else {
        out.resetTurnAndThrust();
    }
}
pub fn flyToPoint(table: &PlayerTable, x: f64, y: f64, out: &mut AIOutput) -> bool {
    let dist = getVectorSize(table.x, table.y, x, y);
    let angleToXY = getAngleToXY(x, y, table.x, table.y);
    if dist < 100.0 {
        out.resetTurn();
        out.setThrust(false);
        return true;
    }
    turnToAngle(table, angleToXY, out);
    out.setThrust(true);
    false
}
pub fn flyToPointNStop(
    table: &PlayerTable,
    x: f64,
    y: f64,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    let dist = getVectorSize(table.x, table.y, x, y);
    let angleToXY = getAngleToXY(x, y, table.x, table.y);
    if dist < 300.0 {
        if table.speed > 0.1 {
            slowDown(table, 0.0, out, ctx);
        } else if dist > 100.0 {
            if turnToAngle(table, angleToXY, out) {
                out.setThrust(table.speed < 0.1);
            }
        } else {
            slowDown(table, 0.0, out, ctx);
            return true;
        }
    } else if turnToAngle(table, angleToXY, out) {
        out.setThrust(true);
    }
    false
}
pub fn flyInCircle(
    computer: &PlayerTable,
    user: &PlayerTable,
    radius: f64,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) {
    let x = radius * (ctx.rotAngle * std::f64::consts::PI / 180.0).sin() + user.x;
    let y = radius * (ctx.rotAngle * std::f64::consts::PI / 180.0).cos() + user.y;
    if flyToPoint(computer, x, y, out) {
        ctx.rotAngle = (ctx.rotAngle + 46.0) % 360.0;
    }
}
pub fn numOfShotsInAir(table: &PlayerTable) -> usize {
    table.shots.len()
}
pub fn getClosestPlasma(computer: &PlayerTable, user: &PlayerTable) -> Option<Point> {
    if user.entityType != MITCHIGITCHI {
        return None;
    }
    let mut closest = None;
    let mut minDist = 1000.0;
    for bullet in &user.shots {
        if bullet.config.tracking {
            let dist = getDistance(computer, &bullet.position);
            if dist < minDist {
                minDist = dist;
                closest = Some(bullet.position);
            }
        }
    }
    closest
}
pub fn getClosestSnot(computer: &PlayerTable, user: &PlayerTable) -> Option<Point> {
    if user.entityType != GONDI {
        return None;
    }
    let mut closest = None;
    let mut minDist = 1000.0;
    for snot in &user.snots {
        if !snot.attached {
            let dist = getDistance(computer, &snot.position);
            if dist < minDist {
                minDist = dist;
                closest = Some(snot.position);
            }
        }
    }
    closest
}
/// The upstream AI deliberately still returns false here, even though its game
/// now implements Sholev. Preserve that behavior rather than invent new tactics.
pub fn isActiveIceShield(_table: &PlayerTable) -> Option<f64> {
    None
}
pub fn getNumberOfSnotsStuck(table: &PlayerTable) -> usize {
    if table.entityType != GONDI {
        return 0;
    }
    table.snots.iter().filter(|s| s.attached).count()
}
pub fn facingAndShooting(
    computer: &PlayerTable,
    user: &PlayerTable,
    ctx: &AIContext,
    rng: &mut dyn FnMut() -> f64,
) -> bool {
    let toAngle = goodAim(user, computer, ctx.constants.goodAimProbability, rng);
    (user.angle - toAngle).abs() < 2.0 && numOfShotsInAir(user) > 0 && realIsInRange(user, computer)
}
pub fn avoidBluePlasma(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    let distance = ctx.constants.distanceToAvoidPlasma;
    if user.entityType == MITCHIGITCHI {
        if let Some(plasma) = getClosestPlasma(computer, user) {
            if getDistance(computer, &plasma) < distance {
                if computer.entityType == MITCHIGITCHI && computer.morphState == MICHI {
                    fireWeapon2Once(out, ctx);
                }
                stayAtADistanceFromEntity(computer, plasma.x, plasma.y, distance, out);
                return true;
            }
        }
    }
    false
}
pub fn avoidSnot(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    let distance = ctx.constants.distanceToAvoidSnot;
    if user.entityType == GONDI {
        if let Some(snot) = getClosestSnot(computer, user) {
            if getDistance(computer, &snot) < distance {
                stayAtADistanceFromEntity(computer, snot.x, snot.y, distance, out);
                return true;
            }
        }
    }
    false
}
pub fn avoidIceSholev(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) -> bool {
    let distance = ctx.constants.distanceToAvoidSholevIce;
    if user.entityType == SHOLEV && isActiveIceShield(user).is_some() && user.distance < distance {
        if computer.entityType == MITCHIGITCHI {
            if computer.morphState == MICHI {
                out.setFirePrimary(true);
                fireWeapon2Once(out, ctx);
            } else if computer.morphState == GICHI {
                inDisadvantage(computer, user, out, ctx, rng);
            }
        } else if computer.entityType == GONDI || computer.entityType == SHOLEV {
            out.setFireSecondary(true);
            inDisadvantage(computer, user, out, ctx, rng);
        }
        return true;
    }
    false
}
pub fn shootWhenSnotsActive(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) -> bool {
    if computer.entityType == GONDI {
        if user.maxSpeed < ctx.constants.BELOW_MAX_SPEED_TO_SHOOT {
            out.setFireSecondary(false);
            if turnToAngle(
                computer,
                goodAim(computer, user, ctx.constants.goodAimProbability, rng),
                out,
            ) {
                if realIsInRange(computer, user) {
                    out.setFirePrimary(true);
                    out.setThrust(false);
                } else {
                    out.setFirePrimary(false);
                    playerThrustSpeed(computer, 0.1, out);
                }
            }
            return true;
        } else if getNumberOfSnotsStuck(computer) >= 2 && (ctx.level == 2 || ctx.level == 3) {
            flyInCircle(computer, user, 200.0, out, ctx);
            out.setFirePrimary(false);
            out.setFireSecondary(true);
            return true;
        } else {
            out.resetShots();
            out.resetTurn();
        }
    }
    false
}
pub fn snotsStuckOnAIAction(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) -> bool {
    if getNumberOfSnotsStuck(user) > 0
        && computer.maxSpeed <= ctx.constants.BELOW_MAX_SPEED_TO_SHOOT
    {
        if computer.entityType == MITCHIGITCHI {
            out.resetShots();
            if computer.morphState == GICHI {
                fireWeapon2Once(out, ctx);
            }
            // Upstream intentionally passes user twice here and for Gondi.
            out.setFirePrimary(realIsInRange(user, user));
        } else if computer.entityType == GONDI {
            out.resetShots();
            if realIsInRange(user, user) {
                if (computer.angle - computer.angleToOp).abs() < 10.0 {
                    out.setFirePrimary(true);
                } else {
                    out.setFirePrimary(false);
                    out.setFireSecondary(true);
                }
            } else {
                out.resetShots();
            }
        } else if computer.entityType == SHOLEV {
            out.setFireSecondary(false);
            if realIsInRange(user, computer) {
                out.setThrust(true);
                out.setFireSecondary(true);
            } else {
                out.setFireSecondary(false);
                out.setThrust(false);
            }
        } else {
            out.resetShots();
            if realIsInRange(user, computer) {
                if (computer.angle - computer.angleToOp).abs() < 10.0 {
                    out.setFirePrimary(true);
                } else {
                    out.setFirePrimary(false);
                    out.setFireSecondary(true);
                }
            } else {
                out.resetShots();
            }
        }
        return true;
    }
    false
}
pub fn superFastAndShoot(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if computer.entityType == MITCHIGITCHI {
        if computer.speed < computer.maxSpeed {
            if computer.morphState == MICHI {
                fireWeapon2Once(out, ctx);
            } else {
                out.resetTurn();
                out.setFirePrimary(false);
                out.setThrust(true);
            }
        } else {
            out.setThrust(false);
            let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, toAngle, out);
            if realIsInRange(computer, user) {
                fireWeapon1Energy(computer, out, ctx);
            } else {
                out.setFirePrimary(false);
            }
            if ctx.num % 250.0 == 1.0 {
                fireWeapon2Once(out, ctx);
            }
        }
    } else if computer.entityType == GONDI {
        if computer.speed < computer.maxSpeed {
            out.resetTurn();
            out.setThrust(true);
            out.setFireSecondary(false);
        } else {
            out.setThrust(false);
            let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, toAngle, out);
            fireWeapon2Energy(computer, out, ctx);
            out.setFirePrimary(realIsInRange(computer, user));
        }
    } else if computer.entityType == SHOLEV {
        if computer.speed < computer.maxSpeed {
            out.resetTurn();
            out.setFirePrimary(false);
            out.setThrust(true);
            fireWeapon2Once(out, ctx);
        } else {
            out.setThrust(false);
            let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, toAngle, out);
            if realIsInRange(computer, user) {
                fireWeapon1Energy(computer, out, ctx);
            } else {
                out.setFirePrimary(false);
            }
        }
    } else if computer.speed < computer.maxSpeed {
        out.resetTurn();
        out.setThrust(true);
        out.setFirePrimary(false);
    } else {
        out.setThrust(false);
        let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
        turnToAngle(computer, toAngle, out);
        if realIsInRange(computer, user) {
            fireWeapon1Energy(computer, out, ctx);
        } else {
            out.setFirePrimary(false);
        }
    }
}
pub fn zigzag(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    let anglesToTurn = 45.0;
    let rateOfChange = 150.0;
    let userVelocityVector = user.VVAngle;
    out.setThrust(true);
    if computer.entityType == MITCHIGITCHI {
        if computer.morphState == MICHI {
            fireWeapon2Once(out, ctx);
        }
    } else if computer.entityType == GONDI {
        if shootWhenSnotsActive(computer, user, out, ctx, rng) {
            return;
        }
        if numOfShotsInAir(computer) < 15 {
            fireWeapon2Energy(computer, out, ctx);
        } else {
            out.setFireSecondary(false);
        }
    }
    if ctx.num % (rateOfChange * 2.0) < rateOfChange {
        turnToAngle(computer, userVelocityVector + anglesToTurn, out);
    } else {
        turnToAngle(computer, userVelocityVector - anglesToTurn, out);
    }
}
pub fn turnAndShoot(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if ctx.num % 500.0 == 1.0 {
        ctx.randomAngle = (rng() * 360.0).floor();
    }
    let numberOfShots = numOfShotsInAir(computer);
    if computer.entityType == MITCHIGITCHI {
        let MGstate = computer.morphState;
        if computer.distance > 350.0 && numberOfShots < 10 && computer.energy > 0.4 {
            if numberOfShots < 5 {
                if MGstate == GICHI {
                    fireWeapon2Once(out, ctx);
                } else if MGstate == MICHI {
                    out.setThrust(false);
                    let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
                    turnToAngle(computer, toAngle, out);
                    out.setFirePrimary(realIsInRange(computer, user));
                }
            } else if MGstate == MICHI {
                fireWeapon2Once(out, ctx);
            } else if MGstate == GICHI {
                out.setThrust(false);
                let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
                turnToAngle(computer, toAngle, out);
                if (computer.angle - toAngle).abs() < 5.0
                    && computer.morphState != GICHI_TO_MICHI
                    && computer.morphState != MICHI_TO_GICHI
                {
                    out.setFirePrimary(realIsInRange(computer, user));
                }
            }
        } else {
            out.resetShots();
            if MGstate == MICHI {
                fireWeapon2Once(out, ctx);
            }
            turnToAngle(computer, ctx.randomAngle, out);
            out.setThrust(true);
        }
    } else if computer.entityType == GONDI {
        fireWeapon2Energy(computer, out, ctx);
        out.setFirePrimary(realIsInRange(computer, user));
        if ctx.num % 250.0 < 125.0 {
            out.setThrust(false);
            let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, toAngle, out);
        } else {
            turnToAngle(computer, ctx.randomAngle, out);
            out.setThrust(true);
        }
    } else if computer.distance > 350.0 && numberOfShots < 10 && computer.energy > 0.4 {
        out.setThrust(false);
        let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
        turnToAngle(computer, toAngle, out);
        if (computer.angle - toAngle).abs() < 5.0 {
            out.setFirePrimary(realIsInRange(computer, user));
        }
    } else {
        out.resetShots();
        turnToAngle(computer, ctx.randomAngle, out);
        out.setThrust(true);
    }
}
fn doZigZag(
    computer: &PlayerTable,
    toAngle: f64,
    anglesToTurn: f64,
    rateOfChange: f64,
    out: &mut AIOutput,
    ctx: &AIContext,
) {
    if ctx.num % 3.0 == 1.0 {
        if ctx.num % (rateOfChange * 2.0) < rateOfChange {
            turnToAngle(computer, toAngle + anglesToTurn, out);
        } else {
            turnToAngle(computer, toAngle - anglesToTurn, out);
        }
    } else {
        out.resetTurn();
    }
}
pub fn zigZagShoot(
    computer: &PlayerTable,
    user: &PlayerTable,
    stop: bool,
    out: &mut AIOutput,
    ctx: &mut AIContext,
) {
    let mut anglesToTurn = 15.0;
    let mut rateOfChange = 100.0;
    let toAngle = computer.angleToOp;
    if computer.entityType == MITCHIGITCHI {
        if computer.morphState == MICHI {
            fireWeapon2Once(out, ctx);
        }
        if !stop || slowDown(computer, 0.2, out, ctx) {
            doZigZag(computer, toAngle, anglesToTurn, rateOfChange, out, ctx);
        }
    } else if computer.entityType == GONDI {
        rateOfChange = 50.0;
        anglesToTurn = 30.0;
        if (!stop || slowDown(computer, 0.0, out, ctx)) && realIsInRange(computer, user) {
            doZigZag(computer, toAngle, anglesToTurn, rateOfChange, out, ctx);
        }
    } else if !stop || slowDown(computer, 0.0, out, ctx) {
        doZigZag(computer, toAngle, anglesToTurn, rateOfChange, out, ctx);
    }
    if realIsInRange(computer, user) {
        fireWeapon1Energy(computer, out, ctx);
    } else {
        out.setFirePrimary(false);
    }
}
pub fn uniqueTactic(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if computer.entityType == MITCHIGITCHI {
        if computer.morphState == GICHI {
            out.setFireSecondary(true);
        }
        turnToAngle(computer, computer.angleToOp, out);
        out.setFirePrimary(true);
        out.setThrust(true);
    } else if computer.entityType == GONDI {
        if user.maxSpeed < ctx.constants.BELOW_MAX_SPEED_TO_SHOOT {
            let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, toAngle, out);
            if realIsInRange(computer, user) {
                out.setFirePrimary(true);
                out.setThrust(false);
            } else {
                out.setThrust(true);
            }
        } else if computer.distance < ctx.constants.holdingDistanceBeforeFly {
            out.setFirePrimary(false);
            turnAwayFromOp(computer, computer.angleToOp, out);
            out.setThrust(true);
            if numOfShotsInAir(user) != 0 {
                zigzag(computer, user, out, ctx, rng);
                fireWeapon2Energy(computer, out, ctx);
            }
        } else {
            out.setFirePrimary(false);
            out.setFireSecondary(false);
            slowDown(computer, 0.2, out, ctx);
        }
    } else if computer.entityType == SHOLEV {
        turnToAngle(computer, computer.angleToOp, out);
        out.setThrust(true);
        if computer.distance < 500.0 {
            out.setFireSecondary(true);
            if computer.distance < ctx.constants.distanceSholevToFreezeAim {
                out.resetTurn();
            }
        } else {
            out.setFireSecondary(false);
        }
    }
}
pub fn cowardFlying(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if computer.entityType == GONDI {
        fireWeapon2Energy(computer, out, ctx);
    }
    let AIRange = computer.weapon1Distance;
    let mut userRange = user.weapon1Distance;
    if user.morphState == MICHI {
        userRange = AIRange;
    }
    if computer.entityType == MITCHIGITCHI && computer.morphState == MICHI {
        fireWeapon2Once(out, ctx);
    }
    if computer.energy < 0.1 {
        ctx.cowardReadyFlag = false;
    }
    if computer.distance > userRange || ctx.cowardReadyFlag {
        if computer.energy > 0.75 {
            ctx.cowardReadyFlag = true;
        }
        if slowDown(computer, 0.0, out, ctx) || realIsInRange(computer, user) {
            let angle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
            turnToAngle(computer, angle, out);
            out.setFirePrimary(computer.distance <= AIRange);
        }
    } else {
        stayAtADistanceFromEntity(computer, user.x, user.y, userRange, out);
        out.setFirePrimary(false);
    }
}
pub fn lowEnergyAttack(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    let AIRange = computer.weapon1Distance;
    let mut userRange = user.weapon1Distance;
    if user.morphState == MICHI {
        userRange = AIRange;
    }
    if computer.entityType == MITCHIGITCHI && computer.morphState == MICHI {
        fireWeapon2Once(out, ctx);
    }
    if (user.energy < 0.3 && computer.energy > 0.3) || ctx.inLowEnergyAttack {
        out.setFireSecondary(false);
        ctx.inLowEnergyAttack = true;
        if computer.energy < 0.05 {
            ctx.inLowEnergyAttack = false;
        }
        let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
        turnToAngle(computer, toAngle, out);
        out.setThrust(true);
        if realIsInRange(computer, user) {
            out.setFirePrimary(true);
            out.setThrust(false);
        }
    } else {
        ctx.inLowEnergyAttack = false;
        out.setFirePrimary(false);
        stayAtADistanceFromEntity(computer, user.x, user.y, userRange, out);
    }
    if computer.entityType == GONDI {
        fireWeapon2Energy(computer, out, ctx);
    }
}
pub fn closeRangeAttack(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if ctx.num - ctx.lastFrameCloseRange > 1.0 {
        ctx.closeRangeCounter = 0.0;
    } else {
        ctx.closeRangeCounter += 1.0;
    }
    ctx.lastFrameCloseRange = ctx.num;
    if ctx.closeRangeCounter > 200.0 {
        out.setThrust(false);
        out.setFireSecondary(false);
        let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
        turnToAngle(computer, toAngle, out);
        out.setFirePrimary(true);
    } else {
        turnToAngle(computer, (user.VVAngle + 135.0) % 360.0, out);
        out.setThrust(true);
        if computer.entityType == GONDI {
            out.setFireSecondary(true);
        }
    }
}
pub fn inDisadvantage(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    _rng: &mut dyn FnMut() -> f64,
) {
    ctx.disAdvantageCounter += 1.0;
    if computer.morphState == MICHI {
        fireWeapon2Once(out, ctx);
    }
    out.setFirePrimary((computer.angle - computer.angleToOp).abs() < 15.0);
    out.setThrust(true);
    if ctx.num % 500.0 < 250.0 {
        turnToAngle(computer, user.VVAngle + 90.0, out);
    } else {
        turnToAngle(computer, user.VVAngle - 90.0, out);
    }
    if ctx.disAdvantageCounter % 50.0 == 1.0 {
        ctx.inDisAdvantage = false;
    }
}
pub fn inAndOut(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
    if computer.distance < ctx.inAndOutdistance {
        stayAtADistance(computer, user, ctx.inAndOutdistance, out);
        if computer.entityType == GONDI {
            fireWeapon2Energy(computer, out, ctx);
        }
    } else {
        out.setFireSecondary(false);
        out.setThrust(true);
        if turnToAngle(computer, toAngle, out) {
            zigZagShoot(computer, user, false, out, ctx);
        }
    }
}
pub fn stationaryAttack(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    if slowDown(computer, 0.0, out, ctx) {
        turnToAngle(computer, computer.angleToOp, out);
        if computer.entityType == GONDI {
            if realIsInRange(computer, user) {
                out.setFirePrimary(true);
            } else {
                out.setFirePrimary(true);
                fireWeapon2Energy(computer, out, ctx);
            }
        } else {
            fire(
                computer, user, 1.0, 1000.0, 150.0, 10.0, true, out, ctx, rng,
            );
        }
    }
}
pub fn kamikazeAttack(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    rng: &mut dyn FnMut() -> f64,
) {
    let toAngle = goodAim(computer, user, ctx.constants.goodAimProbability, rng);
    if computer.morphState == MICHI {
        fireWeapon2Once(out, ctx);
    }
    turnToAngle(computer, toAngle, out);
    if computer.distance > ctx.constants.KAMIKAZE_CLOSE_DIS {
        out.setThrust(true);
    } else {
        out.resetTurnAndThrust();
    }
    if computer.entityType == SHOLEV {
        fireStop(out, 1.0);
        out.setFireSecondary(true);
    } else if computer.entityType == GONDI {
        if computer.distance < computer.weapon1Distance + 200.0 {
            fire(computer, user, 1.0, 500.0, 25.0, 10.0, true, out, ctx, rng);
        } else {
            fireWeapon2Energy(computer, out, ctx);
            out.setFirePrimary(false);
        }
    } else {
        out.setFirePrimary(realIsInRange(computer, user));
    }
}
pub fn sitOnTailAttack(
    computer: &PlayerTable,
    user: &PlayerTable,
    out: &mut AIOutput,
    ctx: &mut AIContext,
    _rng: &mut dyn FnMut() -> f64,
) {
    let angle = computer.angleToOp;
    let dist = computer.distance;
    if computer.morphState == MICHI {
        fireWeapon2Once(out, ctx);
    }
    out.setThrust(dist > user.weapon1Distance - 50.0);
    if dist < ctx.constants.distanceToStayAway && (user.angle - user.angleToOp).abs() < 20.0 {
        stayAtADistanceFromEntity(
            computer,
            user.x,
            user.y,
            ctx.constants.distanceToStayAway,
            out,
        );
    } else {
        turnToAngle(computer, angle, out);
    }
    out.setFirePrimary(realIsInRange(computer, user));
}
