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

// Rust translation of src/libs/graphics/gfxload.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_MountHandle;
    pub type uio_Repository;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type string_table;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_mountDir(
        destRep: *mut uio_Repository,
        mountPoint: *const ::core::ffi::c_char,
        fsType: uio_FileSystemID,
        sourceDir: *mut uio_DirHandle,
        sourcePath: *const ::core::ffi::c_char,
        inPath: *const ::core::ffi::c_char,
        autoMount_0: *mut *mut uio_AutoMount,
        flags: ::core::ffi::c_int,
        relative: *mut uio_MountHandle,
    ) -> *mut uio_MountHandle;
    fn uio_unmountDir(mountHandle: *mut uio_MountHandle) -> ::core::ffi::c_int;
    fn uio_openDir(
        repository_0: *mut uio_Repository,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_DirHandle;
    fn uio_openDirRelative(
        base: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_DirHandle;
    fn uio_closeDir(dirHandle: *mut uio_DirHandle) -> ::core::ffi::c_int;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_fgets(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        stream: *mut uio_Stream,
    ) -> *mut ::core::ffi::c_char;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn LoadDirEntryTable(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        matchType: match_MatchType,
    ) -> DIRENTRY_REF;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn AllocDrawable(num_frames: COUNT) -> DRAWABLE;
    fn TFB_DrawScreen_DeleteImage(img: *mut TFB_Image);
    fn TFB_DrawScreen_DeleteData(_: *mut ::core::ffi::c_void);
    fn TFB_DrawImage_New(canvas: TFB_Canvas) -> *mut TFB_Image;
    fn TFB_DrawCanvas_LoadFromFile(
        dir: *mut ::core::ffi::c_void,
        fileName: *const ::core::ffi::c_char,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_IsPaletted(canvas: TFB_Canvas) -> BOOLEAN;
    fn TFB_DrawCanvas_GetExtent(canvas: TFB_Canvas, size: *mut EXTENT);
    fn TFB_DrawCanvas_Delete(canvas: TFB_Canvas);
    fn TFB_DrawCanvas_GetFontCharData(
        canvas: TFB_Canvas,
        outData: *mut BYTE,
        dataPitch: ::core::ffi::c_uint,
    ) -> BOOLEAN;
    fn TFB_DrawCanvas_SetTransparentIndex(
        canvas: TFB_Canvas,
        i: ::core::ffi::c_int,
        rleaccel: BOOLEAN,
    );
    fn TFB_DrawCanvas_SetTransparentColor(
        canvas: TFB_Canvas,
        color: Color,
        rleaccel: BOOLEAN,
    );
    fn TFB_DrawCanvas_GetError() -> *const ::core::ffi::c_char;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut contentDir: *mut uio_DirHandle;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut repository: *mut uio_Repository;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const uio_MOUNT_BOTTOM: Self = Self(0);
    pub const uio_MOUNT_TOP: Self = Self(4);
    pub const uio_MOUNT_BELOW: Self = Self(8);
    pub const uio_MOUNT_ABOVE: Self = Self(12);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct match_MatchType(pub ::core::ffi::c_uint);
impl match_MatchType {
    pub const match_MATCH_LITERAL: Self = Self(0);
    pub const match_MATCH_PREFIX: Self = Self(1);
    pub const match_MATCH_SUFFIX: Self = Self(2);
    pub const match_MATCH_SUBSTRING: Self = Self(3);
    pub const match_MATCH_REGEX: Self = Self(4);
}
pub type uio_FileSystemID = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_AutoMount {
    pub pattern: *const ::core::ffi::c_char,
    pub matchType: match_MatchType,
    pub fileSystemID: uio_FileSystemID,
    pub mountFlags: ::core::ffi::c_int,
}
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_desc {
    pub Type: DRAWABLE_TYPE,
    pub Index: UWORD,
    pub HotSpot: HOT_SPOT,
    pub Bounds: EXTENT,
    pub image: *mut TFB_Image,
    pub parent: *mut drawable_desc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawable_desc {
    pub Flags: CREATE_FLAGS,
    pub MaxIndex: UWORD,
    pub Frame: *mut FRAME_DESC,
}
pub type FRAME_DESC = frame_desc;
pub type CREATE_FLAGS = BYTE;
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type HOT_SPOT = POINT;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type DRAWABLE_TYPE = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_desc {
    pub Leading: BYTE,
    pub fontPages: *mut FONT_PAGE,
    pub disp: EXTENT,
    pub filename: [::core::ffi::c_char; 4096],
    pub CharSpace: BYTE,
    pub KernAmount: BYTE,
    pub KernTab: [BYTE; 65535],
    pub VertAlign: SBYTE,
    pub HaveFntData: BOOLEAN,
}
pub type FONT_PAGE = FontPage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontPage {
    pub next: *mut FontPage,
    pub pageStart: UniChar,
    pub firstChar: UniChar,
    pub numChars: size_t,
    pub charDesc: *mut TFB_Char,
}
pub type TFB_Char = tfb_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_char {
    pub extent: EXTENT,
    pub disp: EXTENT,
    pub HotSpot: HOT_SPOT,
    pub data: *mut BYTE,
    pub pitch: DWORD,
}
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type DIRENTRY_REF = STRING_TABLE;
pub type DIRENTRY = STRING;
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
pub type AniData = anidata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anidata {
    pub transparent_color: ::core::ffi::c_int,
    pub colormap_index: ::core::ffi::c_int,
    pub hotspot_x: ::core::ffi::c_int,
    pub hotspot_y: ::core::ffi::c_int,
    // Original resolved PNG path; never substituted in the collision drawable.
    pub render_source: [::core::ffi::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BuildCharDesc {
    pub canvas: TFB_Canvas,
    pub index: UniChar,
}
#[inline]
unsafe extern "C" fn AllocFontPage(mut numChars: ::core::ffi::c_int) -> *mut FONT_PAGE {
    let mut result: *mut FONT_PAGE = HMalloc(
        ::core::mem::size_of::<FONT_PAGE>() as size_t,
    ) as *mut FONT_PAGE;
    (*result).charDesc = HCalloc(
        (numChars as size_t).wrapping_mul(::core::mem::size_of::<TFB_Char>() as size_t),
    ) as *mut TFB_Char;
    return result;
}
#[inline]
unsafe extern "C" fn FreeFontPage(mut page: *mut FONT_PAGE) {
    HFree((*page).charDesc as *mut ::core::ffi::c_void);
    HFree(page as *mut ::core::ffi::c_void);
}
static mut autoMount: [*mut uio_AutoMount; 1] = [
    ::core::ptr::null_mut::<uio_AutoMount>(),
];
unsafe extern "C" fn process_image(
    mut FramePtr: FRAME,
    mut img: *mut TFB_Canvas,
    mut ani: *mut AniData,
    mut cel_ct: ::core::ffi::c_int,
) {
    let mut tfbimg: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut hx: ::core::ffi::c_int = 0;
    let mut hy: ::core::ffi::c_int = 0;
    (*FramePtr).Type = 0 as DRAWABLE_TYPE;
    (*FramePtr).Index = cel_ct as UWORD;
    if TFB_DrawCanvas_IsPaletted(*img.offset(cel_ct as isize)).0 != 0 {
        if (*ani.offset(cel_ct as isize)).transparent_color >= 0 as ::core::ffi::c_int {
            TFB_DrawCanvas_SetTransparentIndex(
                *img.offset(cel_ct as isize),
                (*ani.offset(cel_ct as isize)).transparent_color,
                BOOLEAN::FALSE,
            );
        }
    } else if (*ani.offset(cel_ct as isize)).transparent_color == 0 as ::core::ffi::c_int
    {
        let mut color: Color = Color {
            r: 0 as BYTE,
            g: 0 as BYTE,
            b: 0 as BYTE,
            a: 0 as BYTE,
        };
        TFB_DrawCanvas_SetTransparentColor(
            *img.offset(cel_ct as isize),
            color,
            BOOLEAN::FALSE,
        );
    }
    if (*ani.offset(cel_ct as isize)).transparent_color == -1 as ::core::ffi::c_int {
        TFB_DrawCanvas_SetTransparentIndex(
            *img.offset(cel_ct as isize),
            -1 as ::core::ffi::c_int,
            BOOLEAN::FALSE,
        );
    }
    hx = (*ani.offset(cel_ct as isize)).hotspot_x;
    hy = (*ani.offset(cel_ct as isize)).hotspot_y;
    (*FramePtr).image = TFB_DrawImage_New(*img.offset(cel_ct as isize));
    tfbimg = (*FramePtr).image;
    (*tfbimg).colormap_index = (*ani.offset(cel_ct as isize)).colormap_index;
    *img.offset(cel_ct as isize) = (*tfbimg).NormalImg;
    (*FramePtr).HotSpot = MAKE_HOT_SPOT(hx as COORD, hy as COORD);
    (*FramePtr).Bounds.width = (*tfbimg).extent.width;
    (*FramePtr).Bounds.height = (*tfbimg).extent.height;
    crate::uqm_melee_presentation::UQM_MeleePresentationRegister(
        tfbimg.cast(), (*ani.offset(cel_ct as isize)).render_source.as_ptr(),
        _cur_resfile_name, hx as COORD, hy as COORD,
    );
}
unsafe extern "C" fn processFontChar(
    mut CharPtr: *mut TFB_Char,
    mut canvas: TFB_Canvas,
    mut fontPtr: FONT,
) {
    let mut newdata: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    let mut dpitch: size_t = 0;
    TFB_DrawCanvas_GetExtent(canvas, &raw mut (*CharPtr).extent);
    dpitch = (*CharPtr).extent.width as size_t;
    newdata = HMalloc(
        dpitch
            .wrapping_mul((*CharPtr).extent.height as size_t)
            .wrapping_mul(::core::mem::size_of::<BYTE>() as size_t),
    ) as *mut BYTE;
    uqm_lua_checked!(TFB_DrawCanvas_GetFontCharData(canvas, newdata, dpitch as ::core::ffi::c_uint), ());
    (*CharPtr).data = newdata;
    (*CharPtr).pitch = dpitch as DWORD;
    (*CharPtr).disp.width = (*CharPtr).extent.width;
    (*CharPtr).disp.height = (*CharPtr).extent.height;
    (*CharPtr).HotSpot.x = 0 as COORD;
    let mut tune_amount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*CharPtr).disp.height as ::core::ffi::c_int
        <= (8 as ::core::ffi::c_int) << resolutionFactor
    {
        tune_amount = (1 as ::core::ffi::c_int) << resolutionFactor;
    } else if (*CharPtr).disp.height as ::core::ffi::c_int
        == (9 as ::core::ffi::c_int) << resolutionFactor
    {
        tune_amount = (2 as ::core::ffi::c_int) << resolutionFactor;
    } else if (*CharPtr).disp.height as ::core::ffi::c_int
        > (9 as ::core::ffi::c_int) << resolutionFactor
    {
        tune_amount = (3 as ::core::ffi::c_int) << resolutionFactor;
    }
    if (*fontPtr).HaveFntData.0 == 0 {
        (*fontPtr).VertAlign = tune_amount as SBYTE;
    }
    (*CharPtr).HotSpot.y = ((*CharPtr).disp.height as ::core::ffi::c_int
        - (*fontPtr).VertAlign as ::core::ffi::c_int) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn _GetCelData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut cel_total: ::core::ffi::c_int = 0;
    let mut cel_index: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut opos: DWORD = 0;
    let mut CurrentLine: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut filename: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut img: *mut TFB_Canvas = ::core::ptr::null_mut::<TFB_Canvas>();
    let mut ani: *mut AniData = ::core::ptr::null_mut::<AniData>();
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut aniMount: *mut uio_MountHandle = ::core::ptr::null_mut::<uio_MountHandle>();
    let mut aniDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut aniFile: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    opos = uqm_lua_checked!(uio_ftell(fp), ::core::ptr::null_mut()) as DWORD;
    let mut s1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut s2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut aniDirName: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut aniFileName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut buf: [uint8; 4] = [0 as uint8, 0 as uint8, 0 as uint8, 0 as uint8];
    let mut header: uint32 = 0;
    if _cur_resfile_name.is_null()
        || {
            s2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            s1 = strrchr(_cur_resfile_name, '/' as ::core::ffi::c_int);
            s1.is_null() as ::core::ffi::c_int != 0
                && {
                    s2 = strrchr(_cur_resfile_name, '\\' as ::core::ffi::c_int);
                    s2.is_null()
                }
        }
    {
        n = 0 as ::core::ffi::c_int;
    } else {
        if s2 > s1 {
            s1 = s2;
        }
        n = (s1.offset_from(_cur_resfile_name) + 1isize) as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_fread(
        &raw mut buf as *mut uint8 as *mut ::core::ffi::c_void,
        4 as size_t,
        1 as size_t,
        fp,
    ), ::core::ptr::null_mut());
    header = (buf[0usize] as ::core::ffi::c_int
        | (buf[1usize] as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (buf[2usize] as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
        | (buf[3usize] as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32;
    if !_cur_resfile_name.is_null()
        && header == 0x4034b50 as ::core::ffi::c_int as uint32
    {
        if n != 0 {
            strncpy(
                &raw mut aniDirName as *mut ::core::ffi::c_char,
                _cur_resfile_name,
                (n - 1 as ::core::ffi::c_int) as size_t,
            );
            aniDirName[(n - 1 as ::core::ffi::c_int) as usize] = 0
                as ::core::ffi::c_char;
            aniFileName = _cur_resfile_name.offset(n as isize);
        } else {
            strcpy(
                &raw mut aniDirName as *mut ::core::ffi::c_char,
                b".\0".as_ptr() as *const ::core::ffi::c_char,
            );
            aniFileName = _cur_resfile_name;
        }
        aniDir = uqm_lua_checked!(uio_openDir(
            repository,
            &raw mut aniDirName as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ), ::core::ptr::null_mut());
        aniMount = uqm_lua_checked!(uio_mountDir(
            repository,
            &raw mut aniDirName as *mut ::core::ffi::c_char,
            2 as uio_FileSystemID,
            aniDir,
            aniFileName,
            b"/\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut autoMount as *mut *mut uio_AutoMount,
            (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | C2Rust_Unnamed::uio_MOUNT_TOP.0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<uio_MountHandle>(),
        ), ::core::ptr::null_mut());
        aniFile = uqm_lua_checked!(uio_fopen(
            aniDir,
            aniFileName,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ), ::core::ptr::null_mut());
        opos = 0 as DWORD;
        n = 0 as ::core::ffi::c_int;
    } else {
        strncpy(
            &raw mut filename as *mut ::core::ffi::c_char,
            _cur_resfile_name,
            n as size_t,
        );
        aniFile = fp;
        aniDir = contentDir;
    }
    cel_total = 0 as ::core::ffi::c_int;
    uqm_lua_checked!(uio_fseek(aniFile, opos as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
    while !uqm_lua_checked!(uio_fgets(
            &raw mut CurrentLine as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            aniFile,
        ), ::core::ptr::null_mut())
        .is_null()
    {
        cel_total += 1;
    }
    img = HMalloc(
        (::core::mem::size_of::<TFB_Canvas>() as size_t)
            .wrapping_mul(cel_total as size_t),
    ) as *mut TFB_Canvas;
    ani = HMalloc(
        (::core::mem::size_of::<AniData>() as size_t).wrapping_mul(cel_total as size_t),
    ) as *mut AniData;
    if img.is_null() || ani.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Couldn't allocate space for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            _cur_resfile_name,
        );
        if !aniMount.is_null() {
            uqm_lua_checked!(uio_fclose(aniFile), ::core::ptr::null_mut());
            uio_closeDir(aniDir);
            uqm_lua_checked!(uio_unmountDir(aniMount), ::core::ptr::null_mut());
        }
        HFree(img as *mut ::core::ffi::c_void);
        HFree(ani as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    cel_index = 0 as ::core::ffi::c_int;
    uqm_lua_checked!(uio_fseek(aniFile, opos as ::core::ffi::c_long, 0 as ::core::ffi::c_int), ::core::ptr::null_mut());
    while !uqm_lua_checked!(uio_fgets(
            &raw mut CurrentLine as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            aniFile,
        ), ::core::ptr::null_mut())
        .is_null() && cel_index < cel_total
    {
        if sscanf(
            &raw mut CurrentLine as *mut ::core::ffi::c_char,
            b"%s %d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
            (&raw mut filename as *mut ::core::ffi::c_char).offset(n as isize),
            &raw mut (*ani.offset(cel_index as isize)).transparent_color,
            &raw mut (*ani.offset(cel_index as isize)).colormap_index,
            &raw mut (*ani.offset(cel_index as isize)).hotspot_x,
            &raw mut (*ani.offset(cel_index as isize)).hotspot_y,
        ) != 5 as ::core::ffi::c_int
        {
            break;
        }
        strncpy(
            (*ani.offset(cel_index as isize)).render_source.as_mut_ptr(),
            filename.as_ptr(), 4095,
        );
        (*ani.offset(cel_index as isize)).render_source[4095] = 0;
        *img.offset(cel_index as isize) = uqm_lua_checked!(TFB_DrawCanvas_LoadFromFile(
            aniDir as *mut ::core::ffi::c_void,
            &raw mut filename as *mut ::core::ffi::c_char,
        ), ::core::ptr::null_mut());
        if (*img.offset(cel_index as isize)).is_null() {
            let mut err: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            err = TFB_DrawCanvas_GetError();
            log_add(
                log_Level::log_Warning,
                b"_GetCelData: Unable to load image!\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !err.is_null() {
                log_add(
                    log_Level::log_Warning,
                    b"Gfx Driver reports: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    err,
                );
            }
        } else {
            cel_index += 1;
        }
        if uqm_lua_checked!(uio_ftell(aniFile), ::core::ptr::null_mut()) as ::core::ffi::c_int - opos as ::core::ffi::c_int
            >= length as ::core::ffi::c_int
        {
            break;
        }
    }
    Drawable = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    if cel_index != 0
        && {
            Drawable = AllocDrawable(cel_index as COUNT);
            !Drawable.is_null()
        }
    {
        if Drawable.is_null() {
            loop {
                let c2rust_fresh0 = cel_index;
                cel_index -= 1;
                if c2rust_fresh0 == 0 {
                    break;
                }
                TFB_DrawCanvas_Delete(*img.offset(cel_index as isize));
            }
            HFree(Drawable as *mut ::core::ffi::c_void);
            Drawable = ::core::ptr::null_mut::<DRAWABLE_DESC>();
        } else {
            let mut FramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            (*Drawable).Flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as CREATE_FLAGS;
            (*Drawable).MaxIndex = (cel_index - 1 as ::core::ffi::c_int) as UWORD;
            FramePtr = (*Drawable).Frame.offset(cel_index as isize) as FRAME;
            loop {
                FramePtr = FramePtr.offset(-1);
                let c2rust_fresh1 = cel_index;
                cel_index -= 1;
                if c2rust_fresh1 == 0 {
                    break;
                }
                process_image(FramePtr, img, ani, cel_index);
            }
        }
    }
    if Drawable.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Couldn't get cel data for '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            _cur_resfile_name,
        );
    }
    if !aniMount.is_null() {
        uqm_lua_checked!(uio_fclose(aniFile), ::core::ptr::null_mut());
        uio_closeDir(aniDir);
        uqm_lua_checked!(uio_unmountDir(aniMount), ::core::ptr::null_mut());
    }
    HFree(img as *mut ::core::ffi::c_void);
    HFree(ani as *mut ::core::ffi::c_void);
    return Drawable as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _ReleaseCelData(
    mut handle: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    let mut DrawablePtr: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut cel_ct: ::core::ffi::c_int = 0;
    let mut FramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    DrawablePtr = handle as DRAWABLE;
    if DrawablePtr.is_null() {
        return BOOLEAN::FALSE;
    }
    cel_ct = (*DrawablePtr).MaxIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    FramePtr = (*DrawablePtr).Frame as FRAME;
    HFree(handle);
    if !FramePtr.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < cel_ct {
            let mut img: *mut TFB_Image = (*FramePtr.offset(i as isize)).image;
            if !img.is_null() {
                (*FramePtr.offset(i as isize)).image = ::core::ptr::null_mut::<
                    TFB_Image,
                >();
                TFB_DrawScreen_DeleteImage(img);
            }
            i += 1;
        }
        HFree(FramePtr as *mut ::core::ffi::c_void);
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn compareBCDIndex(
    mut arg1: *const ::core::ffi::c_void,
    mut arg2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut bcd1: *const BuildCharDesc = arg1 as *const BuildCharDesc;
    let mut bcd2: *const BuildCharDesc = arg2 as *const BuildCharDesc;
    return (*bcd1).index as ::core::ffi::c_int - (*bcd2).index as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _GetFontData(
    mut fp: *mut uio_Stream,
    mut length: DWORD,
) -> *mut ::core::ffi::c_void {
    let mut numDirEntries: COUNT = 0;
    let mut fontDir: DIRENTRY = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    let mut bcds: *mut BuildCharDesc = ::core::ptr::null_mut::<BuildCharDesc>();
    let mut numBCDs: size_t = 0 as size_t;
    let mut dirEntryI: ::core::ffi::c_int = 0;
    let mut fontDirHandle: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut fontMount: *mut uio_MountHandle = ::core::ptr::null_mut::<uio_MountHandle>();
    let mut fontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut cfgFile: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut cfg_name: *const ::core::ffi::c_char = b"kerndat.fnt\0".as_ptr()
        as *const ::core::ffi::c_char;
    if !_cur_resfile_name.is_null() {
        if fp
            != ::core::ptr::with_exposed_provenance_mut::<
                ::core::ffi::c_void,
            >(!(0 as ::core::ffi::c_int) as usize) as *mut uio_Stream
        {
            let mut s1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut s2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut n: ::core::ffi::c_int = 0;
            let mut fontZipName: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut fontDirName: [::core::ffi::c_char; 4096] = [0; 4096];
            s2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            s1 = strrchr(_cur_resfile_name, '/' as ::core::ffi::c_int);
            if s1.is_null() as ::core::ffi::c_int != 0
                && {
                    s2 = strrchr(_cur_resfile_name, '\\' as ::core::ffi::c_int);
                    s2.is_null()
                }
            {
                strcpy(
                    &raw mut fontDirName as *mut ::core::ffi::c_char,
                    b".\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fontZipName = _cur_resfile_name;
            } else {
                if s2 > s1 {
                    s1 = s2;
                }
                n = (s1.offset_from(_cur_resfile_name) + 1isize) as ::core::ffi::c_int;
                strncpy(
                    &raw mut fontDirName as *mut ::core::ffi::c_char,
                    _cur_resfile_name,
                    (n - 1 as ::core::ffi::c_int) as size_t,
                );
                fontDirName[(n - 1 as ::core::ffi::c_int) as usize] = 0
                    as ::core::ffi::c_char;
                fontZipName = _cur_resfile_name.offset(n as isize);
            }
            fontDirHandle = uqm_lua_checked!(uio_openDir(
                repository,
                &raw mut fontDirName as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            ), ::core::ptr::null_mut());
            fontMount = uqm_lua_checked!(uio_mountDir(
                repository,
                _cur_resfile_name,
                2 as uio_FileSystemID,
                fontDirHandle,
                fontZipName,
                b"/\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut autoMount as *mut *mut uio_AutoMount,
                (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | C2Rust_Unnamed::uio_MOUNT_TOP.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<uio_MountHandle>(),
            ), ::core::ptr::null_mut());
            uio_closeDir(fontDirHandle);
        }
        fontDir = CaptureStringTable(
            uqm_lua_checked!(LoadDirEntryTable(
                contentDir,
                _cur_resfile_name,
                b".\0".as_ptr() as *const ::core::ffi::c_char,
                match_MatchType::match_MATCH_SUBSTRING,
            ), ::core::ptr::null_mut()),
        ) as DIRENTRY;
        if !fontDir.is_null() {
            numDirEntries = GetStringTableCount(fontDir);
            fontDirHandle = uqm_lua_checked!(uio_openDirRelative(
                contentDir,
                _cur_resfile_name,
                0 as ::core::ffi::c_int,
            ), ::core::ptr::null_mut());
            if !fontDirHandle.is_null() {
                bcds = HMalloc(
                    (numDirEntries as size_t)
                        .wrapping_mul(::core::mem::size_of::<BuildCharDesc>() as size_t),
                ) as *mut BuildCharDesc;
                if !bcds.is_null() {
                    dirEntryI = 0 as ::core::ffi::c_int;
                    while dirEntryI < numDirEntries as ::core::ffi::c_int {
                        let mut char_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut charIndex: ::core::ffi::c_uint = 0;
                        let mut canvas: TFB_Canvas = ::core::ptr::null_mut::<
                            ::core::ffi::c_void,
                        >();
                        let mut size: EXTENT = EXTENT { width: 0, height: 0 };
                        char_name = GetStringAddress(
                            SetAbsStringTableIndex(fontDir, dirEntryI as COUNT),
                        ) as *mut ::core::ffi::c_char;
                        if strcmp(char_name, cfg_name) != 0 as ::core::ffi::c_int {
                            if sscanf(
                                char_name,
                                b"%x.\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut charIndex,
                            ) == 1 as ::core::ffi::c_int
                            {
                                if charIndex <= 0xffff as ::core::ffi::c_uint {
                                    canvas = uqm_lua_checked!(TFB_DrawCanvas_LoadFromFile(
                                        fontDirHandle as *mut ::core::ffi::c_void,
                                        char_name,
                                    ), ::core::ptr::null_mut());
                                    if !canvas.is_null() {
                                        TFB_DrawCanvas_GetExtent(canvas, &raw mut size);
                                        if size.width as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                            || size.height as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                        {
                                            TFB_DrawCanvas_Delete(canvas);
                                        } else {
                                            (*bcds.offset(numBCDs as isize)).canvas = canvas;
                                            (*bcds.offset(numBCDs as isize)).index = charIndex
                                                as UniChar;
                                            numBCDs = numBCDs.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                        }
                        dirEntryI += 1;
                    }
                    fontPtr = HCalloc(
                        (::core::mem::size_of::<FONT_DESC>() as size_t)
                            .wrapping_add(0 as size_t),
                    ) as FONT;
                    if !fontPtr.is_null() {
                        cfgFile = uqm_lua_checked!(uio_fopen(
                            fontDirHandle,
                            cfg_name,
                            b"r\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ::core::ptr::null_mut());
                        if !cfgFile.is_null() {
                            let mut CurrentLine: [::core::ffi::c_char; 4096] = [0; 4096];
                            let mut cel_index: ::core::ffi::c_int = 0
                                as ::core::ffi::c_int;
                            let mut cel_total: ::core::ffi::c_int = 0
                                as ::core::ffi::c_int;
                            let mut opos: DWORD = 0 as DWORD;
                            memset(
                                &raw mut (*fontPtr).KernTab as *mut ::core::ffi::c_void,
                                !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int,
                                ::core::mem::size_of::<[BYTE; 65535]>() as size_t,
                            );
                            uqm_lua_checked!(uio_fseek(
                                cfgFile,
                                opos as ::core::ffi::c_long,
                                0 as ::core::ffi::c_int,
                            ), ::core::ptr::null_mut());
                            while !uqm_lua_checked!(uio_fgets(
                                    &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                                        as ::core::ffi::c_int,
                                    cfgFile,
                                ), ::core::ptr::null_mut())
                                .is_null()
                            {
                                cel_total += 1;
                                if cel_total == 0xffff as ::core::ffi::c_int {
                                    break;
                                }
                            }
                            uqm_lua_checked!(uio_fseek(
                                cfgFile,
                                opos as ::core::ffi::c_long,
                                0 as ::core::ffi::c_int,
                            ), ::core::ptr::null_mut());
                            while !uqm_lua_checked!(uio_fgets(
                                    &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                                        as ::core::ffi::c_int,
                                    cfgFile,
                                ), ::core::ptr::null_mut())
                                .is_null() && cel_index < cel_total
                            {
                                if cel_index > 0 as ::core::ffi::c_int {
                                    let mut KernChar: SDWORD = 0;
                                    let mut kernLBits: SDWORD = 0;
                                    let mut kernRBits: SDWORD = 0;
                                    if sscanf(
                                        &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                        b"%x %u %u\0".as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut KernChar,
                                        &raw mut kernLBits,
                                        &raw mut kernRBits,
                                    ) == 3 as ::core::ffi::c_int
                                    {
                                        if kernLBits > 3 as ::core::ffi::c_int
                                            || kernLBits < 0 as ::core::ffi::c_int
                                        {
                                            kernLBits = 3 as ::core::ffi::c_int as SDWORD;
                                        }
                                        if kernRBits > 3 as ::core::ffi::c_int
                                            || kernRBits < 0 as ::core::ffi::c_int
                                        {
                                            kernRBits = 3 as ::core::ffi::c_int as SDWORD;
                                        }
                                        (*fontPtr).KernTab[KernChar as usize] = (kernLBits
                                            << 2 as ::core::ffi::c_int | kernRBits) as BYTE;
                                    }
                                } else {
                                    if sscanf(
                                        &raw mut CurrentLine as *mut ::core::ffi::c_char,
                                        b"%s %hhu %hhu %hhu %hhd\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut (*fontPtr).filename as *mut ::core::ffi::c_char,
                                        &raw mut (*fontPtr).Leading,
                                        &raw mut (*fontPtr).CharSpace,
                                        &raw mut (*fontPtr).KernAmount,
                                        &raw mut (*fontPtr).VertAlign,
                                    ) != 5 as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                    (*fontPtr).HaveFntData = BOOLEAN::TRUE;
                                }
                                cel_index += 1;
                                if cel_index == 0xffff as ::core::ffi::c_int {
                                    break;
                                }
                                if uqm_lua_checked!(uio_ftell(cfgFile), ::core::ptr::null_mut()) as ::core::ffi::c_int
                                    - opos as ::core::ffi::c_int
                                    >= uqm_lua_checked!(LengthResFile(cfgFile), ::core::ptr::null_mut()) as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            uqm_lua_checked!(uio_fclose(cfgFile), ::core::ptr::null_mut());
                        }
                        uio_closeDir(fontDirHandle);
                        DestroyStringTable(ReleaseStringTable(fontDir));
                        if !fontMount.is_null() {
                            uqm_lua_checked!(uio_unmountDir(fontMount), ::core::ptr::null_mut());
                        }
                        qsort(
                            bcds as *mut ::core::ffi::c_void,
                            numBCDs,
                            ::core::mem::size_of::<BuildCharDesc>() as size_t,
                            Some(
                                compareBCDIndex
                                    as unsafe extern "C" fn(
                                        *const ::core::ffi::c_void,
                                        *const ::core::ffi::c_void,
                                    ) -> ::core::ffi::c_int,
                            ),
                        );
                        let mut startBCD: size_t = 0 as size_t;
                        let mut pageStart: UniChar = 0;
                        let mut pageEndPtr: *mut *mut FONT_PAGE = &raw mut (*fontPtr)
                            .fontPages;
                        while startBCD < numBCDs {
                            let mut endBCD: size_t = 0;
                            pageStart = ((*bcds.offset(startBCD as isize)).index
                                & 0xfffff800 as ::core::ffi::c_uint) as UniChar;
                            endBCD = startBCD;
                            while endBCD < numBCDs
                                && (*bcds.offset(endBCD as isize)).index
                                    & 0xfffff800 as ::core::ffi::c_uint == pageStart
                            {
                                endBCD = endBCD.wrapping_add(1);
                            }
                            let mut bcdI: size_t = 0;
                            let mut numChars: ::core::ffi::c_int = (*bcds
                                .offset(endBCD.wrapping_sub(1 as size_t) as isize))
                                .index
                                .wrapping_add(1 as UniChar)
                                .wrapping_sub((*bcds.offset(startBCD as isize)).index)
                                as ::core::ffi::c_int;
                            let mut page: *mut FONT_PAGE = AllocFontPage(numChars);
                            (*page).pageStart = pageStart;
                            (*page).firstChar = (*bcds.offset(startBCD as isize)).index;
                            (*page).numChars = numChars as size_t;
                            *pageEndPtr = page;
                            pageEndPtr = &raw mut (*page).next as *mut *mut FONT_PAGE;
                            bcdI = startBCD;
                            while bcdI < endBCD {
                                let mut bcd: *mut BuildCharDesc = bcds
                                    .offset(bcdI as isize);
                                let mut destChar: *mut TFB_Char = (*page)
                                    .charDesc
                                    .offset(
                                        (*bcd).index.wrapping_sub((*page).firstChar) as isize,
                                    );
                                if !(*destChar).data.is_null() {
                                    log_add(
                                        log_Level::log_Debug,
                                        b"Duplicate image for character %d for font %s.\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*bcd).index as ::core::ffi::c_int,
                                        _cur_resfile_name,
                                    );
                                    TFB_DrawCanvas_Delete((*bcd).canvas);
                                } else {
                                    uqm_lua_checked!(processFontChar(destChar, (*bcd).canvas, fontPtr), ::core::ptr::null_mut());
                                    TFB_DrawCanvas_Delete((*bcd).canvas);
                                    if (*destChar).disp.height as ::core::ffi::c_int
                                        > (*fontPtr).disp.height as ::core::ffi::c_int
                                    {
                                        (*fontPtr).disp.height = (*destChar).disp.height;
                                    }
                                    if (*destChar).disp.width as ::core::ffi::c_int
                                        > (*fontPtr).disp.width as ::core::ffi::c_int
                                    {
                                        (*fontPtr).disp.width = (*destChar).disp.width;
                                    }
                                }
                                bcdI = bcdI.wrapping_add(1);
                            }
                            startBCD = endBCD;
                        }
                        *pageEndPtr = ::core::ptr::null_mut::<FONT_PAGE>();
                        if (*fontPtr).HaveFntData.0 == 0 {
                            (*fontPtr).Leading = ((*fontPtr).disp.height
                                as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
                            (*fontPtr).CharSpace = ((1 as ::core::ffi::c_int)
                                << resolutionFactor) as BYTE;
                        }
                        HFree(bcds as *mut ::core::ffi::c_void);
                        return fontPtr as *mut ::core::ffi::c_void;
                    }
                }
            }
        }
    }
    if !fontPtr.is_null() {
        HFree(fontPtr as *mut ::core::ffi::c_void);
    }
    if !bcds.is_null() {
        let mut bcdI_0: size_t = 0;
        bcdI_0 = 0 as size_t;
        while bcdI_0 < numBCDs {
            TFB_DrawCanvas_Delete((*bcds.offset(bcdI_0 as isize)).canvas);
            bcdI_0 = bcdI_0.wrapping_add(1);
        }
        HFree(bcds as *mut ::core::ffi::c_void);
    }
    if !fontDirHandle.is_null() {
        uio_closeDir(fontDirHandle);
    }
    if !fontDir.is_null() {
        DestroyStringTable(ReleaseStringTable(fontDir));
    }
    if !fontMount.is_null() {
        uqm_lua_checked!(uio_unmountDir(fontMount), ::core::ptr::null_mut());
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn _ReleaseFontData(
    mut handle: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    let mut font: FONT = handle as FONT;
    if font.is_null() {
        return BOOLEAN::FALSE;
    }
    let mut page: *mut FONT_PAGE = ::core::ptr::null_mut::<FONT_PAGE>();
    let mut nextPage: *mut FONT_PAGE = ::core::ptr::null_mut::<FONT_PAGE>();
    page = (*font).fontPages;
    while !page.is_null() {
        let mut charI: size_t = 0;
        charI = 0 as size_t;
        while charI < (*page).numChars {
            let mut c: *mut TFB_Char = (*page).charDesc.offset(charI as isize);
            if !(*c).data.is_null() {
                TFB_DrawScreen_DeleteData((*c).data as *mut ::core::ffi::c_void);
            }
            charI = charI.wrapping_add(1);
        }
        nextPage = (*page).next as *mut FONT_PAGE;
        FreeFontPage(page);
        page = nextPage;
    }
    HFree(font as *mut ::core::ffi::c_void);
    return BOOLEAN::TRUE;
}
