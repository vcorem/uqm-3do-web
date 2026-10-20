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

// Rust translation of src/libs/luauqm/scriptres.c.
extern "C" {
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
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
pub type size_t = usize;
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
#[no_mangle]
pub unsafe extern "C" fn ReleaseScriptResData(
    mut data: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    HFree(data);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn GetScriptResData(
    mut pathName: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    let mut pathNameLen: size_t = strlen(pathName);
    (*resdata).ptr = HMalloc(pathNameLen.wrapping_add(1 as size_t));
    if (*resdata).ptr.is_null() {
        return;
    }
    memcpy(
        (*resdata).ptr,
        pathName as *const ::core::ffi::c_void,
        pathNameLen.wrapping_add(1 as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn InstallScriptResType() -> BOOLEAN {
    InstallResTypeVectors(
        b"SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetScriptResData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            ReleaseScriptResData
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LoadScriptInstance(
    mut res: RESOURCE,
) -> *mut ::core::ffi::c_char {
    let mut data: *mut ::core::ffi::c_void = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null_mut());
    if !data.is_null() {
        res_DetachResource(res);
    }
    return data as *mut ::core::ffi::c_char;
}
