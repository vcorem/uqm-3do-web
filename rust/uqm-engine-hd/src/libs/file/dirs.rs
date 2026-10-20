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

// Contains code handling directories

// Rust translation of src/libs/file/dirs.c.
extern "C" {
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    fn mkdir(_: *const ::core::ffi::c_char, _: mode_t) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn getcwd(_: *mut ::core::ffi::c_char, _: size_t) -> *mut ::core::ffi::c_char;
    fn getuid() -> uid_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn getpwuid(_: uid_t) -> *mut passwd;
}
pub type size_t = ::core::ffi::c_ulong;
pub type time_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type off_t = ::core::ffi::c_longlong;
pub type ino_t = ::core::ffi::c_ulonglong;
pub type dev_t = ::core::ffi::c_uint;
pub type blksize_t = ::core::ffi::c_int;
pub type blkcnt_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub st_ino: ino_t,
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
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn createDirectory(
    mut dir: *const ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return mkdir(dir, mode as mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn mkdirhier(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pathstart: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut pathend: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut len: size_t = 0;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
    };
    len = strlen(path);
    buf = HMalloc(len.wrapping_add(2 as size_t)) as *mut ::core::ffi::c_char;
    ptr = buf;
    pathstart = path;
    if *pathstart as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        let c2rust_fresh0 = pathstart;
        pathstart = pathstart.offset(1);
        let c2rust_fresh1 = ptr;
        ptr = ptr.offset(1);
        *c2rust_fresh1 = *c2rust_fresh0;
    }
    '_success: {
        if *pathstart as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            '_err: {
                loop {
                    pathend = strchr(pathstart, '/' as ::core::ffi::c_int);
                    if pathend.is_null() {
                        pathend = path.offset(len as isize);
                    }
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        pathstart as *const ::core::ffi::c_void,
                        pathend.offset_from(pathstart) as size_t,
                    );
                    ptr = ptr.offset(pathend.offset_from(pathstart) as isize);
                    *ptr = '\0' as ::core::ffi::c_char;
                    if stat(buf, &raw mut statbuf) == -1 as ::core::ffi::c_int {
                        if *__errno_location() == 44 as ::core::ffi::c_int {
                            break;
                        }
                        log_add(
                            log_Level::log_Error,
                            b"Can't stat \"%s\": %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            buf,
                            strerror(*__errno_location()),
                        );
                        break '_err;
                    } else {
                        if *pathend as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                            break '_success;
                        }
                        *ptr = '/' as ::core::ffi::c_char;
                        ptr = ptr.offset(1);
                        pathstart = pathend.offset(1 as ::core::ffi::c_int as isize);
                        while *pathstart as ::core::ffi::c_int
                            == '/' as ::core::ffi::c_int
                        {
                            pathstart = pathstart.offset(1);
                        }
                        if *pathstart as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        {
                            break '_success;
                        }
                    }
                }
                loop {
                    if createDirectory(buf, 0o777 as ::core::ffi::c_int)
                        == -1 as ::core::ffi::c_int
                    {
                        log_add(
                            log_Level::log_Error,
                            b"Error: Can't create %s: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            buf,
                            strerror(*__errno_location()),
                        );
                        break '_err;
                    } else {
                        if *pathend as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                            break '_success;
                        }
                        *ptr = '/' as ::core::ffi::c_char;
                        ptr = ptr.offset(1);
                        pathstart = pathend.offset(1 as ::core::ffi::c_int as isize);
                        while *pathstart as ::core::ffi::c_int
                            == '/' as ::core::ffi::c_int
                        {
                            pathstart = pathstart.offset(1);
                        }
                        if *pathstart as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        {
                            break '_success;
                        }
                        pathend = strchr(pathstart, '/' as ::core::ffi::c_int);
                        if pathend.is_null() {
                            pathend = path.offset(len as isize);
                        }
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            pathstart as *const ::core::ffi::c_void,
                            pathend.offset_from(pathstart) as size_t,
                        );
                        ptr = ptr.offset(pathend.offset_from(pathstart) as isize);
                        *ptr = '\0' as ::core::ffi::c_char;
                    }
                }
            }
            let mut savedErrno: ::core::ffi::c_int = *__errno_location();
            HFree(buf as *mut ::core::ffi::c_void);
            *__errno_location() = savedErrno;
            return -1 as ::core::ffi::c_int;
        }
    }
    HFree(buf as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getHomeDir() -> *const ::core::ffi::c_char {
    let mut home: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut pw: *mut passwd = ::core::ptr::null_mut::<passwd>();
    home = getenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
    if !home.is_null() {
        return home;
    }
    pw = getpwuid(getuid());
    if pw.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*pw).pw_dir;
}
#[no_mangle]
pub unsafe extern "C" fn expandPath(
    mut dest: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut src: *const ::core::ffi::c_char,
    mut what: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut destptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut destend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bufend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut srcend: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    destptr = dest;
    destend = dest.offset(len as isize);
    '_err: {
        if what & 4 as ::core::ffi::c_int != 0 {
            buf = HMalloc(len) as *mut ::core::ffi::c_char;
            bufptr = buf;
            bufend = buf.offset(len as isize);
            while *src as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                match *src as ::core::ffi::c_int {
                    36 => {
                        let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        let mut envName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut envNameLen: size_t = 0;
                        let mut envVar: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        let mut envVarLen: size_t = 0;
                        src = src.offset(1);
                        if *src as ::core::ffi::c_int == '{' as ::core::ffi::c_int {
                            src = src.offset(1);
                            end = strchr(src, '}' as ::core::ffi::c_int);
                            if end.is_null() {
                                *__errno_location() = 28 as ::core::ffi::c_int;
                                break '_err;
                            } else {
                                envNameLen = end.offset_from(src) as size_t;
                                end = end.offset(1);
                            }
                        } else {
                            end = src;
                            while *end as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                                && *end as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
                                || *end as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                                    && *end as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
                                || *end as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                                    && *end as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                                || *end as ::core::ffi::c_int == '_' as ::core::ffi::c_int
                            {
                                end = end.offset(1);
                            }
                            envNameLen = end.offset_from(src) as size_t;
                        }
                        envName = HMalloc(envNameLen.wrapping_add(1 as size_t))
                            as *mut ::core::ffi::c_char;
                        memcpy(
                            envName as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            envNameLen.wrapping_add(1 as size_t),
                        );
                        *envName.offset(envNameLen as isize) = '\0'
                            as ::core::ffi::c_char;
                        envVar = getenv(envName);
                        HFree(envName as *mut ::core::ffi::c_void);
                        if !envVar.is_null() {
                            envVarLen = strlen(envVar);
                            if bufptr.offset(envVarLen as isize) >= bufend {
                                *__errno_location() = 37 as ::core::ffi::c_int;
                                break '_err;
                            } else {
                                memcpy(
                                    bufptr as *mut ::core::ffi::c_void,
                                    envVar as *const ::core::ffi::c_void,
                                    envVarLen,
                                );
                                bufptr = bufptr.offset(envVarLen as isize);
                            }
                        }
                        src = end;
                    }
                    _ => {
                        if bufptr.offset(1 as ::core::ffi::c_int as isize) >= bufend {
                            *__errno_location() = 37 as ::core::ffi::c_int;
                            break '_err;
                        } else {
                            let c2rust_fresh2 = src;
                            src = src.offset(1);
                            let c2rust_fresh3 = bufptr;
                            bufptr = bufptr.offset(1);
                            *c2rust_fresh3 = *c2rust_fresh2;
                        }
                    }
                }
            }
            *bufptr = '\0' as ::core::ffi::c_char;
            src = buf;
            srcend = bufptr;
        } else {
            srcend = src.offset(strlen(src) as isize);
        }
        if what & 1 as ::core::ffi::c_int != 0 {
            if *src.offset(0isize) as ::core::ffi::c_int == '~' as ::core::ffi::c_int {
                let mut home: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut homelen: size_t = 0;
                if *src.offset(1isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                {
                    *__errno_location() = 28 as ::core::ffi::c_int;
                    break '_err;
                } else {
                    home = getHomeDir();
                    if home.is_null() {
                        *__errno_location() = 44 as ::core::ffi::c_int;
                        break '_err;
                    } else {
                        homelen = strlen(home);
                        if what & 2 as ::core::ffi::c_int != 0 {
                            let mut skip: size_t = 0;
                            destptr = expandPathAbsolute(
                                dest,
                                destend.offset_from(dest) as size_t,
                                home,
                                &raw mut skip,
                                what,
                            );
                            if destptr.is_null() {
                                break '_err;
                            } else {
                                home = home.offset(skip as isize);
                                what &= !(2 as ::core::ffi::c_int);
                            }
                        }
                        if destptr.offset(homelen as isize) >= destend {
                            *__errno_location() = 37 as ::core::ffi::c_int;
                            break '_err;
                        } else {
                            memcpy(
                                destptr as *mut ::core::ffi::c_void,
                                home as *const ::core::ffi::c_void,
                                homelen,
                            );
                            destptr = destptr.offset(homelen as isize);
                            src = src.offset(1);
                        }
                    }
                }
            }
        }
        if what & 2 as ::core::ffi::c_int != 0 {
            let mut skip_0: size_t = 0;
            destptr = expandPathAbsolute(
                destptr,
                destend.offset_from(destptr) as size_t,
                src,
                &raw mut skip_0,
                what,
            );
            if destptr.is_null() {
                break '_err;
            } else {
                src = src.offset(skip_0 as isize);
            }
        }
        if destptr.offset(srcend.offset_from(src) as isize) >= destend {
            *__errno_location() = 37 as ::core::ffi::c_int;
        } else {
            memcpy(
                destptr as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                (srcend.offset_from(src) + 1isize) as size_t,
            );
            if what & 16 as ::core::ffi::c_int != 0 {
                destptr = dest;
                while *destptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    if *destptr as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
                        *destptr = '/' as ::core::ffi::c_char;
                    }
                    destptr = destptr.offset(1);
                }
            }
            if what & 8 as ::core::ffi::c_int != 0 {
                let mut pathStart: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut startPart: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut endPart: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                pathStart = dest;
                if *pathStart.offset(0isize) as ::core::ffi::c_int
                    == '/' as ::core::ffi::c_int
                {
                    pathStart = pathStart.offset(1);
                }
                startPart = pathStart;
                destptr = pathStart;
                loop {
                    endPart = strchr(startPart, '/' as ::core::ffi::c_int);
                    if endPart.is_null() {
                        endPart = startPart.offset(strlen(startPart) as isize);
                    }
                    if !(endPart.offset_from(startPart) == 1isize
                        && *startPart.offset(0isize) as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int)
                    {
                        if endPart.offset_from(startPart) == 2isize
                            && *startPart.offset(0isize) as ::core::ffi::c_int
                                == '.' as ::core::ffi::c_int
                            && *startPart.offset(1isize) as ::core::ffi::c_int
                                == '.' as ::core::ffi::c_int
                        {
                            let mut lastSlash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            lastSlash = strrchr2(
                                pathStart,
                                '/' as ::core::ffi::c_int,
                                destptr.offset(-(1 as ::core::ffi::c_int as isize)),
                            );
                            if lastSlash.is_null() {
                                if destptr == pathStart {
                                    *__errno_location() = 28 as ::core::ffi::c_int;
                                    break '_err;
                                } else {
                                    destptr = pathStart;
                                }
                            } else {
                                destptr = lastSlash;
                                if *endPart as ::core::ffi::c_int
                                    == '/' as ::core::ffi::c_int
                                {
                                    destptr = destptr.offset(1);
                                }
                            }
                        } else {
                            memmove(
                                destptr as *mut ::core::ffi::c_void,
                                startPart as *const ::core::ffi::c_void,
                                endPart.offset_from(startPart) as size_t,
                            );
                            destptr = destptr
                                .offset(endPart.offset_from(startPart) as isize);
                            if *endPart as ::core::ffi::c_int
                                == '/' as ::core::ffi::c_int
                            {
                                *destptr = '/' as ::core::ffi::c_char;
                                destptr = destptr.offset(1);
                            }
                        }
                    }
                    if *endPart as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                        break;
                    }
                    startPart = endPart.offset(1 as ::core::ffi::c_int as isize);
                }
                *destptr = '\0' as ::core::ffi::c_char;
            }
            if what & 32 as ::core::ffi::c_int != 0 {
                let mut srcptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                srcptr = dest;
                destptr = dest;
                while *srcptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    let mut ch: ::core::ffi::c_char = *srcptr;
                    let c2rust_fresh4 = srcptr;
                    srcptr = srcptr.offset(1);
                    let c2rust_fresh5 = destptr;
                    destptr = destptr.offset(1);
                    *c2rust_fresh5 = *c2rust_fresh4;
                    if ch as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                        while *srcptr as ::core::ffi::c_int == '/' as ::core::ffi::c_int
                        {
                            srcptr = srcptr.offset(1);
                        }
                    }
                }
                *destptr = '\0' as ::core::ffi::c_char;
            }
            HFree(buf as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
    }
    if !buf.is_null() {
        let mut savedErrno: ::core::ffi::c_int = *__errno_location();
        HFree(buf as *mut ::core::ffi::c_void);
        *__errno_location() = savedErrno;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn expandPathAbsolute(
    mut dest: *mut ::core::ffi::c_char,
    mut destLen: size_t,
    mut src: *const ::core::ffi::c_char,
    mut skipSrc: *mut size_t,
    mut what: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut orgSrc: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if *src.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || what & 16 as ::core::ffi::c_int != 0
            && *src.offset(0isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
    {
        *skipSrc = 0 as size_t;
        return dest;
    }
    orgSrc = src;
    if getcwd(dest, destLen).is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut tempLen: size_t = 0;
    tempLen = strlen(dest);
    if tempLen == 0 as size_t {
        *__errno_location() = 44 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    dest = dest.offset(tempLen as isize);
    destLen = destLen.wrapping_sub(tempLen);
    if *dest.offset(-1isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
        *dest.offset(0isize) = '/' as ::core::ffi::c_char;
        dest = dest.offset(1);
        destLen = destLen.wrapping_sub(1);
    }
    *skipSrc = src.offset_from(orgSrc) as size_t;
    return dest;
}
unsafe extern "C" fn strrchr2(
    mut start: *const ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    loop {
        end = end.offset(-1);
        if end < start {
            return ::core::ptr::null_mut::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_char;
        }
        if *end as ::core::ffi::c_int == c {
            return unconst(end as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_char;
        }
    };
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
