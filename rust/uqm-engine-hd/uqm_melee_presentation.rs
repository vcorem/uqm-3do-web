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
}

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
    resource: *const c_char, hot_x: i16, hot_y: i16) {
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
    if image.is_null() && !avenger { return; }
    let entry = Entry { original: original as usize, rendered: image as usize,
        avenger, melee_ui };
    registry(|entries| entries.push(entry));
}

pub fn UQM_MeleePresentationImage(original: *mut c_void) -> *mut c_void {
    lookup(original).filter(|entry| entry.rendered != 0).map_or(original, |entry| {
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
