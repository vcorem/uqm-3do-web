// SPDX-License-Identifier: GPL-2.0-or-later
//! Opt-in Afterlight campaign encounter mailbox. The browser only configures
//! fleets; every ship, weapon, AI decision, casualty and winner is native UQM.
//! Atomics synchronize the browser thread with the existing game thread.
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub const MAX_CREW: [u32; 25] = [20, 6, 36, 42, 14, 18, 22, 42, 20, 20, 20, 16, 8, 6, 12, 30, 12, 42, 8, 10, 42, 20, 20, 20, 10];
static ENABLED: AtomicBool = AtomicBool::new(false);
// 0 disabled; 1 idle; 2 command queued; 3 battle running; 4 result available;
// 5 writer owns the mailbox. Only the game thread transitions 2 -> 3 -> 4.
static PHASE: AtomicU32 = AtomicU32::new(0);
static PLAYER: AtomicU32 = AtomicU32::new(5);
static CREW: AtomicU32 = AtomicU32::new(18);
static SEED: AtomicU32 = AtomicU32::new(1);
static RATING: AtomicU32 = AtomicU32::new(16);
static ENEMIES: [AtomicU32; 14] = [const { AtomicU32::new(255) }; 14];
static RETREAT: AtomicBool = AtomicBool::new(false);
static OUTCOME: AtomicU32 = AtomicU32::new(0);
static REMAINING: AtomicU32 = AtomicU32::new(0);
static FRAMES: AtomicU32 = AtomicU32::new(0);
static INITIAL_CREW_APPLIED: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "campaign-test-driver")]
static TEST_AI: AtomicBool = AtomicBool::new(false);

pub fn enabled() -> bool { ENABLED.load(Ordering::Acquire) }
pub fn pending() -> bool { PHASE.load(Ordering::Acquire) == 2 }
pub fn running() -> bool { enabled() && PHASE.load(Ordering::Acquire) == 3 }
pub fn player() -> u32 { PLAYER.load(Ordering::Relaxed) }
pub fn crew() -> u32 { CREW.load(Ordering::Relaxed) }
pub fn seed() -> u32 { SEED.load(Ordering::Relaxed) }
pub fn rating() -> u32 { RATING.load(Ordering::Relaxed) }
pub fn enemy(index: usize) -> u32 { ENEMIES[index].load(Ordering::Relaxed) }
pub fn retreat_requested() -> bool { running() && RETREAT.load(Ordering::Acquire) }
pub fn frame() { if running() { FRAMES.fetch_add(1, Ordering::Relaxed); } }
pub fn observe_crew(side: usize, value: u32) {
    if running() && side == 0 { CREW.store(value, Ordering::Relaxed); }
}
pub fn starting_crew(maximum: u32) -> Option<u32> {
    if running() && !INITIAL_CREW_APPLIED.swap(true, Ordering::Relaxed) { Some(crew().min(maximum)) }
    else { None }
}
pub fn start() { PHASE.store(3, Ordering::Release); }
pub fn result(player_left: u32, enemies_left: u32, aborted: bool) {
    if !running() { return; }
    REMAINING.store(enemies_left, Ordering::Relaxed);
    // Casualties win over a simultaneous retreat click. No resurrection.
    let outcome = if player_left == 0 { CREW.store(0, Ordering::Relaxed); 2 }
        else if enemies_left == 0 { 1 } else if aborted || RETREAT.load(Ordering::Acquire) { 3 }
        else { 4 }; // An incomplete/unexpected native result is an error, never victory.
    OUTCOME.store(outcome, Ordering::Relaxed);
}
pub fn complete() {
    if running() {
        if OUTCOME.load(Ordering::Relaxed) == 0 { OUTCOME.store(4, Ordering::Relaxed); }
        PHASE.store(4, Ordering::Release);
    }
}
pub fn player_control() -> u8 {
    #[cfg(feature = "campaign-test-driver")]
    if TEST_AI.load(Ordering::Relaxed) { return 6 | 64; }
    1 | 16
}
#[no_mangle]
pub extern "C" fn uqm_campaign_enable() -> u32 {
    if !ENABLED.swap(true, Ordering::AcqRel) { PHASE.store(1, Ordering::Release); }
    1
}
#[no_mangle]
pub extern "C" fn uqm_campaign_begin(ship: u32, crew: u32, seed: u32, difficulty: u32,
    e0: u32, e1: u32, e2: u32, e3: u32, e4: u32, e5: u32, e6: u32,
    e7: u32, e8: u32, e9: u32, e10: u32, e11: u32, e12: u32, e13: u32) -> u32 {
    let enemies = [e0,e1,e2,e3,e4,e5,e6,e7,e8,e9,e10,e11,e12,e13];
    if !enabled() || ship >= 25 || crew == 0 || crew > MAX_CREW[ship as usize] || difficulty > 2
        || e0 >= 25 || enemies.iter().any(|v| *v >= 25 && *v != 255) { return 0; }
    let phase = PHASE.load(Ordering::Acquire);
    if !matches!(phase, 1 | 4) || PHASE.compare_exchange(phase, 5, Ordering::AcqRel, Ordering::Acquire).is_err() { return 0; }
    PLAYER.store(ship, Ordering::Relaxed); CREW.store(crew, Ordering::Relaxed);
    SEED.store(seed, Ordering::Relaxed); RATING.store(16 << difficulty, Ordering::Relaxed);
    for (dest, value) in ENEMIES.iter().zip(enemies) { dest.store(value, Ordering::Relaxed); }
    RETREAT.store(false, Ordering::Relaxed); OUTCOME.store(0, Ordering::Relaxed);
    FRAMES.store(0, Ordering::Relaxed);
    INITIAL_CREW_APPLIED.store(false, Ordering::Relaxed);
    REMAINING.store(enemies.iter().filter(|v| **v != 255).count() as u32, Ordering::Relaxed);
    PHASE.store(2, Ordering::Release);
    1
}
#[no_mangle]
pub extern "C" fn uqm_campaign_phase() -> u32 { PHASE.load(Ordering::Acquire) }
#[no_mangle]
pub extern "C" fn uqm_campaign_outcome() -> u32 { OUTCOME.load(Ordering::Acquire) }
#[no_mangle]
pub extern "C" fn uqm_campaign_crew() -> u32 { CREW.load(Ordering::Acquire) }
#[no_mangle]
pub extern "C" fn uqm_campaign_remaining() -> u32 { REMAINING.load(Ordering::Acquire) }
#[no_mangle]
pub extern "C" fn uqm_campaign_frames() -> u32 { FRAMES.load(Ordering::Acquire) }
#[no_mangle]
pub extern "C" fn uqm_campaign_retreat() -> u32 {
    if !matches!(PHASE.load(Ordering::Acquire), 2 | 3) { return 0; }
    RETREAT.store(true, Ordering::Release); 1
}
#[cfg(feature = "campaign-test-driver")]
#[no_mangle]
pub extern "C" fn uqm_campaign_test_ai(value: u32) -> u32 {
    if !matches!(PHASE.load(Ordering::Acquire), 1 | 4) { return 0; }
    TEST_AI.store(value != 0, Ordering::Relaxed); 1
}

#[cfg(test)]
mod tests {
    use super::*;
    fn begin(ship: u32, crew: u32, enemy: u32) -> u32 {
        uqm_campaign_begin(ship,crew,7,1,enemy,255,255,255,255,255,255,255,255,255,255,255,255,255)
    }
    #[test]
    fn mailbox_validation_and_authentic_outcomes() {
        assert!(!enabled()); assert_eq!(begin(5,18,19),0);
        assert_eq!(uqm_campaign_enable(),1); assert_eq!(uqm_campaign_phase(),1);
        for (ship,crew,enemy) in [(25,1,1),(5,0,1),(5,19,1),(5,18,25),(5,18,255)] {
            assert_eq!(begin(ship,crew,enemy),0); assert_eq!(uqm_campaign_phase(),1);
        }
        assert_eq!(begin(5,17,19),1); assert_eq!(begin(5,18,19),0);
        assert!(pending()); start(); observe_crew(1,1); assert_eq!(crew(),17);
        assert_eq!(starting_crew(18),Some(17)); assert_eq!(starting_crew(18),None);
        observe_crew(0,9); frame(); result(1,0,false); complete();
        assert_eq!((uqm_campaign_phase(),uqm_campaign_outcome(),crew(),uqm_campaign_frames()),(4,1,9,1));
        assert_eq!(begin(5,9,19),1); start(); assert_eq!(uqm_campaign_retreat(),1);
        result(1,1,true); complete(); assert_eq!(uqm_campaign_outcome(),3);
        assert_eq!(begin(5,9,19),1); start(); uqm_campaign_retreat(); result(0,1,true); complete();
        assert_eq!(uqm_campaign_outcome(),2); assert_eq!(crew(),0);
        assert_eq!(begin(5,9,19),1); start(); result(1,1,false); complete(); assert_eq!(uqm_campaign_outcome(),4);
    }
}
