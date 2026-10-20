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

// Rust translation of src/libs/decomp/update.c.
extern "C" {
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut _lpCurCodeDesc: PLZHCODE_DESC;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type PVOIDFUNC = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LZHCODE_DESC {
    pub buf_index: COUNT,
    pub restart_index: COUNT,
    pub bytes_left: COUNT,
    pub text_buf: [BYTE; 4111],
    pub freq: [COUNT; 540],
    pub prnt: [COUNT; 809],
    pub son: [COUNT; 539],
    pub workbuf: UWORD,
    pub workbuflen: BYTE,
    pub StreamType: STREAM_TYPE,
    pub Stream: *mut ::core::ffi::c_void,
    pub StreamIndex: DWORD,
    pub StreamLength: DWORD,
    pub StreamMode: STREAM_MODE,
    pub CleanupFunc: PVOIDFUNC,
}
pub type STREAM_MODE = BYTE;
pub type STREAM_TYPE = BYTE;
pub type LZHCODE_DESC = _LZHCODE_DESC;
pub type PLZHCODE_DESC = *mut LZHCODE_DESC;
unsafe extern "C" fn reconst() {
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    j = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        if (*_lpCurCodeDesc).son[i as usize] as ::core::ffi::c_int
            >= (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            (*_lpCurCodeDesc).freq[j as usize] = ((*_lpCurCodeDesc).freq[i as usize]
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COUNT;
            (*_lpCurCodeDesc).son[j as usize] = (*_lpCurCodeDesc).son[i as usize];
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    j = (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as COUNT;
    while (j as ::core::ffi::c_int)
        < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        let mut k: SWORD = 0;
        let mut f: UWORD = 0;
        let mut l: UWORD = 0;
        k = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SWORD;
        (*_lpCurCodeDesc).freq[j as usize] = ((*_lpCurCodeDesc).freq[i as usize]
            as ::core::ffi::c_int
            + (*_lpCurCodeDesc).freq[k as usize] as ::core::ffi::c_int) as COUNT;
        f = (*_lpCurCodeDesc).freq[j as usize] as UWORD;
        k = (j as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SWORD;
        while (f as ::core::ffi::c_int)
            < (*_lpCurCodeDesc).freq[k as usize] as ::core::ffi::c_int
        {
            k -= 1;
        }
        k += 1;
        l = (j as ::core::ffi::c_int - k as ::core::ffi::c_int) as UWORD;
        memmove(
            (&raw mut (*_lpCurCodeDesc).freq as *mut COUNT)
                .offset(k as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (&raw mut (*_lpCurCodeDesc).freq as *mut COUNT)
                .offset(k as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<COUNT>() as size_t).wrapping_mul(l as size_t),
        );
        (*_lpCurCodeDesc).freq[k as usize] = f as COUNT;
        memmove(
            (&raw mut (*_lpCurCodeDesc).son as *mut COUNT)
                .offset(k as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (&raw mut (*_lpCurCodeDesc).son as *mut COUNT)
                .offset(k as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<COUNT>() as size_t).wrapping_mul(l as size_t),
        );
        (*_lpCurCodeDesc).son[k as usize] = i;
        i = (i as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT;
        j = j.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        j = (*_lpCurCodeDesc).son[i as usize];
        if j as ::core::ffi::c_int
            >= (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            (*_lpCurCodeDesc).prnt[j as usize] = i;
        } else {
            (*_lpCurCodeDesc)
                .prnt[(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = i;
            (*_lpCurCodeDesc).prnt[j as usize] = (*_lpCurCodeDesc)
                .prnt[(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _update(mut c: COUNT) {
    let mut lpCD: PLZHCODE_DESC = ::core::ptr::null_mut::<LZHCODE_DESC>();
    lpCD = _lpCurCodeDesc;
    if (*lpCD)
        .freq[((256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
        == 0x8000 as ::core::ffi::c_int
    {
        reconst();
    }
    c = (*lpCD).prnt[c as usize];
    loop {
        let mut i: COUNT = 0;
        let mut l: COUNT = 0;
        (*lpCD).freq[c as usize] = (*lpCD).freq[c as usize].wrapping_add(1);
        i = (*lpCD).freq[c as usize];
        l = (c as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
        if i as ::core::ffi::c_int > (*lpCD).freq[l as usize] as ::core::ffi::c_int {
            let mut j: COUNT = 0;
            loop {
                l = l.wrapping_add(1);
                if i as ::core::ffi::c_int
                    <= (*lpCD).freq[l as usize] as ::core::ffi::c_int
                {
                    break;
                }
            }
            l = l.wrapping_sub(1);
            (*lpCD).freq[c as usize] = (*lpCD).freq[l as usize];
            (*lpCD).freq[l as usize] = i;
            i = (*lpCD).son[c as usize];
            j = (*lpCD).son[l as usize];
            (*lpCD).son[l as usize] = i;
            (*lpCD).son[c as usize] = j;
            (*lpCD).prnt[i as usize] = l;
            if (i as ::core::ffi::c_int)
                < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            {
                (*lpCD)
                    .prnt[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize] = l;
            }
            (*lpCD).prnt[j as usize] = c;
            if (j as ::core::ffi::c_int)
                < (256 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            {
                (*lpCD)
                    .prnt[(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize] = c;
            }
            c = l;
        }
        c = (*lpCD).prnt[c as usize];
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
    };
}
