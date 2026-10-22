// Compile the production presentation registry against a small graphics backend.
// This checks ownership and collision-data isolation without emulating Melee.
#![allow(non_snake_case, non_camel_case_types, dead_code)]
extern crate alloc;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod src {
pub mod uqm { pub mod globdata {
    pub struct GameState { pub CurrentActivity: u16 }
    pub struct GlobalData { pub Game_state: GameState }
    pub static mut GlobData: GlobalData = GlobalData { Game_state: GameState { CurrentActivity: 1 } };
} }
pub mod libs { pub mod graphics { pub mod tfb_draw {
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Point { pub x: i16, pub y: i16 }
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Extent { pub width: i16, pub height: i16 }
    #[derive(Clone, Debug, PartialEq)]
    #[repr(C)]
    pub struct TFB_Image { pub extent: Extent, pub NormalHs: Point, pub collision_pixels: [u8; 6] }
    #[repr(C)]
    pub struct DrawMode { pub kind: u8, pub factor: i16 }
    #[repr(transparent)]
    pub struct SCREEN(pub u32);
    impl SCREEN { pub const TFB_SCREEN_MAIN: Self = Self(0); }
} } } }
use src::libs::graphics::tfb_draw::*;
#[path = "../../rust/uqm-engine-hd/uqm_melee_presentation.rs"]
mod presentation;

static FILES: Mutex<Vec<(String, (i16, i16))>> = Mutex::new(Vec::new());
static READ_PATH: Mutex<String> = Mutex::new(String::new());
static DELETED: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static MIPMAPS: Mutex<Vec<(usize, usize, i32, i32)>> = Mutex::new(Vec::new());
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
#[no_mangle]
static mut contentDir: *mut c_void = std::ptr::null_mut();

#[no_mangle]
unsafe extern "C" fn uio_fopen(_: *mut c_void, path: *const c_char, _: *const c_char) -> *mut c_void {
    let path = CStr::from_ptr(path).to_str().unwrap();
    if FILES.lock().unwrap().iter().any(|(name, _)| name == path) {
        std::ptr::dangling_mut()
    } else { std::ptr::null_mut() }
}
#[no_mangle]
unsafe extern "C" fn uio_fclose(_: *mut c_void) -> c_int { 0 }
#[no_mangle]
unsafe extern "C" fn TFB_DrawCanvas_LoadFromFile(_: *mut c_void, path: *const c_char) -> *mut c_void {
    let path = CStr::from_ptr(path).to_str().unwrap();
    *READ_PATH.lock().unwrap() = path.to_owned();
    let size = FILES.lock().unwrap().iter().find(|(name, _)| name == path).unwrap().1;
    Box::into_raw(Box::new(size)).cast()
}
#[no_mangle]
unsafe extern "C" fn TFB_DrawImage_New(canvas: *mut c_void) -> *mut TFB_Image {
    let (width, height) = *Box::from_raw(canvas.cast::<(i16, i16)>());
    make_image(width, height)
}
#[no_mangle]
unsafe extern "C" fn TFB_DrawImage_Delete(image: *mut TFB_Image) {
    presentation::UQM_MeleePresentationForget(image.cast());
    DELETED.lock().unwrap().push(image as usize);
    drop(Box::from_raw(image));
}
#[no_mangle]
unsafe extern "C" fn TFB_DrawScreen_SetMipmap(image: *mut TFB_Image, mipmap: *mut TFB_Image, x: c_int, y: c_int) {
    MIPMAPS.lock().unwrap().push((image as usize, mipmap as usize, x, y));
    presentation::UQM_MeleePresentationMipmap(image.cast(), mipmap.cast(), x, y);
}
#[no_mangle]
unsafe extern "C" fn TFB_DrawScreen_Image(_: *mut TFB_Image, _: c_int, _: c_int,
    _: c_int, _: c_int, _: *mut c_void, _: DrawMode, _: SCREEN) {}

fn make_image(width: i16, height: i16) -> *mut TFB_Image {
    ALLOCATED.fetch_add(1, Ordering::Relaxed);
    Box::into_raw(Box::new(TFB_Image { extent: Extent { width, height },
        NormalHs: Point { x: 11, y: 17 }, collision_pixels: [0, 0, 255, 64, 255, 0] }))
}

unsafe fn register(image: *mut TFB_Image, filename: &str, size: (i16, i16)) {
    let relative = filename.split_once("/ships/").map(|(_, rest)| format!("ships/{rest}"))
        .or_else(|| filename.split_once("/battle/").map(|(_, rest)| format!("battle/{rest}"))).unwrap();
    FILES.lock().unwrap().push((format!("remastered/render/{relative}"), size));
    presentation::UQM_MeleePresentationRegister(image.cast(), CString::new(filename).unwrap().as_ptr(),
        c"battle-hull.ani".as_ptr(), 11, 17, 0, 0);
}

#[test]
fn presentation_never_changes_collision_images_and_owns_its_resources() { unsafe {
    use presentation::*;
    FILES.lock().unwrap().clear();
    DELETED.lock().unwrap().clear();
    MIPMAPS.lock().unwrap().clear();
    let hull = make_image(145, 145);
    let medium = make_image(72, 72);
    let cloak = make_image(100, 100);
    let effect = make_image(24, 24);
    let invalid = make_image(40, 40);
    let missing_cloak = make_image(100, 100);
    let invalid_cloak = make_image(100, 100);
    let hud_icon = make_image(59, 152);
    let fleet_preview = make_image(37, 37);
    let original = (*hull).clone();
    register(hull, "addons/mm-hd/ships/human/cruiser-big-003.png", (145, 145));
    assert_eq!(&*READ_PATH.lock().unwrap(), "remastered/render/ships/human/cruiser-big-003.png");
    register(medium, "addons/mm-hd/ships/human/cruiser-med-003.png", (72, 72));
    register(cloak, "addons/mm-hd/ships/ilwrath/avenger-big-000.png", (100, 100));
    register(effect, "addons/mm-hd/battle/blast-big-000.png", (24, 24));
    register(invalid, "addons/mm-hd/ships/mmrnmhrm/ywing-big-000.png", (41, 40));
    UQM_MeleePresentationRegister(missing_cloak.cast(),
        c"addons/mm-hd/ships/ilwrath/avenger-big-001.png".as_ptr(), c"avenger-big.ani".as_ptr(), 11, 17, 0, 0);
    register(invalid_cloak, "addons/mm-hd/ships/ilwrath/avenger-big-002.png", (101, 100));
    FILES.lock().unwrap().extend([
        ("remastered/render/ships/human/cruiser-icons-001.png".to_owned(), (59, 152)),
        ("remastered/render/ships/human/cruiser-sml-001.png".to_owned(), (37, 37)),
    ]);
    UQM_MeleePresentationRegister(hud_icon.cast(), c"addons/mm-hd/ships/human/cruiser-icons-001.png".as_ptr(),
        c"addons/mm-hd/ships/human/cruiser-icons.ani".as_ptr(), 11, 17, 0, 0);
    UQM_MeleePresentationRegister(fleet_preview.cast(), c"addons/mm-hd/ships/human/cruiser-sml-001.png".as_ptr(),
        c"addons/mm-hd/ships/human/cruiser-meleeicons.ani".as_ptr(), 11, 17, 0, 0);
    assert_eq!(UQM_GetMeleeRemasteredImageCount(), 6);
    assert_eq!(*hull, original, "registration cannot alter dimensions, hotspot, or collision pixels");
    assert_eq!(UQM_MeleePresentationImage(hull.cast()), hull.cast(), "menus are unchanged");
    for icon in [hud_icon, fleet_preview] {
        assert_eq!(UQM_MeleePresentationImage(icon.cast()), icon.cast(), "campaign icons stay original");
    }
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 0;
    for icon in [hud_icon, fleet_preview] {
        assert_ne!(UQM_MeleePresentationImage(icon.cast()), icon.cast(),
            "Melee HUD and facing previews can use art outside the battle draw scope");
    }
    assert_eq!(UQM_MeleePresentationImage(hull.cast()), hull.cast(), "battle hulls require battle draw scope");
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 1;
    {
        let _scope = UQM_MeleePresentationBegin(false);
        assert_eq!(UQM_MeleePresentationImage(hull.cast()), hull.cast(), "campaign is unchanged");
    }
    let rendered;
    {
        let _scope = UQM_MeleePresentationBegin(true);
        rendered = UQM_MeleePresentationImage(hull.cast());
        assert_ne!(rendered, hull.cast());
        assert_ne!(UQM_MeleePresentationImage(effect.cast()), effect.cast());
        assert_eq!(UQM_MeleePresentationImage(invalid.cast()), invalid.cast(), "bad geometry falls back");
        assert!(UQM_MeleePresentationHiddenCloak(cloak.cast(), 0, 0, 0));
        for fallback in [missing_cloak, invalid_cloak] {
            assert_eq!(UQM_MeleePresentationImage(fallback.cast()), fallback.cast());
            assert!(UQM_MeleePresentationHiddenCloak(fallback.cast(), 0, 0, 0),
                "missing or malformed optional art must not reveal cloaked ships");
        }
        assert!(!UQM_MeleePresentationHiddenCloak(cloak.cast(), 1, 0, 0), "cloak transition colors stay visible");
        assert!(!UQM_MeleePresentationHiddenCloak(hull.cast(), 0, 0, 0));
        assert_eq!(*hull, original, "drawing cannot alter collision data");
        UQM_SetMeleeRemastered(0);
        assert_eq!(UQM_MeleePresentationImage(hull.cast()), rendered, "toggle takes effect at next frame");
    }
    {
        let _scope = UQM_MeleePresentationBegin(true);
        assert_eq!(UQM_MeleePresentationImage(hull.cast()), hull.cast());
        assert!(!UQM_MeleePresentationHiddenCloak(cloak.cast(), 0, 0, 0));
    }
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 0;
    for icon in [hud_icon, fleet_preview] {
        assert_eq!(UQM_MeleePresentationImage(icon.cast()), icon.cast(), "classic mode also preserves Melee UI");
    }
    UQM_MeleePresentationMipmap(hull.cast(), medium.cast(), 11, 17);
    let mappings = MIPMAPS.lock().unwrap();
    assert_eq!(mappings.len(), 1, "mipmap forwarding must terminate and use separate images");
    assert_eq!(mappings[0].0, rendered as usize);
    assert_ne!(mappings[0].1, medium as usize);
    drop(mappings);
    UQM_MeleePresentationMipmap(missing_cloak.cast(), invalid_cloak.cast(), 11, 17);
    assert_eq!(MIPMAPS.lock().unwrap().len(), 1, "identity-only entries have no mipmap images");
    for image in [hull, medium, cloak, effect, invalid, missing_cloak, invalid_cloak, hud_icon, fleet_preview] { TFB_DrawImage_Delete(image); }
    assert_eq!(UQM_GetMeleeRemasteredImageCount(), 0);
    assert!(DELETED.lock().unwrap().contains(&(rendered as usize)));
    assert_eq!(DELETED.lock().unwrap().len(), 17, "all baseline, accepted and rejected presentation images freed");
    UQM_SetMeleeRemastered(1);
} }

unsafe fn captain_set(race: &str, count: usize, missing: Option<usize>, malformed: Option<usize>)
    -> (u32, Vec<*mut TFB_Image>) {
    let drawable = presentation::UQM_MeleePresentationNextDrawable();
    let frames: Vec<_> = (0..count).map(|row| if row == 0 { make_image(220, 120) }
        else { make_image(24, 16) }).collect();
    // The native loader visits ANI rows backwards. Rows1/2 deliberately share
    // a PNG, but have independent images, hotspots, and diagnostic counters.
    for row in (0..count).rev() {
        let png_row = if row == 2 { 1 } else { row };
        let relative = format!("ships/{race}/portrait-cap-{png_row:03}.png");
        if Some(row) != missing {
            let size = if row == 0 { (220, 120) } else if Some(row) == malformed { (25, 16) }
                else { (24, 16) };
            FILES.lock().unwrap().push((format!("remastered/render/{relative}"), size));
        }
        presentation::UQM_MeleePresentationRegister(frames[row].cast(),
            CString::new(format!("addons/mm-hd/{relative}")).unwrap().as_ptr(),
            CString::new(format!("addons/mm-hd/ships/{race}/portrait-cap.ani")).unwrap().as_ptr(),
            -(row as i16) * 3, -(row as i16) * 2, drawable, row);
    }
    (drawable, frames)
}

#[test]
fn captains_compose_atomically_and_refresh_without_changing_native_frames() { unsafe {
    use presentation::*;
    FILES.lock().unwrap().clear();
    DELETED.lock().unwrap().clear();
    let allocated_before = ALLOCATED.load(Ordering::Relaxed);
    UQM_SetMeleeRemastered(1);
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 0;
    let (drawable, frames) = captain_set("human", 15, None, None);
    let native: Vec<_> = frames.iter().map(|frame| (**frame).clone()).collect();
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(5), 0, "partial ANI is never displayed");
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert!(frames.iter().all(|frame| UQM_MeleePresentationImage((*frame).cast()) == (*frame).cast()));
    }
    UQM_MeleePresentationFinalizeCaptain(drawable, 16);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(5), 0, "missing ANI rows reject the whole set");
    UQM_MeleePresentationFinalizeCaptain(drawable, 15);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(5), 15);
    {
        let _battle = UQM_MeleePresentationBegin(true);
        assert_eq!(UQM_MeleePresentationImage(frames[0].cast()), frames[0].cast(),
            "captains need their own composition scope, not the ship battle scope");
    }
    let mut rendered = Vec::new();
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert!(UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        assert!(!UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        assert!(UQM_MeleePresentationCaptainNeedsRefresh(1, frames[0].cast()), "each HUD refreshes independently");
        for (row, frame) in frames.iter().enumerate() {
            let image = UQM_MeleePresentationImage((*frame).cast()).cast::<TFB_Image>();
            assert_ne!(image, *frame);
            assert_eq!((*image).NormalHs, Point { x: -(row as i16) * 3, y: -(row as i16) * 2 });
            rendered.push(image);
        }
        assert_ne!(rendered[1], rendered[2], "PNG aliases retain separate ANI rows");
        assert_eq!(UQM_GetMeleeRemasteredCaptainSeenMask(5), (1 << 15) - 1);
        assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(5, 1), 1);
        assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(5, 2), 1);
        UQM_SetMeleeRemastered(0);
        let _nested = UQM_MeleePresentationCaptainBegin();
        for (frame, expected) in frames.iter().zip(&rendered) {
            assert_eq!(UQM_MeleePresentationImage((*frame).cast()), (*expected).cast(),
                "a toggle cannot mix background and action-patch styles mid-composition");
        }
    }
    let enhanced_count = UQM_GetMeleeRemasteredCaptainDrawCount(5, -1);
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert!(UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        assert!(!UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        assert!(frames.iter().all(|frame| UQM_MeleePresentationImage((*frame).cast()) == (*frame).cast()));
    }
    assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(5, -1), enhanced_count, "classic draws do not increment enhanced diagnostics");
    UQM_SetMeleeRemastered(1);
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert!(UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        for (frame, expected) in frames.iter().zip(&rendered) {
            assert_eq!(UQM_MeleePresentationImage((*frame).cast()), (*expected).cast());
        }
    }
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 1;
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert!(!UQM_MeleePresentationCaptainNeedsRefresh(0, frames[0].cast()));
        assert!(frames.iter().all(|frame| UQM_MeleePresentationImage((*frame).cast()) == (*frame).cast()), "campaign portraits stay original");
    }
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 0;
    let (missing_id, missing) = captain_set("arilou", 15, Some(5), None);
    let (malformed_id, malformed) = captain_set("chenjesu", 15, None, Some(4));
    UQM_MeleePresentationFinalizeCaptain(missing_id, 15);
    UQM_MeleePresentationFinalizeCaptain(malformed_id, 15);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(1), 0);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(2), 0);
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        for frame in missing.iter().chain(&malformed) {
            assert_eq!(UQM_MeleePresentationImage((*frame).cast()), (*frame).cast(),
                "one missing or mis-sized patch falls back for the whole captain");
        }
    }
    let (shofixti_id, shofixti) = captain_set("shofixti", 20, None, None);
    UQM_MeleePresentationFinalizeCaptain(shofixti_id, 20);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(13), 20);
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        assert_ne!(UQM_MeleePresentationImage(shofixti[19].cast()), shofixti[19].cast());
    }
    assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(13, 19), 1);
    assert_eq!(UQM_GetMeleeRemasteredCaptainSeenMask(13), 1 << 19);
    // A real death draw is a later, separate batch containing only row19.
    // Toggle between the live pose and that partial stamp in both directions.
    for old_style in [0, 1] {
        UQM_SetMeleeRemastered(old_style);
        {
            let _live = UQM_MeleePresentationCaptainBegin();
            UQM_MeleePresentationCaptainNeedsRefresh(1, shofixti[0].cast());
            assert_eq!(UQM_MeleePresentationImage(shofixti[0].cast()) != shofixti[0].cast(), old_style != 0);
        }
        UQM_SetMeleeRemastered(1 - old_style);
        {
            let _death = UQM_MeleePresentationCaptainBeginFinalFrame(1, shofixti[0].cast());
            let _nested_frame = UQM_MeleePresentationCaptainBegin();
            assert_eq!(UQM_MeleePresentationImage(shofixti[19].cast()) != shofixti[19].cast(), old_style != 0,
                "the partial death patch must match the existing base after a concurrent toggle");
            assert_eq!(UQM_GetMeleeRemastered(), 1 - old_style, "the final patch must not revert the user's setting");
        }
        {
            let _next = UQM_MeleePresentationCaptainBegin();
            assert_eq!(UQM_MeleePresentationImage(shofixti[19].cast()) != shofixti[19].cast(), old_style == 0,
                "the final-frame scope must not leak into later portrait draws");
            assert!(UQM_MeleePresentationCaptainNeedsRefresh(1, shofixti[0].cast()));
        }
    }
    UQM_SetMeleeRemastered(1);
    {
        let _live = UQM_MeleePresentationCaptainBegin();
        UQM_MeleePresentationCaptainNeedsRefresh(1, shofixti[0].cast());
    }
    UQM_SetMeleeRemastered(0);
    {
        let _different = UQM_MeleePresentationCaptainBeginFinalFrame(1, shofixti[1].cast());
        assert_eq!(UQM_MeleePresentationImage(shofixti[19].cast()), shofixti[19].cast(),
            "style inheritance must match the exact portrait identity");
    }
    UQM_SetMeleeRemastered(1);
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 1;
    {
        let _death = UQM_MeleePresentationCaptainBeginFinalFrame(1, shofixti[0].cast());
        assert_eq!(UQM_MeleePresentationImage(shofixti[19].cast()), shofixti[19].cast(),
            "final-frame style inheritance must not affect campaign portraits");
    }
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 0;
    assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(13, 20), 0);
    assert_eq!(UQM_GetMeleeRemasteredCaptainDrawCount(-1, 0), 0);
    assert_eq!(UQM_GetMeleeRemasteredCaptainSeenMask(25), 0);
    for (frame, original) in frames.iter().zip(native) {
        assert_eq!(**frame, original, "native pixels, geometry and hotspots are immutable");
    }
    for frame in frames.into_iter().chain(missing).chain(malformed).chain(shofixti) { TFB_DrawImage_Delete(frame); }
    assert_eq!(UQM_GetMeleeRemasteredImageCount(), 0);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(5), 0);
    let (reload_id, reloaded) = captain_set("human", 15, None, None);
    UQM_MeleePresentationFinalizeCaptain(reload_id, 15);
    assert_eq!(UQM_GetMeleeRemasteredCaptainImageCount(5), 15);
    {
        let _captain = UQM_MeleePresentationCaptainBegin();
        for frame in &reloaded { assert_ne!(UQM_MeleePresentationImage((*frame).cast()), (*frame).cast()); }
    }
    for frame in reloaded { TFB_DrawImage_Delete(frame); }
    assert_eq!(UQM_GetMeleeRemasteredImageCount(), 0, "reloaded aliases are independently freed");
    assert_eq!(DELETED.lock().unwrap().len(), ALLOCATED.load(Ordering::Relaxed) - allocated_before,
        "accepted and rejected portrait images follow their original deferred deletion");
    src::uqm::globdata::GlobData.Game_state.CurrentActivity = 1;
} }
