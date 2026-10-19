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

// Rust translation of src/libs/graphics/resgfx.c.
extern "C" {
    pub type uio_Stream;
    fn LoadResourceFromPath(
        pathname: *const ::core::ffi::c_char,
        r#fn: Option<ResourceLoadFileFun>,
    ) -> *mut ::core::ffi::c_void;
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn _GetCelData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _ReleaseCelData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn _GetFontData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _ReleaseFontData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
}
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub union RESOURCE_DATA {
    pub num: DWORD,
    pub ptr: *mut ::core::ffi::c_void,
    pub str: *const ::core::ffi::c_char,
}
pub type ResourceLoadFun = unsafe extern "C" fn(
    *const ::core::ffi::c_char,
    *mut RESOURCE_DATA,
) -> ();
pub type ResourceFreeFun = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN;
pub type ResourceStringFun = unsafe extern "C" fn(
    *mut RESOURCE_DATA,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uint,
) -> ();
pub type ResourceLoadFileFun = unsafe extern "C" fn(
    *mut uio_Stream,
    DWORD,
) -> *mut ::core::ffi::c_void;
unsafe extern "C" fn GetCelFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = LoadResourceFromPath(
        pathname,
        Some(
            _GetCelData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    );
}
unsafe extern "C" fn GetFontFileData(
    mut pathname: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    (*resdata).ptr = LoadResourceFromPath(
        pathname,
        Some(
            _GetFontData
                as unsafe extern "C" fn(
                    *mut uio_Stream,
                    DWORD,
                ) -> *mut ::core::ffi::c_void,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn InstallGraphicResTypes() -> BOOLEAN {
    InstallResTypeVectors(
        b"GFXRES\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetCelFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            _ReleaseCelData as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    InstallResTypeVectors(
        b"FONTRES\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetFontFileData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            _ReleaseFontData as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadGraphicInstance(
    mut res: RESOURCE,
) -> *mut ::core::ffi::c_void {
    let mut hData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hData = res_GetResource(res);
    if !hData.is_null() {
        res_DetachResource(res);
    }
    return hData;
}
