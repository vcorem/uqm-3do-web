// SPDX-License-Identifier: GPL-2.0-or-later
//! Optional Melee artwork. Original images remain the sole collision source.
//!
//! The registry owns separate draw images; neither FRAME nor ELEMENT is changed.
//! Entries are removed on the graphics thread when the original image's queued
//! deletion executes, after all draw commands referring to either image.

use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::ffi::{c_char, c_int, c_void, CStr};
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use crate::src::libs::graphics::tfb_draw::{TFB_Image, DrawMode, SCREEN};

extern "C" {
    static mut contentDir: *mut c_void;
    fn uio_fopen(dir: *mut c_void, path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn uio_fclose(stream: *mut c_void) -> c_int;
    fn TFB_DrawCanvas_LoadFromFile(dir: *mut c_void, path: *const c_char) -> *mut c_void;
    fn TFB_DrawImage_New(canvas: *mut c_void) -> *mut TFB_Image;
    fn TFB_DrawImage_Delete(image: *mut TFB_Image);
    fn TFB_DrawScreen_SetMipmap(image: *mut TFB_Image, mipmap: *mut TFB_Image, x: c_int, y: c_int);
    fn TFB_DrawScreen_Image(image: *mut TFB_Image, x: c_int, y: c_int,
        scale: c_int, scale_mode: c_int, cmap: *mut c_void, mode: DrawMode, screen: SCREEN);
}

#[derive(Clone, Copy)]
struct Entry {
    original: usize,
    rendered: usize,
    avenger: bool,
    melee_ui: bool,
    captain: Option<CaptainFrame>,
}

#[derive(Clone, Copy)]
struct CaptainFrame { race: usize, row: usize, drawable: u32, ready: bool }

// Native MeleeShip order, not SPECIES_ID order. ANI rows remain distinct even
// when several rows refer to the same PNG filename.
const CAPTAIN_RACES: [&[u8]; 25] = [b"androsynth", b"arilou", b"chenjesu", b"chmmr",
    b"druuge", b"human", b"ilwrath", b"kohrah", b"melnorme", b"mmrnmhrm",
    b"mycon", b"orz", b"pkunk", b"shofixti", b"slylandro", b"spathi", b"supox",
    b"syreen", b"thraddash", b"umgah", b"urquan", b"utwig", b"vux", b"yehat", b"zoqfotpik"];
const CAPTAIN_ROWS: usize = 20;

struct Registry {
    locked: AtomicBool,
    entries: UnsafeCell<Vec<Entry>>,
}
// All access to entries is guarded; no graphics calls run under this lock.
unsafe impl Sync for Registry {}
static REGISTRY: Registry = Registry {
    locked: AtomicBool::new(false),
    entries: UnsafeCell::new(Vec::new()),
};
static ENABLED: AtomicBool = AtomicBool::new(true);
static DRAW_ACTIVE: AtomicBool = AtomicBool::new(false);
static BACKGROUND: AtomicUsize = AtomicUsize::new(0);
static BACKGROUND_ATTEMPTED: AtomicBool = AtomicBool::new(false);
static DRAW_COUNT: AtomicU32 = AtomicU32::new(0);
static BACKGROUND_COUNT: AtomicU32 = AtomicU32::new(0);
static NEXT_DRAWABLE: AtomicU32 = AtomicU32::new(1);
// 0: outside a portrait batch; 1: original; 2: enhanced. Nested batches inherit
// their parent's snapshot, so a main-thread toggle cannot split a composition.
static CAPTAIN_MODE: AtomicU32 = AtomicU32::new(0);
static CAPTAIN_LAST_IMAGE: [AtomicUsize; 2] = [const { AtomicUsize::new(0) }; 2];
static CAPTAIN_LAST_MODE: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
static CAPTAIN_DRAWS: [AtomicU32; 25 * CAPTAIN_ROWS] = [const { AtomicU32::new(0) }; 25 * CAPTAIN_ROWS];
static CAPTAIN_SEEN: [AtomicU32; 25] = [const { AtomicU32::new(0) }; 25];

fn registry<T>(action: impl FnOnce(&mut Vec<Entry>) -> T) -> T {
    while REGISTRY.locked.compare_exchange_weak(false, true,
        Ordering::Acquire, Ordering::Relaxed).is_err() {
        core::hint::spin_loop();
    }
    struct Unlock;
    impl Drop for Unlock {
        fn drop(&mut self) { REGISTRY.locked.store(false, Ordering::Release); }
    }
    let _unlock = Unlock;
    action(unsafe { &mut *REGISTRY.entries.get() })
}

fn lookup(original: *mut c_void) -> Option<Entry> {
    registry(|entries| entries.iter().find(|e| e.original == original as usize).copied())
}

#[no_mangle]
pub extern "C" fn UQM_SetMeleeRemastered(enabled: c_int) {
    ENABLED.store(enabled != 0, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemastered() -> c_int {
    ENABLED.load(Ordering::Acquire) as c_int
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredImageCount() -> u32 {
    registry(|entries| entries.iter().filter(|entry| entry.rendered != 0).count() as u32)
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredDrawCount() -> u32 {
    DRAW_COUNT.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredBackgroundCount() -> u32 {
    BACKGROUND_COUNT.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredCaptainImageCount(race: c_int) -> u32 {
    registry(|entries| entries.iter().filter(|entry| entry.captain.is_some_and(
        |captain| captain.race == race as usize && captain.ready)).count() as u32)
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredCaptainDrawCount(race: c_int, row: c_int) -> u32 {
    if !(0..25).contains(&race) || !(-1..CAPTAIN_ROWS as c_int).contains(&row) { return 0; }
    let start = race as usize * CAPTAIN_ROWS;
    if row == -1 {
        CAPTAIN_DRAWS[start..start + CAPTAIN_ROWS].iter().fold(0u32,
            |total, count| total.wrapping_add(count.load(Ordering::Relaxed)))
    } else { CAPTAIN_DRAWS[start + row as usize].load(Ordering::Relaxed) }
}

#[no_mangle]
pub extern "C" fn UQM_GetMeleeRemasteredCaptainSeenMask(race: c_int) -> u32 {
    CAPTAIN_SEEN.get(race as usize).map_or(0, |mask| mask.load(Ordering::Relaxed))
}

pub struct CaptainScope(u32);
impl Drop for CaptainScope {
    fn drop(&mut self) { CAPTAIN_MODE.store(self.0, Ordering::Release); }
}

pub fn UQM_MeleePresentationCaptainBegin() -> CaptainScope {
    let previous = CAPTAIN_MODE.load(Ordering::Acquire);
    if previous == 0 {
        let enhanced = ENABLED.load(Ordering::Acquire) && unsafe {
            crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff == 0
        };
        CAPTAIN_MODE.store(if enhanced { 2 } else { 1 }, Ordering::Release);
    }
    CaptainScope(previous)
}

/// A final partial death patch must match the portrait already on screen. There
/// is no later live refresh: retain that pose's style even if the setting changed
/// since PreProcessStatus. This affects drawing only, not the requested setting.
pub fn UQM_MeleePresentationCaptainBeginFinalFrame(player: c_int, portrait: *mut c_void) -> CaptainScope {
    let scope = UQM_MeleePresentationCaptainBegin();
    if scope.0 == 0 && (0..2).contains(&player) && !portrait.is_null() && unsafe {
        crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff == 0
    } && CAPTAIN_LAST_IMAGE[player as usize].load(Ordering::Relaxed) == portrait as usize {
        let mode = CAPTAIN_LAST_MODE[player as usize].load(Ordering::Relaxed);
        if mode != 0 { CAPTAIN_MODE.store(mode, Ordering::Release); }
    }
    scope
}

/// Called from the existing live captain draw path, after native offsets update.
/// Only presentation bookkeeping changes; the caller reads the current pose.
pub fn UQM_MeleePresentationCaptainNeedsRefresh(player: c_int, original: *mut c_void) -> bool {
    if !(0..2).contains(&player) || original.is_null() || unsafe {
        crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff != 0
    } { return false; }
    let mode = CAPTAIN_MODE.load(Ordering::Acquire);
    if mode == 0 { return false; }
    let previous_image = CAPTAIN_LAST_IMAGE[player as usize].swap(original as usize, Ordering::Relaxed);
    let previous_mode = CAPTAIN_LAST_MODE[player as usize].swap(mode, Ordering::Relaxed);
    previous_image != original as usize || previous_mode != mode
}

pub fn UQM_MeleePresentationNextDrawable() -> u32 {
    NEXT_DRAWABLE.fetch_add(1, Ordering::Relaxed)
}

/// A composited portrait is accepted atomically, never a mixture of old and new
/// patches. Original rows and duplicate-PNG aliases are loaded independently.
pub fn UQM_MeleePresentationFinalizeCaptain(drawable: u32, frame_count: usize) {
    registry(|entries| {
        let frames = entries.iter().filter(|entry| entry.captain.is_some_and(|c| c.drawable == drawable));
        let ready = frames.clone().count() == frame_count && frame_count > 0 &&
            frames.clone().all(|entry| entry.rendered != 0) &&
            (0..frame_count).all(|row| frames.clone().any(|entry| entry.captain.unwrap().row == row));
        for entry in entries.iter_mut() {
            if let Some(captain) = entry.captain.as_mut().filter(|c| c.drawable == drawable) {
                captain.ready = ready;
            }
        }
    });
}

pub struct DrawScope;
impl Drop for DrawScope {
    fn drop(&mut self) { UQM_MeleePresentationEnd(); }
}

/// Snapshot the user setting once per displayed frame, outside simulation.
pub fn UQM_MeleePresentationBegin(is_melee: bool) -> DrawScope {
    DRAW_ACTIVE.store(is_melee && ENABLED.load(Ordering::Acquire), Ordering::Release);
    DrawScope
}

pub fn UQM_MeleePresentationEnd() { DRAW_ACTIVE.store(false, Ordering::Release); }

unsafe fn load_optional(path: &[u8]) -> *mut TFB_Image {
    // Avoid noisy missing-file diagnostics for intentionally unmodified assets.
    let file = uio_fopen(contentDir, path.as_ptr().cast(), b"rb\0".as_ptr().cast());
    if file.is_null() { return core::ptr::null_mut(); }
    uio_fclose(file);
    let canvas = TFB_DrawCanvas_LoadFromFile(contentDir, path.as_ptr().cast());
    if canvas.is_null() { return core::ptr::null_mut(); }
    TFB_DrawImage_New(canvas)
}

/// Called only after the original PNG and its geometry have been finalized.
pub unsafe fn UQM_MeleePresentationRegister(original: *mut c_void, source: *const c_char,
    resource: *const c_char, hot_x: i16, hot_y: i16, drawable: u32, row: usize) {
    if original.is_null() || source.is_null() { return; }
    let source = CStr::from_ptr(source).to_bytes();
    let Some(offset) = source.windows(7).position(|s| s == b"/ships/")
        .or_else(|| source.windows(8).position(|s| s == b"/battle/")) else { return; };
    let relative = &source[offset + 1..];
    if !relative.ends_with(b".png") || relative.windows(2).any(|s| s == b"..") { return; }
    let mut path = Vec::with_capacity(19 + relative.len());
    path.extend_from_slice(b"remastered/render/");
    path.extend_from_slice(relative);
    path.push(0);
    let avenger = relative.starts_with(b"ships/ilwrath/avenger-");
    // Melee icon drawables also contain the small facing PNGs. Classify the
    // originating ANI, not only filenames, so those fleet previews are covered.
    let melee_ui = !resource.is_null() && {
        let resource = CStr::from_ptr(resource).to_bytes();
        resource.ends_with(b"-icons.ani") || resource.ends_with(b"-meleeicons.ani")
    };
    let captain = (!resource.is_null() && CStr::from_ptr(resource).to_bytes().ends_with(b"-cap.ani"))
        .then(|| relative.strip_prefix(b"ships/").and_then(|path| path.split(|c| *c == b'/').next()))
        .flatten().and_then(|race| CAPTAIN_RACES.iter().position(|name| *name == race))
        .filter(|_| row < CAPTAIN_ROWS)
        .map(|race| CaptainFrame { race, row, drawable, ready: false });
    let mut image = load_optional(&path);
    let baseline = original.cast::<TFB_Image>();
    // Reject a mispackaged frame. Presentation has identical logical geometry.
    if !image.is_null() && ((*image).extent.width != (*baseline).extent.width ||
       (*image).extent.height != (*baseline).extent.height) {
        TFB_DrawImage_Delete(image);
        image = core::ptr::null_mut();
    }
    if !image.is_null() {
        (*image).NormalHs.x = hot_x;
        (*image).NormalHs.y = hot_y;
    }
    // Identity is independent of optional art: missing art must never reveal a
    // cloaked Avenger as a black silhouette against the nebula.
    if image.is_null() && !avenger && captain.is_none() { return; }
    let entry = Entry { original: original as usize, rendered: image as usize,
        avenger, melee_ui, captain };
    registry(|entries| entries.push(entry));
}

pub fn UQM_MeleePresentationImage(original: *mut c_void) -> *mut c_void {
    lookup(original).filter(|entry| entry.rendered != 0).map_or(original, |entry| {
        if let Some(captain) = entry.captain {
            if !captain.ready || CAPTAIN_MODE.load(Ordering::Acquire) != 2 { return original; }
            CAPTAIN_DRAWS[captain.race * CAPTAIN_ROWS + captain.row].fetch_add(1, Ordering::Relaxed);
            CAPTAIN_SEEN[captain.race].fetch_or(1 << captain.row, Ordering::Relaxed);
            DRAW_COUNT.fetch_add(1, Ordering::Relaxed);
            return entry.rendered as *mut c_void;
        }
        // UI draws run outside RedrawQueue. Read activity only on this existing
        // game-thread draw path, and never modify it or any other game state.
        if !DRAW_ACTIVE.load(Ordering::Acquire) && !(entry.melee_ui &&
            ENABLED.load(Ordering::Acquire) && unsafe {
                crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff == 0
            }) { return original; }
        DRAW_COUNT.fetch_add(1, Ordering::Relaxed);
        entry.rendered as *mut c_void
    })
}

/// A fully cloaked Avenger must stay invisible against the new backdrop.
/// Other filled stamps keep the original silhouette and native shield colors.
pub fn UQM_MeleePresentationHiddenCloak(original: *mut c_void, r: u8, g: u8, b: u8) -> bool {
    DRAW_ACTIVE.load(Ordering::Acquire) && r == 0 && g == 0 && b == 0 &&
        lookup(original).is_some_and(|entry| entry.avenger)
}

pub unsafe fn UQM_MeleePresentationMipmap(original: *mut c_void, mipmap: *mut c_void, x: c_int, y: c_int) {
    let (Some(image), Some(mipmap)) = (lookup(original), lookup(mipmap)) else { return; };
    if image.rendered == 0 || mipmap.rendered == 0 { return; }
    // Re-entering with presentation pointers has no registry match.
    TFB_DrawScreen_SetMipmap(image.rendered as *mut TFB_Image,
        mipmap.rendered as *mut TFB_Image, x, y);
}

pub unsafe fn UQM_MeleePresentationForget(original: *mut c_void) {
    let rendered = registry(|entries| entries.iter().position(|e| e.original == original as usize)
        .map(|index| entries.swap_remove(index).rendered));
    if let Some(rendered) = rendered.filter(|rendered| *rendered != 0) {
        TFB_DrawImage_Delete(rendered as *mut TFB_Image);
    }
}

pub unsafe fn UQM_MeleePresentationBackground() {
    if !DRAW_ACTIVE.load(Ordering::Acquire) { return; }
    if !BACKGROUND_ATTEMPTED.swap(true, Ordering::AcqRel) {
        let image = load_optional(b"remastered/render/background.png\0");
        BACKGROUND.store(image as usize, Ordering::Release);
    }
    let image = BACKGROUND.load(Ordering::Acquire) as *mut TFB_Image;
    if image.is_null() { return; }
    BACKGROUND_COUNT.fetch_add(1, Ordering::Relaxed);
    // Existing SpaceContext clipping prevents painting over the native HUD.
    TFB_DrawScreen_Image(image, 0, 0, 256, 0, core::ptr::null_mut(),
        DrawMode { kind: 0, factor: 0 }, SCREEN::TFB_SCREEN_MAIN);
}
