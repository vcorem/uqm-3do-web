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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/libs/strings/unicode.c.
extern "C" {
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut resolutionFactor: ::core::ffi::c_uint;
}
pub type size_t = usize;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type UNICODE = ::core::ffi::c_char;
pub type UniChar = uint32;
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
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_def {
    pub lim: UniChar,
    pub marker: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn resyncUTF8(mut ptr: *mut *const ::core::ffi::c_uchar) {
    while **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        == 0x80 as ::core::ffi::c_int
    {
        *ptr = (*ptr).offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getCharFromString(
    mut ptr: *mut *const ::core::ffi::c_uchar,
) -> UniChar {
    let mut origPtr: *const ::core::ffi::c_uchar = *ptr;
    let mut result: UniChar = 0;
    let mut errData: UniChar = 0;
    if (**ptr as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
        result = **ptr as UniChar;
        *ptr = (*ptr).offset(1);
        return result;
    }
    if **ptr as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
        == 0xc0 as ::core::ffi::c_int
    {
        result = (**ptr as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as UniChar;
        *ptr = (*ptr).offset(1);
        if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        {
            result = result << 6 as ::core::ffi::c_int
                | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as UniChar;
            *ptr = (*ptr).offset(1);
            if result >= 0x80 as UniChar {
                return result;
            }
        }
    } else if **ptr as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
        == 0xe0 as ::core::ffi::c_int
    {
        result = (**ptr as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UniChar;
        *ptr = (*ptr).offset(1);
        if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        {
            result = result << 6 as ::core::ffi::c_int
                | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as UniChar;
            *ptr = (*ptr).offset(1);
            if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
            {
                result = result << 6 as ::core::ffi::c_int
                    | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as UniChar;
                *ptr = (*ptr).offset(1);
                if result >= 0x800 as UniChar {
                    return result;
                }
            }
        }
    } else if **ptr as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
        == 0xf0 as ::core::ffi::c_int
    {
        result = (**ptr as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as UniChar;
        *ptr = (*ptr).offset(1);
        if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        {
            result = result << 6 as ::core::ffi::c_int
                | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as UniChar;
            *ptr = (*ptr).offset(1);
            if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
            {
                result = result << 6 as ::core::ffi::c_int
                    | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as UniChar;
                *ptr = (*ptr).offset(1);
                if **ptr as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
                {
                    result = result << 6 as ::core::ffi::c_int
                        | (**ptr as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                            as UniChar;
                    *ptr = (*ptr).offset(1);
                    if result >= 0x10000 as ::core::ffi::c_int as UniChar {
                        return result;
                    }
                }
            }
        }
    }
    errData = (*origPtr.offset(0isize) as ::core::ffi::c_int
        * 0x1000000 as ::core::ffi::c_int) as UniChar;
    if *origPtr.offset(0isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(1isize) as ::core::ffi::c_int != 0
    {
        errData
            &= (*origPtr.offset(1isize) as ::core::ffi::c_int
                * 0x10000 as ::core::ffi::c_int) as UniChar;
    }
    if *origPtr.offset(0isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(1isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(2isize) as ::core::ffi::c_int != 0
    {
        errData
            &= (*origPtr.offset(2isize) as ::core::ffi::c_int
                * 0x100 as ::core::ffi::c_int) as UniChar;
    }
    if *origPtr.offset(0isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(1isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(2isize) as ::core::ffi::c_int != 0
        && *origPtr.offset(3isize) as ::core::ffi::c_int != 0
    {
        errData &= *origPtr.offset(3isize) as UniChar;
    }
    log_add(
        log_Level::log_Warning,
        b"Warning: Invalid UTF8 sequence: result 0x%x last byte 0x%02x str 0x%08x %s\0"
            .as_ptr() as *const ::core::ffi::c_char,
        result,
        **ptr as ::core::ffi::c_uint,
        errData,
        origPtr,
    );
    resyncUTF8(ptr);
    return 0 as UniChar;
}
#[no_mangle]
pub unsafe extern "C" fn getCharFromStringN(
    mut ptr: *mut *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
) -> UniChar {
    let mut numBytes: size_t = 0;
    '_err: {
        if *ptr != end {
            if (**ptr as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                numBytes = 1 as size_t;
            } else if **ptr as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                numBytes = 2 as size_t;
            } else if **ptr as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                numBytes = 3 as size_t;
            } else if **ptr as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                numBytes = 4 as size_t;
            } else {
                break '_err;
            }
            if (*ptr).offset(numBytes as isize) <= end {
                return getCharFromString(ptr);
            }
        }
    }
    *ptr = end;
    return 0 as UniChar;
}
#[no_mangle]
pub unsafe extern "C" fn getLineFromString(
    mut start: *const ::core::ffi::c_uchar,
    mut end: *mut *const ::core::ffi::c_uchar,
    mut startNext: *mut *const ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    let mut ptr: *const ::core::ffi::c_uchar = start;
    let mut lastPtr: *const ::core::ffi::c_uchar = ::core::ptr::null::<
        ::core::ffi::c_uchar,
    >();
    let mut ch: UniChar = 0;
    loop {
        if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            *end = ptr;
            *startNext = ptr;
            return unconst(start as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_uchar;
        }
        lastPtr = ptr;
        ch = getCharFromString(&raw mut ptr);
        if ch == '\0' as UniChar {
            *end = lastPtr;
            *startNext = ptr;
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if ch == '\n' as UniChar {
            *end = lastPtr;
            if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                *startNext = ptr;
                return unconst(start as *const ::core::ffi::c_void)
                    as *mut ::core::ffi::c_uchar;
            }
            ch = getCharFromString(&raw mut ptr);
            if ch == '\0' as UniChar {
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            if ch == '\r' as UniChar {
                *startNext = ptr;
            } else {
                *startNext = *end;
            }
            return unconst(start as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_uchar;
        } else if ch == '\r' as UniChar {
            *end = lastPtr;
            *startNext = ptr;
            return unconst(start as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_uchar;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringCount(
    mut start: *const ::core::ffi::c_uchar,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    let mut ch: UniChar = 0;
    loop {
        ch = getCharFromString(&raw mut start);
        if ch == '\0' as UniChar {
            return count;
        }
        count = count.wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringCountN(
    mut start: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    let mut ch: UniChar = 0;
    loop {
        ch = getCharFromStringN(&raw mut start, end);
        if ch == '\0' as UniChar {
            return count;
        }
        count = count.wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8CharCount(
    mut start: *const ::core::ffi::c_uchar,
    mut uni_char: UniChar,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    let mut ch: UniChar = 0;
    loop {
        ch = getCharFromString(&raw mut start);
        if ch == '\0' as UniChar {
            return count;
        }
        if ch == uni_char {
            count = count.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringPos(
    mut pStr: *const ::core::ffi::c_uchar,
    mut ch: UniChar,
) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0;
    pos = 0 as ::core::ffi::c_int;
    while *pStr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if getCharFromString(&raw mut pStr) == ch {
            return pos;
        }
        pos += 1;
    }
    if ch == '\0' as UniChar && *pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return pos;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringLastPos(
    mut pStr: *const ::core::ffi::c_uchar,
    mut ch: UniChar,
) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0;
    let mut last_pos: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    pos = 0 as ::core::ffi::c_int;
    while *pStr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if getCharFromString(&raw mut pStr) == ch {
            last_pos = pos;
        }
        pos += 1;
    }
    if last_pos != -1 as ::core::ffi::c_int {
        return last_pos;
    }
    if ch == '\0' as UniChar && *pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return pos;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringCopy(
    mut dst: *mut ::core::ffi::c_uchar,
    mut size: size_t,
    mut src: *const ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    if size == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    strncpy(dst as *mut ::core::ffi::c_char, src as *const ::core::ffi::c_char, size);
    *dst.offset(size.wrapping_sub(1 as size_t) as isize) = '\0' as ::core::ffi::c_uchar;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8StringCompare(
    mut str1: *const ::core::ffi::c_uchar,
    mut str2: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return strcmp(
        str1 as *const ::core::ffi::c_char,
        str2 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn skipUTF8Chars(
    mut ptr: *const ::core::ffi::c_uchar,
    mut num: size_t,
) -> *mut ::core::ffi::c_uchar {
    let mut ch: UniChar = 0;
    let mut oldPtr: *const ::core::ffi::c_uchar = ::core::ptr::null::<
        ::core::ffi::c_uchar,
    >();
    loop {
        let c2rust_fresh0 = num;
        num = num.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        oldPtr = ptr;
        ch = getCharFromString(&raw mut ptr);
        if ch == '\0' as UniChar {
            return unconst(oldPtr as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_uchar;
        }
    }
    return unconst(ptr as *const ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn getUniCharFromStringN(
    mut wstr: *mut UniChar,
    mut maxcount: size_t,
    mut start: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
) -> size_t {
    let mut next: *mut UniChar = ::core::ptr::null_mut::<UniChar>();
    if maxcount == 0 as size_t {
        return 0 as size_t;
    }
    maxcount = maxcount.wrapping_sub(1);
    next = wstr;
    while maxcount > 0 as size_t {
        *next = getCharFromStringN(&raw mut start, end);
        if *next == 0 as UniChar {
            break;
        }
        next = next.offset(1);
        maxcount = maxcount.wrapping_sub(1);
    }
    *next = 0 as UniChar;
    return next.offset_from(wstr) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn getUniCharFromString(
    mut wstr: *mut UniChar,
    mut maxcount: size_t,
    mut start: *const ::core::ffi::c_uchar,
) -> size_t {
    let mut next: *mut UniChar = ::core::ptr::null_mut::<UniChar>();
    if maxcount == 0 as size_t {
        return 0 as size_t;
    }
    maxcount = maxcount.wrapping_sub(1);
    next = wstr;
    while maxcount > 0 as size_t {
        *next = getCharFromString(&raw mut start);
        if *next == 0 as UniChar {
            break;
        }
        next = next.offset(1);
        maxcount = maxcount.wrapping_sub(1);
    }
    *next = 0 as UniChar;
    return next.offset_from(wstr) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn getStringFromChar(
    mut ptr: *mut ::core::ffi::c_uchar,
    mut size: size_t,
    mut ch: UniChar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    static mut ranges: [range_def; 7] = [
        range_def {
            lim: 0x7f as UniChar,
            marker: 0 as ::core::ffi::c_int,
            mask: 0x7f as ::core::ffi::c_int,
        },
        range_def {
            lim: 0x7ff as UniChar,
            marker: 0xc0 as ::core::ffi::c_int,
            mask: 0x1f as ::core::ffi::c_int,
        },
        range_def {
            lim: 0xffff as UniChar,
            marker: 0xe0 as ::core::ffi::c_int,
            mask: 0xf as ::core::ffi::c_int,
        },
        range_def {
            lim: 0x1fffff as ::core::ffi::c_int as UniChar,
            marker: 0xf0 as ::core::ffi::c_int,
            mask: 0x7 as ::core::ffi::c_int,
        },
        range_def {
            lim: 0x3ffffff as ::core::ffi::c_int as UniChar,
            marker: 0xf8 as ::core::ffi::c_int,
            mask: 0x3 as ::core::ffi::c_int,
        },
        range_def {
            lim: 0x7fffffff as ::core::ffi::c_int as UniChar,
            marker: 0xfc as ::core::ffi::c_int,
            mask: 0x1 as ::core::ffi::c_int,
        },
        range_def {
            lim: 0 as UniChar,
            marker: 0 as ::core::ffi::c_int,
            mask: 0 as ::core::ffi::c_int,
        },
    ];
    let mut def: *const range_def = ::core::ptr::null::<range_def>();
    i = 0 as ::core::ffi::c_int;
    def = &raw const ranges as *const range_def as *const range_def;
    while ch > (*def).lim && (*def).mask != 0 as ::core::ffi::c_int {
        i += 1;
        def = def.offset(1);
    }
    if (*def).mask == 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"Warning: Invalid or unsupported unicode char (%lu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            ch as ::core::ffi::c_ulong,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (i as size_t).wrapping_add(1 as size_t) > size {
        return -(i + 1 as ::core::ffi::c_int);
    }
    's_91: {
        'c_3063: {
            'c_3061: {
                'c_3059: {
                    'c_3057: {
                        match i {
                            5 => {
                                *ptr.offset(5isize) = (ch & 0x3f as UniChar
                                    | 0x80 as UniChar) as ::core::ffi::c_uchar;
                                ch >>= 6 as ::core::ffi::c_int;
                            }
                            4 => {}
                            3 => {
                                break 'c_3057;
                            }
                            2 => {
                                break 'c_3059;
                            }
                            1 => {
                                break 'c_3061;
                            }
                            0 => {
                                break 'c_3063;
                            }
                            _ => {
                                break 's_91;
                            }
                        }
                        *ptr.offset(4isize) = (ch & 0x3f as UniChar | 0x80 as UniChar)
                            as ::core::ffi::c_uchar;
                        ch >>= 6 as ::core::ffi::c_int;
                    }
                    *ptr.offset(3isize) = (ch & 0x3f as UniChar | 0x80 as UniChar)
                        as ::core::ffi::c_uchar;
                    ch >>= 6 as ::core::ffi::c_int;
                }
                *ptr.offset(2isize) = (ch & 0x3f as UniChar | 0x80 as UniChar)
                    as ::core::ffi::c_uchar;
                ch >>= 6 as ::core::ffi::c_int;
            }
            *ptr.offset(1isize) = (ch & 0x3f as UniChar | 0x80 as UniChar)
                as ::core::ffi::c_uchar;
            ch >>= 6 as ::core::ffi::c_int;
        }
        *ptr.offset(0isize) = (ch & (*def).mask as UniChar | (*def).marker as UniChar)
            as ::core::ffi::c_uchar;
    }
    return i + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getStringFromWideN(
    mut ptr: *mut ::core::ffi::c_uchar,
    mut size: size_t,
    mut wstr: *const UniChar,
    mut count: size_t,
) -> size_t {
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut used: ::core::ffi::c_int = 0;
    if size == 0 as size_t {
        return 0 as size_t;
    }
    size = size.wrapping_sub(1);
    next = ptr;
    while size > 0 as size_t && count > 0 as size_t {
        used = getStringFromChar(next, size, *wstr);
        if used < 0 as ::core::ffi::c_int {
            break;
        }
        if used == 0 as ::core::ffi::c_int {
            *next = '?' as ::core::ffi::c_uchar;
            used = 1 as ::core::ffi::c_int;
        }
        size = size.wrapping_sub(used as size_t);
        next = next.offset(used as isize);
        count = count.wrapping_sub(1);
        wstr = wstr.offset(1);
    }
    *next = '\0' as ::core::ffi::c_uchar;
    return (next.offset_from(ptr) + 1isize) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn getStringFromWide(
    mut ptr: *mut ::core::ffi::c_uchar,
    mut size: size_t,
    mut wstr: *const UniChar,
) -> size_t {
    let mut end: *const UniChar = ::core::ptr::null::<UniChar>();
    end = wstr;
    while *end != 0 as UniChar {
        end = end.offset(1);
    }
    return getStringFromWideN(ptr, size, wstr, end.offset_from(wstr) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn UniChar_isGraph(mut ch: UniChar) -> ::core::ffi::c_int {
    return (ch > 0xa0 as UniChar && (ch < 0xe000 as UniChar || ch > 0xf8ff as UniChar)
        || ch > 0x20 as UniChar && ch < 0x7f as UniChar) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UniChar_isPrint(mut ch: UniChar) -> ::core::ffi::c_int {
    return (ch == 0x20 as UniChar || ch == 0x9 as UniChar || UniChar_isGraph(ch) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UniChar_toUpper(mut ch: UniChar) -> UniChar {
    return if ch < 0x100 as UniChar {
        toupper(ch as ::core::ffi::c_int) as UniChar
    } else {
        ch
    };
}
#[no_mangle]
pub unsafe extern "C" fn UniChar_toLower(mut ch: UniChar) -> UniChar {
    return if ch < 0x100 as UniChar {
        tolower(ch as ::core::ffi::c_int) as UniChar
    } else {
        ch
    };
}
#[no_mangle]
pub unsafe extern "C" fn AlignText(
    mut str: *const UNICODE,
    mut loc_x: *mut sint16,
) -> *mut UNICODE {
    let mut modSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut first_pos: ::core::ffi::c_int = utf8StringPos(
        str as *mut ::core::ffi::c_uchar,
        0x7c as UniChar,
    );
    let mut last_pos: ::core::ffi::c_int = utf8StringLastPos(
        str as *mut ::core::ffi::c_uchar,
        0x7c as UniChar,
    );
    if utf8CharCount(str as *mut ::core::ffi::c_uchar, 0x7c as UniChar) != 2 as size_t
        || str.is_null() || first_pos != 0 as ::core::ffi::c_int
        || last_pos == -1 as ::core::ffi::c_int || last_pos == 0 as ::core::ffi::c_int
    {
        return str as *mut UNICODE;
    }
    if sscanf(str, b"|%d|\0".as_ptr() as *const ::core::ffi::c_char, &raw mut modSize)
        != 1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Debug,
            b"\nVariable between delimiters is missing, corrupt, or not an integer: %s\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            str,
        );
        return str as *mut UNICODE;
    }
    if modSize != 0 as ::core::ffi::c_int {
        *loc_x = (*loc_x as ::core::ffi::c_int + (modSize << resolutionFactor))
            as sint16;
    }
    return skipUTF8Chars(
        str as *mut ::core::ffi::c_uchar,
        (last_pos + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut UNICODE;
}
#[no_mangle]
pub unsafe extern "C" fn AddPadd(
    mut str: *const UNICODE,
    mut padding: *mut sint16,
) -> *mut UNICODE {
    let mut modSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut first_pos: ::core::ffi::c_int = utf8StringPos(
        str as *mut ::core::ffi::c_uchar,
        0x3a as UniChar,
    );
    let mut last_pos: ::core::ffi::c_int = utf8StringLastPos(
        str as *mut ::core::ffi::c_uchar,
        0x3a as UniChar,
    );
    if utf8CharCount(str as *mut ::core::ffi::c_uchar, 0x3a as UniChar) != 2 as size_t
        || str.is_null() || first_pos != 0 as ::core::ffi::c_int
        || last_pos == -1 as ::core::ffi::c_int || last_pos == 0 as ::core::ffi::c_int
    {
        return str as *mut UNICODE;
    }
    if sscanf(str, b":%d:\0".as_ptr() as *const ::core::ffi::c_char, &raw mut modSize)
        != 1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Debug,
            b"\nVariable between delimiters is missing, corrupt, or not an integer: %s\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            str,
        );
        return str as *mut UNICODE;
    }
    if modSize != 0 as ::core::ffi::c_int {
        *padding = (*padding as ::core::ffi::c_int + (modSize << resolutionFactor))
            as sint16;
    }
    return skipUTF8Chars(
        str as *mut ::core::ffi::c_uchar,
        (last_pos + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut UNICODE;
}
#[inline]
unsafe extern "C" fn unconst(
    mut arg: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut u: C2Rust_Unnamed = C2Rust_Unnamed {
        c: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    u.cc = arg;
    return u.c;
}
