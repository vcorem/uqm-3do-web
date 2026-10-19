//Copyright Paul Reiche, Fred Ford. 1992-2002

/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

// Rust translation of src/libs/graphics/cmap.c.
extern "C" {
    pub type string_table_entry;
    pub type NativePalette;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn AllocNativePalette() -> *mut NativePalette;
    fn FreeNativePalette(_: *mut NativePalette);
    fn SetNativePaletteColor(_: *mut NativePalette, index: ::core::ffi::c_int, _: Color);
    fn GetNativePaletteColor(_: *mut NativePalette, index: ::core::ffi::c_int) -> Color;
    fn GetTimeCounter() -> TimeCount;
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn exit(_: ::core::ffi::c_int) -> !;
    static mut QuitPosted: ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UBYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type COLORMAP = STRING;
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type TFB_ColorMap = tfb_colormap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub TaskControl: [XFORM_CONTROL; 16],
    pub Highest: ::core::ffi::c_int,
    pub Lock: Mutex,
}
pub type XFORM_CONTROL = xform_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_control {
    pub CMapIndex: ::core::ffi::c_int,
    pub CMapPtr: COLORMAPPTR,
    pub Ticks: SIZE,
    pub StartTime: DWORD,
    pub EndTime: DWORD,
    pub OldCMap: [Color; 256],
}
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
static mut XFormControl: C2Rust_Unnamed = C2Rust_Unnamed {
    TaskControl: [XFORM_CONTROL {
        CMapIndex: 0,
        CMapPtr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Ticks: 0,
        StartTime: 0,
        EndTime: 0,
        OldCMap: [Color { r: 0, g: 0, b: 0, a: 0 }; 256],
    }; 16],
    Highest: 0,
    Lock: ::core::ptr::null_mut::<::core::ffi::c_void>(),
};
static mut fadeAmount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
static mut fadeDelta: ::core::ffi::c_int = 0;
static mut fadeStartTime: TimeCount = 0;
static mut fadeInterval: sint32 = 0;
static mut fadeLock: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut poolhead: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
static mut poolcount: ::core::ffi::c_int = 0;
static mut colormaps: [*mut TFB_ColorMap; 250] = [::core::ptr::null_mut::<
    TFB_ColorMap,
>(); 250];
static mut mapcount: ::core::ffi::c_int = 0;
static mut maplock: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn InitColorMaps() {
    let mut i: ::core::ffi::c_int = 0;
    maplock = CreateMutex_Core(
        b"Colormaps Lock\0".as_ptr() as *const ::core::ffi::c_char,
        (C2Rust_Unnamed_0::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int) as DWORD,
    );
    ::core::ptr::write_volatile(&raw mut XFormControl.Highest, -1 as ::core::ffi::c_int);
    XFormControl.Lock = CreateMutex_Core(
        b"Transform Lock\0".as_ptr() as *const ::core::ffi::c_char,
        (C2Rust_Unnamed_0::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int) as DWORD,
    );
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        XFormControl.TaskControl[i as usize].CMapIndex = -1 as ::core::ffi::c_int;
        i += 1;
    }
    fadeLock = CreateMutex_Core(
        b"Fade Lock\0".as_ptr() as *const ::core::ffi::c_char,
        (C2Rust_Unnamed_0::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int) as DWORD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UninitColorMaps() {
    let mut i: ::core::ffi::c_int = 0;
    let mut next: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    i = 0 as ::core::ffi::c_int;
    while i < 250 as ::core::ffi::c_int {
        let mut map: *mut TFB_ColorMap = colormaps[i as usize];
        if !map.is_null() {
            release_colormap(map);
            colormaps[i as usize] = ::core::ptr::null_mut::<TFB_ColorMap>();
        }
        i += 1;
    }
    while !poolhead.is_null() {
        next = (*poolhead).next as *mut TFB_ColorMap;
        delete_colormap(poolhead);
        poolhead = next;
        poolcount -= 1;
    }
    DestroyMutex(fadeLock);
    DestroyMutex(XFormControl.Lock);
    DestroyMutex(maplock);
}
#[inline]
unsafe extern "C" fn alloc_colormap() -> *mut TFB_ColorMap {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    if !poolhead.is_null() {
        map = poolhead;
        poolhead = (*map).next as *mut TFB_ColorMap;
        poolcount -= 1;
    } else {
        map = HMalloc(::core::mem::size_of::<TFB_ColorMap>()) as *mut TFB_ColorMap;
        (*map).palette = AllocNativePalette();
        if (*map).palette.is_null() {
            HFree(map as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<TFB_ColorMap>();
        }
    }
    (*map).next = ::core::ptr::null_mut::<tfb_colormap>();
    (*map).index = -1 as ::core::ffi::c_int;
    (*map).refcount = 1 as ::core::ffi::c_int;
    (*map).version = 0 as ::core::ffi::c_int;
    return map;
}
unsafe extern "C" fn clone_colormap(
    mut from: *mut TFB_ColorMap,
    mut index: ::core::ffi::c_int,
) -> *mut TFB_ColorMap {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    map = alloc_colormap();
    if map.is_null() {
        log_add(
            log_Level::log_Warning,
            b"FATAL: clone_colormap(): could not allocate a map\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    } else {
        (*map).index = index;
        if !from.is_null() {
            (*map).version = (*from).version;
        }
    }
    (*map).version += 1;
    return map;
}
unsafe extern "C" fn delete_colormap(mut map: *mut TFB_ColorMap) {
    FreeNativePalette((*map).palette);
    HFree(map as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn free_colormap(mut map: *mut TFB_ColorMap) {
    if map.is_null() {
        log_add(
            log_Level::log_Warning,
            b"free_colormap(): tried to free a NULL map\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if poolcount < 20 as ::core::ffi::c_int {
        (*map).next = poolhead as *mut tfb_colormap;
        poolhead = map;
        poolcount += 1;
    } else {
        delete_colormap(map);
    };
}
#[inline]
unsafe extern "C" fn get_colormap(mut index: ::core::ffi::c_int) -> *mut TFB_ColorMap {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    map = colormaps[index as usize];
    if map.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"BUG: get_colormap(): map not present\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*map).refcount += 1;
    return map;
}
unsafe extern "C" fn release_colormap(mut map: *mut TFB_ColorMap) {
    if map.is_null() {
        return;
    }
    if (*map).refcount <= 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"BUG: release_colormap(): refcount not >0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*map).refcount -= 1;
    if (*map).refcount == 0 as ::core::ffi::c_int {
        free_colormap(map);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ReturnColorMap(mut map: *mut TFB_ColorMap) {
    LockMutex(maplock);
    release_colormap(map);
    UnlockMutex(maplock);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetColorMap(
    mut index: ::core::ffi::c_int,
) -> *mut TFB_ColorMap {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    LockMutex(maplock);
    map = get_colormap(index);
    UnlockMutex(maplock);
    return map;
}
#[no_mangle]
pub unsafe extern "C" fn GetColorMapColors(
    mut colors: *mut Color,
    mut map: *mut TFB_ColorMap,
) {
    let mut i: ::core::ffi::c_int = 0;
    if map.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        *colors.offset(i as isize) = GetNativePaletteColor((*map).palette, i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetColorMap(mut map: COLORMAPPTR) -> BOOLEAN {
    let mut start: ::core::ffi::c_int = 0;
    let mut end: ::core::ffi::c_int = 0;
    let mut total_size: ::core::ffi::c_int = 0;
    let mut colors: *mut UBYTE = map as *mut UBYTE;
    let mut mpp: *mut *mut TFB_ColorMap = ::core::ptr::null_mut::<*mut TFB_ColorMap>();
    if map.is_null() {
        return BOOLEAN::TRUE;
    }
    let c2rust_fresh0 = colors;
    colors = colors.offset(1);
    start = *c2rust_fresh0 as ::core::ffi::c_int;
    let c2rust_fresh1 = colors;
    colors = colors.offset(1);
    end = *c2rust_fresh1 as ::core::ffi::c_int;
    if start > end {
        log_add(
            log_Level::log_Warning,
            b"ERROR: SetColorMap(): starting map (%d) not less or eq ending (%d)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            start,
            end,
        );
        return BOOLEAN::FALSE;
    }
    if start >= 250 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"ERROR: SetColorMap(): starting map (%d) beyond range (0-%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            start,
            250 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
        return BOOLEAN::FALSE;
    }
    if end >= 250 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"SetColorMap(): ending map (%d) beyond range (0-%d)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            end,
            250 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
        end = 250 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    total_size = end + 1 as ::core::ffi::c_int;
    LockMutex(maplock);
    if total_size > mapcount {
        mapcount = total_size;
    }
    mpp = (&raw mut colormaps as *mut *mut TFB_ColorMap).offset(start as isize);
    while start <= end {
        let mut i: ::core::ffi::c_int = 0;
        let mut newmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
        let mut oldmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
        oldmap = *mpp;
        newmap = clone_colormap(oldmap, start);
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
            color.a = 0xff as BYTE;
            color.r = *colors.offset(0isize) as BYTE;
            color.g = *colors.offset(1isize) as BYTE;
            color.b = *colors.offset(2isize) as BYTE;
            SetNativePaletteColor((*newmap).palette, i, color);
            i += 1;
            colors = colors.offset(3 as ::core::ffi::c_int as isize);
        }
        *mpp = newmap;
        release_colormap(oldmap);
        start += 1;
        mpp = mpp.offset(1);
    }
    UnlockMutex(maplock);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn GetFadeAmount() -> ::core::ffi::c_int {
    let mut newAmount: ::core::ffi::c_int = 0;
    LockMutex(fadeLock);
    if fadeInterval != 0 {
        let mut Now: TimeCount = GetTimeCounter();
        let mut elapsed: sint32 = 0;
        elapsed = Now.wrapping_sub(fadeStartTime) as sint32;
        if elapsed > fadeInterval {
            elapsed = fadeInterval;
        }
        newAmount = (fadeAmount as ::core::ffi::c_long
            + fadeDelta as ::core::ffi::c_long * elapsed as ::core::ffi::c_long
                / fadeInterval as ::core::ffi::c_long) as ::core::ffi::c_int;
        if elapsed >= fadeInterval {
            fadeAmount = newAmount;
            fadeInterval = 0 as ::core::ffi::c_int as sint32;
        }
    } else {
        newAmount = fadeAmount;
    }
    UnlockMutex(fadeLock);
    return newAmount;
}
unsafe extern "C" fn finishPendingFade() {
    if fadeInterval != 0 {
        fadeAmount += fadeDelta;
        fadeInterval = 0 as ::core::ffi::c_int as sint32;
    }
}
unsafe extern "C" fn FlushFadeXForms() {
    LockMutex(fadeLock);
    finishPendingFade();
    UnlockMutex(fadeLock);
}
#[no_mangle]
pub unsafe extern "C" fn FadeScreen(
    mut fadeType: ScreenFadeType,
    mut TimeInterval: SIZE,
) -> DWORD {
    let mut TimeOut: TimeCount = 0;
    let mut FadeEnd: ::core::ffi::c_int = 0;
    match fadeType {
        ScreenFadeType::FadeAllToBlack | ScreenFadeType::FadeSomeToBlack => {
            FadeEnd = 0 as ::core::ffi::c_int;
        }
        ScreenFadeType::FadeAllToColor | ScreenFadeType::FadeSomeToColor => {
            FadeEnd = 255 as ::core::ffi::c_int;
        }
        ScreenFadeType::FadeAllToWhite | ScreenFadeType::FadeSomeToWhite => {
            FadeEnd = 510 as ::core::ffi::c_int;
        }
        _ => return GetTimeCounter(),
    }
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
        TimeInterval = 0 as SIZE;
    }
    LockMutex(fadeLock);
    finishPendingFade();
    if TimeInterval as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        || fadeAmount == FadeEnd
    {
        fadeAmount = FadeEnd;
        fadeInterval = 0 as ::core::ffi::c_int as sint32;
        TimeOut = GetTimeCounter();
    } else {
        fadeInterval = TimeInterval as sint32;
        fadeDelta = FadeEnd - fadeAmount;
        fadeStartTime = GetTimeCounter();
        TimeOut = fadeStartTime
            .wrapping_add(TimeInterval as TimeCount)
            .wrapping_add(1 as TimeCount);
    }
    UnlockMutex(fadeLock);
    return TimeOut;
}
unsafe extern "C" fn finish_colormap_xform(mut which: ::core::ffi::c_int) {
    SetColorMap(XFormControl.TaskControl[which as usize].CMapPtr);
    XFormControl.TaskControl[which as usize].CMapIndex = -1 as ::core::ffi::c_int;
    if which
        == ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest)
    {
        loop {
            which -= 1;
            if !(which >= 0 as ::core::ffi::c_int
                && XFormControl.TaskControl[which as usize].CMapIndex
                    == -1 as ::core::ffi::c_int)
            {
                break;
            }
        }
        ::core::ptr::write_volatile(&raw mut XFormControl.Highest, which);
    }
}
#[inline]
unsafe extern "C" fn blendChan(
    mut c1: BYTE,
    mut c2: BYTE,
    mut weight: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
) -> BYTE {
    return (c1 as ::core::ffi::c_int
        + (c2 as ::core::ffi::c_int - c1 as ::core::ffi::c_int) * weight / scale)
        as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn XFormColorMap_step() -> BOOLEAN {
    let mut Changed: BOOLEAN = BOOLEAN::FALSE;
    let mut x: ::core::ffi::c_int = 0;
    let mut Now: DWORD = GetTimeCounter();
    LockMutex(XFormControl.Lock);
    x = 0 as ::core::ffi::c_int;
    while x
        <= ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest)
    {
        let mut control: *mut XFORM_CONTROL = (&raw mut XFormControl.TaskControl
            as *mut XFORM_CONTROL)
            .offset(x as isize);
        let mut index: ::core::ffi::c_int = (*control).CMapIndex;
        let mut TicksLeft: ::core::ffi::c_int = (*control).EndTime.wrapping_sub(Now)
            as ::core::ffi::c_int;
        let mut curmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
        if index >= 0 as ::core::ffi::c_int {
            LockMutex(maplock);
            curmap = colormaps[index as usize];
            if curmap.is_null() {
                UnlockMutex(maplock);
                log_add(
                    log_Level::log_Error,
                    b"BUG: XFormColorMap_step(): no current map\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                finish_colormap_xform(x);
            } else {
                if TicksLeft > 0 as ::core::ffi::c_int {
                    let mut newmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<
                        TFB_ColorMap,
                    >();
                    let mut newClr: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
                    let mut oldClr: *mut Color = ::core::ptr::null_mut::<Color>();
                    let mut frac: ::core::ffi::c_int = 0;
                    let mut i: ::core::ffi::c_int = 0;
                    newmap = clone_colormap(curmap, index);
                    oldClr = &raw mut (*control).OldCMap as *mut Color;
                    newClr = ((*control).CMapPtr as *mut UBYTE)
                        .offset(2 as ::core::ffi::c_int as isize);
                    frac = ((*control).Ticks as ::core::ffi::c_int - TicksLeft)
                        * 0x10000 as ::core::ffi::c_int
                        / (*control).Ticks as ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < 256 as ::core::ffi::c_int {
                        let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
                        color.a = 0xff as BYTE;
                        color.r = blendChan(
                            (*oldClr).r,
                            *newClr.offset(0isize),
                            frac,
                            0x10000 as ::core::ffi::c_int,
                        );
                        color.g = blendChan(
                            (*oldClr).g,
                            *newClr.offset(1isize),
                            frac,
                            0x10000 as ::core::ffi::c_int,
                        );
                        color.b = blendChan(
                            (*oldClr).b,
                            *newClr.offset(2isize),
                            frac,
                            0x10000 as ::core::ffi::c_int,
                        );
                        SetNativePaletteColor((*newmap).palette, i, color);
                        i += 1;
                        oldClr = oldClr.offset(1);
                        newClr = newClr.offset(3 as ::core::ffi::c_int as isize);
                    }
                    colormaps[index as usize] = newmap;
                    release_colormap(curmap);
                }
                UnlockMutex(maplock);
                if TicksLeft <= 0 as ::core::ffi::c_int {
                    finish_colormap_xform(x);
                }
                Changed = BOOLEAN::TRUE;
            }
        }
        x += 1;
    }
    UnlockMutex(XFormControl.Lock);
    return Changed;
}
unsafe extern "C" fn FlushPLUTXForms() {
    let mut i: ::core::ffi::c_int = 0;
    LockMutex(XFormControl.Lock);
    i = 0 as ::core::ffi::c_int;
    while i
        <= ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest)
    {
        if XFormControl.TaskControl[i as usize].CMapIndex >= 0 as ::core::ffi::c_int {
            finish_colormap_xform(i);
        }
        i += 1;
    }
    ::core::ptr::write_volatile(&raw mut XFormControl.Highest, -1 as ::core::ffi::c_int);
    UnlockMutex(XFormControl.Lock);
}
unsafe extern "C" fn XFormPLUT(
    mut ColorMapPtr: COLORMAPPTR,
    mut TimeInterval: SIZE,
) -> DWORD {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    let mut control: *mut XFORM_CONTROL = ::core::ptr::null_mut::<XFORM_CONTROL>();
    let mut index: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut first_avail: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut EndTime: DWORD = 0;
    let mut Now: DWORD = 0;
    Now = GetTimeCounter() as DWORD;
    index = *(ColorMapPtr as *mut UBYTE) as ::core::ffi::c_int;
    LockMutex(XFormControl.Lock);
    x = 0 as ::core::ffi::c_int;
    while x
        <= ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest)
        && index != XFormControl.TaskControl[x as usize].CMapIndex
    {
        if first_avail == -1 as ::core::ffi::c_int
            && XFormControl.TaskControl[x as usize].CMapIndex == -1 as ::core::ffi::c_int
        {
            first_avail = x;
        }
        x += 1;
    }
    if index == XFormControl.TaskControl[x as usize].CMapIndex {
        finish_colormap_xform(x);
    } else if first_avail >= 0 as ::core::ffi::c_int {
        x = first_avail;
    } else if x >= 16 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Debug,
            b"WARNING: XFormPLUT(): no slots available\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        x = ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest);
        finish_colormap_xform(x);
    }
    control = (&raw mut XFormControl.TaskControl as *mut XFORM_CONTROL)
        .offset(x as isize);
    if x
        > ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const XFormControl.Highest)
    {
        ::core::ptr::write_volatile(&raw mut XFormControl.Highest, x);
    }
    LockMutex(maplock);
    map = colormaps[index as usize];
    if map.is_null() {
        UnlockMutex(maplock);
        UnlockMutex(XFormControl.Lock);
        log_add(
            log_Level::log_Warning,
            b"BUG: XFormPLUT(): no current map\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as DWORD;
    }
    GetColorMapColors(&raw mut (*control).OldCMap as *mut Color, map);
    UnlockMutex(maplock);
    (*control).CMapIndex = index;
    (*control).CMapPtr = ColorMapPtr;
    (*control).Ticks = TimeInterval;
    if ((*control).Ticks as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*control).Ticks = 0 as SIZE;
    }
    (*control).StartTime = Now;
    EndTime = Now.wrapping_add((*control).Ticks as DWORD);
    (*control).EndTime = EndTime;
    UnlockMutex(XFormControl.Lock);
    return EndTime;
}
#[no_mangle]
pub unsafe extern "C" fn XFormColorMap(
    mut ColorMapPtr: COLORMAPPTR,
    mut TimeInterval: SIZE,
) -> DWORD {
    if ColorMapPtr.is_null() {
        return 0 as DWORD;
    }
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
        TimeInterval = 0 as SIZE;
    }
    return XFormPLUT(ColorMapPtr, TimeInterval);
}
#[no_mangle]
pub unsafe extern "C" fn FlushColorXForms() {
    FlushFadeXForms();
    FlushPLUTXForms();
}
#[no_mangle]
pub unsafe extern "C" fn GetColorMapAddress(mut colormap: COLORMAP) -> COLORMAPPTR {
    return GetStringAddress(colormap) as COLORMAPPTR;
}
unsafe extern "C" fn DoTransformColorMap(
    mut colors: *mut Color,
    mut ColorMapPtr: COLORMAPPTR,
    mut from: COUNT,
    mut to: COUNT,
) {
    let mut map: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    let mut p_index: ::core::ffi::c_int = *(ColorMapPtr as *mut UBYTE)
        as ::core::ffi::c_int;
    LockMutex(maplock);
    map = colormaps[p_index as usize];
    if map.is_null() {
        UnlockMutex(maplock);
        log_add(
            log_Level::log_Error,
            b"BUG: XFormColorMap_step(): no current map\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut newmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    let mut newPtr: *mut UBYTE = (ColorMapPtr as *mut UBYTE)
        .offset(2 as ::core::ffi::c_int as isize);
    let mut c: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut i: ::core::ffi::c_int = 0;
    newmap = clone_colormap(map, p_index);
    c = colors;
    i = from as ::core::ffi::c_int;
    while i < to as ::core::ffi::c_int {
        *newPtr.offset(0isize) = (*c).r as UBYTE;
        *newPtr.offset(1isize) = (*c).g as UBYTE;
        *newPtr.offset(2isize) = (*c).b as UBYTE;
        SetNativePaletteColor((*newmap).palette, i, *c);
        i += 1;
        c = c.offset(1);
        newPtr = newPtr.offset(3 as ::core::ffi::c_int as isize);
    }
    colormaps[p_index as usize] = newmap;
    release_colormap(map);
    UnlockMutex(maplock);
}
#[no_mangle]
pub unsafe extern "C" fn SetColorMapColors(
    mut colors: *mut Color,
    mut ColorMapPtr: COLORMAPPTR,
    mut from: COUNT,
    mut to: COUNT,
) {
    if ColorMapPtr.is_null() {
        return;
    }
    DoTransformColorMap(colors, ColorMapPtr, from, to);
}
#[no_mangle]
pub unsafe extern "C" fn GetColorMapColor(
    mut ColorMapIndex: COUNT,
    mut ColorIndex: COUNT,
) -> Color {
    return GetNativePaletteColor(
        (*colormaps[ColorMapIndex as usize]).palette,
        ColorIndex as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetColorMapTableIndex(mut map: COLORMAP) -> UBYTE {
    let mut index: *mut UBYTE = GetColorMapAddress(map) as *mut UBYTE;
    return *index;
}
