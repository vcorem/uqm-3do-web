/*
 * Copyright (C) 2003  Serge van den Boom
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * Nota bene: later versions of the GNU General Public License do not apply
 * to this program.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

// Rust translation of src/libs/uio/fileblock.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn uio_read(
        handle: *mut uio_Handle,
        buf: *mut ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_lseek(
        handle: *mut uio_Handle,
        offset: off_t,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn uio_Handle_delete(handle: *mut uio_Handle);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PDirEntryHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PRoot {
    pub mountRef: ::core::ffi::c_int,
    pub handleRef: ::core::ffi::c_int,
    pub rootDir: *mut uio_PDirHandle,
    pub handler: *mut uio_FileSystemHandler,
    pub flags: ::core::ffi::c_int,
    pub handle: *mut uio_Handle,
    pub extra: uio_PRootExtra,
}
pub type uio_PRootExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_Handle {
    pub r#ref: ::core::ffi::c_int,
    pub root: *mut uio_PRoot,
    pub native: uio_NativeHandle,
    pub openFlags: ::core::ffi::c_int,
}
pub type uio_NativeHandle = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_FileSystemHandler {
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub unInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut uio_PRoot, ::core::ffi::c_int) -> ()>,
    pub mount: Option<
        unsafe extern "C" fn(*mut uio_Handle, ::core::ffi::c_int) -> *mut uio_PRoot,
    >,
    pub umount: Option<unsafe extern "C" fn(*mut uio_PRoot) -> ::core::ffi::c_int>,
    pub access: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(*mut uio_Handle) -> ()>,
    pub fstat: Option<
        unsafe extern "C" fn(*mut uio_Handle, *mut stat) -> ::core::ffi::c_int,
    >,
    pub stat: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            *mut stat,
        ) -> ::core::ffi::c_int,
    >,
    pub mkdir: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            mode_t,
        ) -> *mut uio_PDirHandle,
    >,
    pub open: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            mode_t,
        ) -> *mut uio_Handle,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            *mut uio_Handle,
            *mut ::core::ffi::c_void,
            size_t,
        ) -> ssize_t,
    >,
    pub rename: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub rmdir: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub seek: Option<
        unsafe extern "C" fn(*mut uio_Handle, off_t, ::core::ffi::c_int) -> off_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut uio_Handle,
            *const ::core::ffi::c_void,
            size_t,
        ) -> ssize_t,
    >,
    pub unlink: Option<
        unsafe extern "C" fn(
            *mut uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub openEntries: Option<
        unsafe extern "C" fn(*mut uio_PDirHandle) -> uio_NativeEntriesContext,
    >,
    pub readEntries: Option<
        unsafe extern "C" fn(
            *mut uio_NativeEntriesContext,
            *mut ::core::ffi::c_char,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub closeEntries: Option<unsafe extern "C" fn(uio_NativeEntriesContext) -> ()>,
    pub getPDirEntryHandle: Option<
        unsafe extern "C" fn(
            *const uio_PDirHandle,
            *const ::core::ffi::c_char,
        ) -> *mut uio_PDirEntryHandle,
    >,
    pub deletePRootExtra: Option<unsafe extern "C" fn(uio_PRootExtra) -> ()>,
    pub deletePDirHandleExtra: Option<unsafe extern "C" fn(uio_PDirHandleExtra) -> ()>,
    pub deletePFileHandleExtra: Option<unsafe extern "C" fn(uio_PFileHandleExtra) -> ()>,
}
pub type uio_PFileHandleExtra = *mut ::core::ffi::c_void;
pub type uio_PDirHandleExtra = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_PDirHandle {
    pub flags: ::core::ffi::c_int,
    pub r#ref: ::core::ffi::c_int,
    pub pRoot: *mut uio_PRoot,
    pub extra: uio_PDirHandleExtra,
}
pub type uio_NativeEntriesContext = *mut ::core::ffi::c_void;
pub type size_t = ::core::ffi::c_ulong;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
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
pub type ino_t = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type time_t = ::core::ffi::c_longlong;
pub type blkcnt_t = ::core::ffi::c_int;
pub type blksize_t = ::core::ffi::c_int;
pub type dev_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type uid_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_FileBlock {
    pub handle: *mut uio_Handle,
    pub flags: ::core::ffi::c_int,
    pub offset: off_t,
    pub blockSize: size_t,
    pub buffer: *mut ::core::ffi::c_char,
    pub bufSize: size_t,
    pub bufOffset: off_t,
    pub bufFill: size_t,
    pub readAheadBufSize: size_t,
}
unsafe extern "C" fn uio_FileBlock_new(
    mut handle: *mut uio_Handle,
    mut flags: ::core::ffi::c_int,
    mut offset: off_t,
    mut blockSize: size_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufSize: size_t,
    mut bufOffset: off_t,
    mut bufFill: size_t,
    mut readAheadBufSize: size_t,
) -> *mut uio_FileBlock {
    let mut result: *mut uio_FileBlock = ::core::ptr::null_mut::<uio_FileBlock>();
    result = uio_FileBlock_alloc();
    (*result).handle = handle;
    (*result).flags = flags;
    (*result).offset = offset;
    (*result).blockSize = blockSize;
    (*result).buffer = buffer;
    (*result).bufSize = bufSize;
    (*result).bufOffset = bufOffset;
    (*result).bufFill = bufFill;
    (*result).readAheadBufSize = readAheadBufSize;
    return result;
}
#[inline]
unsafe extern "C" fn uio_FileBlock_alloc() -> *mut uio_FileBlock {
    return malloc(::core::mem::size_of::<uio_FileBlock>() as size_t)
        as *mut uio_FileBlock;
}
#[inline]
unsafe extern "C" fn uio_FileBlock_free(mut block: *mut uio_FileBlock) {
    free(block as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn uio_openFileBlock(
    mut handle: *mut uio_Handle,
) -> *mut uio_FileBlock {
    let mut statBuf: stat = stat {
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
    if uio_fstat(handle, &raw mut statBuf) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<uio_FileBlock>();
    }
    uio_Handle_ref(handle);
    return uio_FileBlock_new(
        handle,
        0 as ::core::ffi::c_int,
        0 as off_t,
        statBuf.st_size as size_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        0 as off_t,
        0 as size_t,
        0 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uio_openFileBlock2(
    mut handle: *mut uio_Handle,
    mut offset: off_t,
    mut size: size_t,
) -> *mut uio_FileBlock {
    uio_Handle_ref(handle);
    return uio_FileBlock_new(
        handle,
        0 as ::core::ffi::c_int,
        offset,
        size,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        0 as off_t,
        0 as size_t,
        0 as size_t,
    );
}
#[inline]
unsafe extern "C" fn uio_accessFileBlockMmap(
    mut block: *mut uio_FileBlock,
    mut offset: off_t,
    mut length: size_t,
    mut buffer: *mut *mut ::core::ffi::c_char,
) -> ssize_t {
    *__errno_location() = 52 as ::core::ffi::c_int;
    return -1 as ssize_t;
}
#[inline]
unsafe extern "C" fn uio_accessFileBlockNoMmap(
    mut block: *mut uio_FileBlock,
    mut offset: off_t,
    mut length: size_t,
    mut buffer: *mut *mut ::core::ffi::c_char,
) -> ssize_t {
    let mut numRead: ssize_t = 0;
    let mut start: off_t = 0;
    let mut end: off_t = 0;
    let mut bufSize: size_t = 0;
    let mut oldBuffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if offset > (*block).blockSize as off_t {
        *buffer = (*block).buffer;
        return 0 as ssize_t;
    }
    if length as off_t > (*block).blockSize as off_t - offset {
        length = ((*block).blockSize as off_t - offset) as size_t;
    }
    if !(*block).buffer.is_null() {
        if offset >= (*block).bufOffset
            && (offset - (*block).bufOffset + length as off_t)
                < (*block).bufFill as off_t
        {
            *buffer = (*block).buffer.offset((offset - (*block).bufOffset) as isize);
            return length as ssize_t;
        }
    }
    if length < (*block).readAheadBufSize
        && (*block).flags & (1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
    {
        match (*block).flags & (1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int) {
            1 => {
                start = offset;
                end = if (*block).readAheadBufSize as off_t
                    > (*block).blockSize as off_t - offset
                {
                    (*block).blockSize as off_t
                } else {
                    offset + (*block).readAheadBufSize as off_t
                };
            }
            2 => {
                end = offset + length as off_t;
                start = if end <= (*block).blockSize as off_t {
                    0 as off_t
                } else {
                    end - (*block).bufSize as off_t
                };
            }
            3 => {
                let mut extraBefore: off_t = (*block)
                    .readAheadBufSize
                    .wrapping_sub(length)
                    .wrapping_div(2 as size_t) as off_t;
                start = if offset < extraBefore {
                    0 as off_t
                } else {
                    offset - extraBefore
                };
                end = if (*block).readAheadBufSize as off_t
                    > (*block).blockSize as off_t - start
                {
                    (*block).blockSize as off_t
                } else {
                    start + (*block).readAheadBufSize as off_t
                };
            }
            _ => {}
        }
    } else {
        start = offset;
        end = offset + length as off_t;
    }
    bufSize = if length > (*block).readAheadBufSize {
        length
    } else {
        (*block).readAheadBufSize
    };
    oldBuffer = (*block).buffer;
    if !(*block).buffer.is_null() || (*block).bufSize != bufSize {
        (*block).buffer = malloc(bufSize) as *mut ::core::ffi::c_char;
        (*block).bufSize = bufSize;
    }
    if !oldBuffer.is_null() {
        if oldBuffer != (*block).buffer {
            free(oldBuffer as *mut ::core::ffi::c_void);
        }
    }
    (*block).bufFill = 0 as size_t;
    (*block).bufOffset = start;
    if uio_lseek((*block).handle, (*block).offset + start, 0 as ::core::ffi::c_int)
        as off_t == -1 as ::core::ffi::c_int as off_t
    {
        return -1 as ssize_t;
    }
    numRead = uio_read(
        (*block).handle,
        (*block).buffer as *mut ::core::ffi::c_void,
        (end - start) as size_t,
    );
    if numRead == -1 as ssize_t {
        return -1 as ssize_t;
    }
    (*block).bufFill = numRead as size_t;
    *buffer = (*block).buffer.offset((offset - (*block).bufOffset) as isize);
    if numRead as off_t <= offset - (*block).bufOffset {
        return 0 as ssize_t;
    }
    if numRead as size_t >= length {
        return length as ssize_t;
    }
    return (numRead as off_t - offset) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn uio_accessFileBlock(
    mut block: *mut uio_FileBlock,
    mut offset: off_t,
    mut length: size_t,
    mut buffer: *mut *mut ::core::ffi::c_char,
) -> ssize_t {
    if (*block).flags & 4 as ::core::ffi::c_int != 0 {
        return uio_accessFileBlockMmap(block, offset, length, buffer)
    } else {
        return uio_accessFileBlockNoMmap(block, offset, length, buffer)
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_copyFileBlock(
    mut block: *mut uio_FileBlock,
    mut offset: off_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    if (*block).flags & 4 as ::core::ffi::c_int != 0 {
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    } else {
        let mut numCopied: ssize_t = 0 as ssize_t;
        let mut readResult: ssize_t = 0;
        if offset > (*block).blockSize as off_t {
            return 0 as ::core::ffi::c_int;
        }
        if length as off_t > (*block).blockSize as off_t - offset {
            length = ((*block).blockSize as off_t - offset) as size_t;
        }
        if !(*block).buffer.is_null() && offset >= (*block).bufOffset
            && offset < (*block).bufOffset + (*block).bufFill as off_t
        {
            let mut toCopy: size_t = ((*block).bufFill as off_t - offset) as size_t;
            if toCopy > length {
                toCopy = length;
            }
            memcpy(
                buffer as *mut ::core::ffi::c_void,
                (*block).buffer.offset((offset - (*block).bufOffset) as isize)
                    as *const ::core::ffi::c_void,
                toCopy,
            );
            numCopied = (numCopied as size_t).wrapping_add(toCopy) as ssize_t;
            length = length.wrapping_sub(toCopy);
            if length == 0 as size_t {
                return numCopied as ::core::ffi::c_int;
            }
            buffer = buffer.offset(toCopy as isize);
            offset += toCopy as off_t;
        }
        if uio_lseek((*block).handle, (*block).offset + offset, 0 as ::core::ffi::c_int)
            as off_t == -1 as ::core::ffi::c_int as off_t
        {
            return -1 as ::core::ffi::c_int;
        }
        readResult = uio_read(
            (*block).handle,
            buffer as *mut ::core::ffi::c_void,
            length,
        );
        if readResult == -1 as ssize_t {
            return -1 as ::core::ffi::c_int;
        }
        numCopied += readResult;
        return numCopied as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn uio_closeFileBlock(
    mut block: *mut uio_FileBlock,
) -> ::core::ffi::c_int {
    if (*block).flags & 4 as ::core::ffi::c_int == 0 {
        if !(*block).buffer.is_null() {
            free((*block).buffer as *mut ::core::ffi::c_void);
        }
    }
    uio_Handle_unref((*block).handle);
    uio_FileBlock_free(block);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uio_setFileBlockUsageHint(
    mut block: *mut uio_FileBlock,
    mut usage: ::core::ffi::c_int,
    mut readAheadBufSize: size_t,
) {
    (*block).flags = (*block).flags
        & !(1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
        | usage & (1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int);
    (*block).readAheadBufSize = readAheadBufSize;
}
#[no_mangle]
pub unsafe extern "C" fn uio_clearFileBlockBuffers(mut block: *mut uio_FileBlock) {
    if !(*block).buffer.is_null() {
        free((*block).buffer as *mut ::core::ffi::c_void);
        (*block).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[inline]
unsafe extern "C" fn uio_Handle_ref(mut handle: *mut uio_Handle) {
    (*handle).r#ref += 1;
}
#[inline]
unsafe extern "C" fn uio_Handle_unref(mut handle: *mut uio_Handle) {
    ((*handle).r#ref > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"handle->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/uio/iointrn.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                130 as ::core::ffi::c_int,
                b"uio_Handle_unref\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*handle).r#ref -= 1;
    if (*handle).r#ref == 0 as ::core::ffi::c_int {
        uio_Handle_delete(handle);
    }
}
