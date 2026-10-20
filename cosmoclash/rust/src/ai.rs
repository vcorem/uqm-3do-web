//! Faithful port of upstream `src/ai`, retaining its frame timers and learning.
//! Randomness is injected from the simulation RNG, including draws whose result
//! does not affect a later branch. All geometry and counters use JS-equivalent f64.
#![allow(non_snake_case, non_upper_case_globals)]

use serde::{Deserialize, Serialize};
pub mod oracle;
pub mod tactics;
use tactics::*;

pub const MITCHIGITCHI: f64 = 1.0;
pub const GONDI: f64 = 4.0;
pub const SHOLEV: f64 = 8.0;
pub const MICHI: Option<u8> = Some(0);
pub const MICHI_TO_GICHI: Option<u8> = Some(1);
pub const GICHI: Option<u8> = Some(2);
pub const GICHI_TO_MICHI: Option<u8> = Some(3);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
pub trait PointLike {
    fn point(&self) -> Point;
}
impl PointLike for Point {
    fn point(&self) -> Point {
        *self
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AiShotConfig {
    pub tracking: bool,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AiShot {
    pub position: Point,
    pub config: AiShotConfig,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AiSnot {
    pub position: Point,
    pub attached: bool,
}

/// Simulation-facing bridge snapshot. Shots and snots are already filtered by
/// active state and owner, in the original entity-pool iteration order.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AiPlayer {
    pub position: Point,
    pub angle: f64,
    pub accelVec: Point,
    pub health: f64,
    pub energy: f64,
    pub maxSpeed: f64,
    pub snotCount: f64,
    pub entityType: f64,
    pub morphState: Option<u8>,
    pub shots: Vec<AiShot>,
    pub snots: Vec<AiSnot>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerTable {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub speed: f64,
    pub maxSpeed: f64,
    pub dxVector: f64,
    pub dyVector: f64,
    pub life: f64,
    pub energy: f64,
    pub entityType: f64,
    pub distance: f64,
    pub angleToOp: f64,
    pub angleToSun: f64,
    pub weapon1Distance: f64,
    pub shots: Vec<AiShot>,
    pub snots: Vec<AiSnot>,
    pub snotCount: f64,
    pub VVAngle: f64,
    pub morphState: Option<u8>,
}
impl PointLike for PlayerTable {
    fn point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

pub fn getAIEntityType(id: &str, isMichiGichi: bool) -> f64 {
    if isMichiGichi {
        MITCHIGITCHI
    } else if id == "gondi" {
        GONDI
    } else if id == "sholev" {
        SHOLEV
    } else {
        0.0
    }
}
pub fn getSpeedOfShot(entityType: f64, morphState: Option<u8>) -> f64 {
    if entityType == MITCHIGITCHI {
        if morphState == MICHI || morphState == GICHI_TO_MICHI {
            0.2
        } else {
            0.525
        }
    } else if entityType == GONDI {
        0.18
    } else if entityType == SHOLEV {
        0.6
    } else {
        0.525
    }
}
pub fn getMaxTtlOfShot(entityType: f64, morphState: Option<u8>) -> f64 {
    if entityType == MITCHIGITCHI {
        if morphState == MICHI || morphState == GICHI_TO_MICHI {
            5000.0
        } else {
            1000.0
        }
    } else if entityType == GONDI || entityType == SHOLEV {
        700.0
    } else {
        1000.0
    }
}
pub fn getMaxDistanceOfShot(entityType: f64, morphState: Option<u8>) -> f64 {
    getMaxTtlOfShot(entityType, morphState) * getSpeedOfShot(entityType, morphState)
}
pub fn buildPlayerTable(player: &AiPlayer, opponent: &AiPlayer) -> PlayerTable {
    let dxVector = player.accelVec.x * 0.35;
    let dyVector = player.accelVec.y * 0.35;
    let dx = (player.position.x - opponent.position.x).abs();
    let dy = (player.position.y - opponent.position.y).abs();
    PlayerTable {
        x: player.position.x,
        y: player.position.y,
        angle: player.angle,
        speed: (dxVector * dxVector + dyVector * dyVector).sqrt(),
        maxSpeed: player.maxSpeed,
        dxVector,
        dyVector,
        life: player.health,
        energy: player.energy,
        entityType: player.entityType,
        distance: (dx * dx + dy * dy).sqrt(),
        angleToOp: getAngleToXY(
            opponent.position.x,
            opponent.position.y,
            player.position.x,
            player.position.y,
        ),
        angleToSun: getAngleToXY(0.0, 0.0, player.position.x, player.position.y),
        weapon1Distance: getMaxDistanceOfShot(player.entityType, player.morphState),
        shots: player.shots.clone(),
        snots: player.snots.clone(),
        snotCount: player.snotCount,
        VVAngle: getVVAngle(dxVector, dyVector),
        morphState: player.morphState,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AIOutput {
    pub thrust: bool,
    pub rotateLeft: bool,
    pub rotateRight: bool,
    pub firePrimary: bool,
    pub fireSecondary: bool,
}
impl AIOutput {
    pub fn setThrust(&mut self, on: bool) {
        self.thrust = on;
    }
    pub fn setTurnLeft(&mut self, on: bool) {
        if on {
            self.rotateRight = false;
        }
        self.rotateLeft = on;
    }
    pub fn setTurnRight(&mut self, on: bool) {
        if on {
            self.rotateLeft = false;
        }
        self.rotateRight = on;
    }
    pub fn setFirePrimary(&mut self, on: bool) {
        self.firePrimary = on;
    }
    pub fn setFireSecondary(&mut self, on: bool) {
        self.fireSecondary = on;
    }
    pub fn resetTurnAndThrust(&mut self) {
        self.thrust = false;
        self.resetTurn();
    }
    pub fn resetTurn(&mut self) {
        self.rotateLeft = false;
        self.rotateRight = false;
    }
    pub fn resetShots(&mut self) {
        self.firePrimary = false;
        self.fireSecondary = false;
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AILevelConstants {
    pub distanceToAvoidPlasma: f64,
    pub distanceToAvoidSnot: f64,
    pub distanceToStayAway: f64,
    pub distanceSholevToFreezeAim: f64,
    pub distanceToAvoidSholevIce: f64,
    pub BELOW_MAX_SPEED_TO_SHOOT: f64,
    pub goodAimProbability: f64,
    pub minEnergyLevel: f64,
    pub resumeEnergyLevel: f64,
    pub rateOfLearningAI: f64,
    pub inAndOutdistance: f64,
    pub holdingDistanceBeforeFly: f64,
    pub KAMIKAZE_CLOSE_DIS: f64,
    pub SIT_ON_TAIL_DIS: f64,
    pub DEFAULT_SHOOT_TIME: f64,
    pub DEFAULT_SHOOT_INETRVAL: f64,
    pub DEFAULT_TIME_TO_REST: f64,
}
pub fn getDifficultyConstants(level: u8) -> AILevelConstants {
    let a = match level {
        1 => [
            50.0, 50.0, 150.0, 200.0, 300.0, 0.05, 0.4, 0.8, 6.0, 100.0, 400.0, 200.0, 400.0,
            100.0, 500.0,
        ],
        2 => [
            175.0, 100.0, 300.0, 100.0, 400.0, 0.06, 0.7, 0.5, 5.0, 0.0, 450.0, 150.0, 300.0, 50.0,
            250.0,
        ],
        3 => [
            250.0, 150.0, 350.0, 50.0, 500.0, 0.069, 1.0, 0.4, 4.0, 0.0, 500.0, 100.0, 200.0, 25.0,
            100.0,
        ],
        _ => panic!("AI difficulty must be 1, 2 or 3"),
    };
    AILevelConstants {
        distanceToAvoidPlasma: a[0],
        distanceToAvoidSnot: a[1],
        distanceToStayAway: a[2],
        distanceSholevToFreezeAim: a[3],
        distanceToAvoidSholevIce: a[4],
        BELOW_MAX_SPEED_TO_SHOOT: a[5],
        goodAimProbability: a[6],
        minEnergyLevel: 0.01,
        resumeEnergyLevel: a[7],
        rateOfLearningAI: a[8],
        inAndOutdistance: a[9],
        holdingDistanceBeforeFly: a[10],
        KAMIKAZE_CLOSE_DIS: a[11],
        SIT_ON_TAIL_DIS: a[12],
        DEFAULT_SHOOT_TIME: 1000.0,
        DEFAULT_SHOOT_INETRVAL: a[13],
        DEFAULT_TIME_TO_REST: a[14],
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AILearning {
    pub hittingInAttackTable: [f64; 21],
    pub hittingInDefenseTable: [f64; 21],
    pub gotHitInAttackTable: [f64; 21],
    pub gotHitInDefenseTable: [f64; 21],
    pub attackMaxTactic: f64,
    pub attackMax: f64,
    pub defenseMaxTactic: f64,
    pub defenseMax: f64,
}
impl AILearning {
    pub fn updateTacticsTable(&mut self, attack: bool, tactic: f64, aiHit: bool) {
        let i = tactic as usize;
        if aiHit {
            if attack {
                self.hittingInAttackTable[i] += 1.0;
                if self.hittingInAttackTable[i] > self.attackMax {
                    self.attackMax = self.hittingInAttackTable[i];
                    self.attackMaxTactic = tactic;
                }
            } else {
                self.hittingInDefenseTable[i] += 1.0;
                if self.hittingInDefenseTable[i] > self.defenseMax {
                    self.defenseMax = self.hittingInDefenseTable[i];
                    self.defenseMaxTactic = tactic;
                }
            }
        } else if attack {
            self.gotHitInAttackTable[i] += 1.0;
        } else {
            self.gotHitInDefenseTable[i] += 1.0;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIContext {
    pub num: f64,
    pub constants: AILevelConstants,
    pub sunTable: Point,
    pub learning: AILearning,
    pub level: u8,
    pub tacticTime: f64,
    pub inDisAdvantage: bool,
    pub smallGitchiFlag: bool,
    pub fireWeapon2Flag: bool,
    pub thrustOnceFlag: bool,
    pub energyFillFlag: bool,
    pub cowardReadyFlag: bool,
    pub inLowEnergyAttack: bool,
    pub slowDownSunFlag: bool,
    pub countShootTime: f64,
    pub readyToFire: bool,
    pub rotAngle: f64,
    pub lastFrameCloseRange: f64,
    pub closeRangeCounter: f64,
    pub disAdvantageCounter: f64,
    pub randomAngle: f64,
    pub inAndOutdistance: f64,
}
impl AIContext {
    pub fn new(level: u8) -> Self {
        let constants = getDifficultyConstants(level);
        let inAndOutdistance = constants.inAndOutdistance;
        Self {
            num: 0.0,
            constants,
            sunTable: Point::default(),
            learning: AILearning::default(),
            level,
            tacticTime: 1000.0,
            inDisAdvantage: false,
            smallGitchiFlag: false,
            fireWeapon2Flag: false,
            thrustOnceFlag: false,
            energyFillFlag: false,
            cowardReadyFlag: false,
            inLowEnergyAttack: false,
            slowDownSunFlag: false,
            countShootTime: 0.0,
            readyToFire: true,
            rotAngle: 0.0,
            lastFrameCloseRange: 0.0,
            closeRangeCounter: 0.0,
            disAdvantageCounter: 0.0,
            randomAngle: 0.0,
            inAndOutdistance,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIMode {
    pub frameCounter: f64,
    pub whichTactic: f64,
    pub lastFrameLife: f64,
    pub numOfTacticsPlayed: f64,
}
impl AIMode {
    pub fn new(attack: bool) -> Self {
        Self {
            frameCounter: 0.0,
            whichTactic: if attack { 11.0 } else { 2.0 },
            lastFrameLife: 1.0,
            numOfTacticsPlayed: 1.0,
        }
    }
    pub fn update(
        &mut self,
        attack: bool,
        ai: &PlayerTable,
        user: &PlayerTable,
        out: &mut AIOutput,
        ctx: &mut AIContext,
        rng: &mut dyn FnMut() -> f64,
    ) {
        if !attack
            && ai.distance < 250.0
            && !(ai.entityType == SHOLEV && (self.whichTactic == 7.0 || self.whichTactic == 8.0))
        {
            closeRangeAttack(ai, user, out, ctx, rng);
            return;
        }
        if ai.life < self.lastFrameLife
            || facingAndShooting(ai, user, ctx, rng)
            || (!attack && ctx.inDisAdvantage)
        {
            ctx.inDisAdvantage = true;
            out.resetShots();
            inDisadvantage(ai, user, out, ctx, rng);
            if ai.speed > ai.maxSpeed / 2.0 {
                self.lastFrameLife = ai.life;
            }
            return;
        }
        self.frameCounter += 1.0;
        if self.frameCounter % ctx.tacticTime == 1.0 {
            self.numOfTacticsPlayed += 1.0;
            self.whichTactic = if self.numOfTacticsPlayed % ctx.constants.rateOfLearningAI == 1.0 {
                if attack {
                    ctx.learning.attackMaxTactic
                } else {
                    ctx.learning.defenseMaxTactic
                }
            } else {
                (rng() * 10.0).floor() + if attack { 11.0 } else { 1.0 }
            };
            out.resetShots();
            out.resetTurnAndThrust();
        }
        let t = self.whichTactic;
        if attack {
            if t == 11.0 || t == 12.0 {
                kamikazeAttack(ai, user, out, ctx, rng);
                ctx.tacticTime = 1500.0;
            } else if t == 13.0 || t == 14.0 {
                uniqueTactic(ai, user, out, ctx, rng);
                ctx.tacticTime = 1000.0;
            } else if t == 15.0 || t == 16.0 {
                sitOnTailAttack(ai, user, out, ctx, rng);
                ctx.tacticTime = 2000.0;
            } else if t == 17.0 || t == 18.0 {
                stationaryAttack(ai, user, out, ctx, rng);
                ctx.tacticTime = 1000.0;
            } else if t == 19.0 || t == 20.0 {
                inAndOut(ai, user, out, ctx, rng);
                ctx.tacticTime = 2000.0;
            } else {
                turnAndShoot(ai, user, out, ctx, rng);
                ctx.tacticTime = 3000.0;
            }
        } else if t == 1.0 || t == 2.0 {
            cowardFlying(ai, user, out, ctx, rng);
            ctx.tacticTime = 4000.0;
        } else if t == 3.0 || t == 4.0 {
            lowEnergyAttack(ai, user, out, ctx, rng);
            ctx.tacticTime = 2000.0;
        } else if t == 5.0 || t == 6.0 {
            zigzag(ai, user, out, ctx, rng);
            ctx.tacticTime = 500.0;
        } else if t == 7.0 || t == 8.0 {
            uniqueTactic(ai, user, out, ctx, rng);
            ctx.tacticTime = 1000.0;
        } else {
            turnAndShoot(ai, user, out, ctx, rng);
            ctx.tacticTime = 3000.0;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiController {
    pub level: u8,
    pub num: f64,
    pub out: AIOutput,
    pub attackMode: AIMode,
    pub defenseMode: AIMode,
    pub ctx: AIContext,
    pub lastAILife: f64,
    pub lastUserLife: f64,
}
impl AiController {
    pub fn new(level: u8) -> Self {
        Self {
            level,
            num: 0.0,
            out: AIOutput::default(),
            attackMode: AIMode::new(true),
            defenseMode: AIMode::new(false),
            ctx: AIContext::new(level),
            lastAILife: 1.0,
            lastUserLife: 1.0,
        }
    }
    pub fn update(
        &mut self,
        ai: &AiPlayer,
        user: &AiPlayer,
        rng: &mut dyn FnMut() -> f64,
    ) -> AIOutput {
        self.update_tables(
            &buildPlayerTable(ai, user),
            &buildPlayerTable(user, ai),
            rng,
        )
    }
    /// Equivalent to AIController.update after the bridge. dt intentionally has
    /// no influence: upstream AI counts frames, not milliseconds.
    pub fn update_tables(
        &mut self,
        ai: &PlayerTable,
        user: &PlayerTable,
        rng: &mut dyn FnMut() -> f64,
    ) -> AIOutput {
        self.num += 1.0;
        self.ctx.num = self.num;
        if self.num <= 5.0 {
            return self.out;
        }
        let out = &mut self.out;
        let ctx = &mut self.ctx;
        onceMethod(out, ctx);
        if self.level == 2 || self.level == 3 {
            ctx.inAndOutdistance = ai.weapon1Distance - 100.0;
        }
        if getDistance(ai, &ctx.sunTable) <= ctx.constants.distanceToStayAway {
            stayAtADistanceFromSun(ai, ctx.sunTable, ctx.constants.distanceToStayAway, out, ctx);
        } else if avoidBluePlasma(ai, user, out, ctx)
            || snotsStuckOnAIAction(ai, user, out, ctx)
            || avoidSnot(ai, user, out, ctx)
            || avoidIceSholev(ai, user, out, ctx, rng)
            || shootWhenSnotsActive(ai, user, out, ctx, rng)
        {
            // Short circuit priority is part of the original behavior.
        } else {
            out.resetTurnAndThrust();
            out.resetShots();
            if ai.life >= user.life {
                self.attackMode.update(true, ai, user, out, ctx, rng);
            } else {
                self.defenseMode.update(false, ai, user, out, ctx, rng);
            }
        }
        let attack = ai.life >= user.life;
        let tactic = if attack {
            self.attackMode.whichTactic
        } else {
            self.defenseMode.whichTactic
        };
        if ai.life < self.lastAILife {
            ctx.learning.updateTacticsTable(attack, tactic, false);
        }
        if user.life < self.lastUserLife {
            ctx.learning.updateTacticsTable(attack, tactic, true);
        }
        self.lastAILife = ai.life;
        self.lastUserLife = user.life;
        self.out
    }
    pub fn reset(&mut self) {
        // Original reset retains learned tactic statistics and constants.
        let learning = std::mem::take(&mut self.ctx.learning);
        let constants = self.ctx.constants.clone();
        let sun = self.ctx.sunTable;
        *self = Self::new(self.level);
        self.ctx.learning = learning;
        self.ctx.constants = constants;
        self.ctx.sunTable = sun;
        self.ctx.inAndOutdistance = self.ctx.constants.inAndOutdistance;
    }
}

pub fn getDistance(a: &impl PointLike, b: &impl PointLike) -> f64 {
    let a = a.point();
    let b = b.point();
    getVectorSize(a.x, a.y, b.x, b.y)
}
pub fn getVectorSize(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    (dx * dx + dy * dy).sqrt()
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}
pub fn getAngleBetweenTwoVectors(v1: Vector, v2: Vector) -> f64 {
    if v1.size < 0.0001 || v2.size < 0.0001 {
        return 0.0;
    }
    let dot = (v1.x * v2.x + v1.y * v2.y) / (v1.size * v2.size);
    let clamped = dot.clamp(-1.0, 1.0);
    let mut angle = clamped.acos() * (180.0 / std::f64::consts::PI);
    if v1.x > 0.0 {
        angle = 360.0 - angle;
    }
    angle
}
pub fn getAngleToXY(toX: f64, toY: f64, fromX: f64, fromY: f64) -> f64 {
    let dx = toX - fromX;
    let dy = toY - fromY;
    getVVAngle(dx, dy)
}
pub fn getVVAngle(dxVector: f64, dyVector: f64) -> f64 {
    let size = (dxVector * dxVector + dyVector * dyVector).sqrt();
    if size < 0.0001 {
        return 0.0;
    }
    getAngleBetweenTwoVectors(
        Vector {
            x: dxVector,
            y: dyVector,
            size,
        },
        Vector {
            x: 0.0,
            y: 1.0,
            size: 1.0,
        },
    )
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnDecision {
    pub left: bool,
    pub right: bool,
    pub reached: bool,
}
pub fn turnToAngleDecision(currentAngle: f64, desiredAngle: f64) -> TurnDecision {
    let current = currentAngle.floor();
    let desired = ((desiredAngle % 360.0) + 360.0) % 360.0;
    if (current - desired).abs() <= 1.0 || (current - desired).abs() >= 359.0 {
        return TurnDecision {
            left: false,
            right: false,
            reached: true,
        };
    }
    let plusOneEighty = (current + 180.0) % 360.0;
    let inRight = if current < plusOneEighty {
        desired >= current && desired <= plusOneEighty
    } else {
        !(desired < current && desired > plusOneEighty)
    };
    TurnDecision {
        left: !inRight,
        right: inRight,
        reached: false,
    }
}
pub fn goodAim(
    computer: &PlayerTable,
    user: &PlayerTable,
    probability: f64,
    rng: &mut dyn FnMut() -> f64,
) -> f64 {
    let framesToHit = getDistance(computer, user) * 1.5;
    let futureX = user.x + user.dxVector * framesToHit;
    let futureY = user.y + user.dyVector * framesToHit;
    let correctedAngle = getAngleToXY(futureX, futureY, computer.x, computer.y);
    if rng() * 100.0 < 100.0 * probability {
        correctedAngle
    } else {
        computer.angleToOp
    }
}
pub fn realIsInRange(computer: &PlayerTable, user: &PlayerTable) -> bool {
    let n = getMaxTtlOfShot(computer.entityType, computer.morphState);
    let (sx, sy) = if computer.entityType == GONDI {
        (
            computer.x + computer.dxVector * n,
            computer.y + computer.dyVector * n,
        )
    } else {
        (computer.x, computer.y)
    };
    let ux = user.x + user.dxVector * n;
    let uy = user.y + user.dyVector * n;
    getVectorSize(sx, sy, ux, uy) < computer.weapon1Distance
}
pub fn isInFutureRange(computer: &PlayerTable, user: &PlayerTable, n: f64) -> bool {
    getVectorSize(
        computer.x + computer.dxVector * n,
        computer.y + computer.dyVector * n,
        user.x + user.dxVector * n,
        user.y + user.dyVector * n,
    ) < computer.weapon1Distance
}
