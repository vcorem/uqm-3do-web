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

// Rust translation of src/libs/uio/debug.c.
extern "C" {
    pub type _IO_FILE;
    pub type uio_DirHandle;
    pub type uio_Handle;
    pub type uio_Repository;
    pub type uio_Stream;
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn clearerr(_: *mut FILE);
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fileno(_: *mut FILE) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn ctime(_: *const time_t) -> *mut ::core::ffi::c_char;
    fn waitpid(_: pid_t, _: *mut ::core::ffi::c_int, _: ::core::ffi::c_int) -> pid_t;
    static mut repository: *mut uio_Repository;
    fn uio_open(
        dir: *mut uio_DirHandle,
        file: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: mode_t,
    ) -> *mut uio_Handle;
    fn uio_close(handle: *mut uio_Handle) -> ::core::ffi::c_int;
    fn uio_rename(
        oldDir: *mut uio_DirHandle,
        oldPath: *const ::core::ffi::c_char,
        newDir: *mut uio_DirHandle,
        newPath: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_stat(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        statBuf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn uio_mkdir(
        dir: *mut uio_DirHandle,
        name: *const ::core::ffi::c_char,
        mode: mode_t,
    ) -> ::core::ffi::c_int;
    fn uio_read(
        handle: *mut uio_Handle,
        buf: *mut ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_rmdir(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_write(
        handle: *mut uio_Handle,
        buf: *const ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn uio_unlink(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn uio_getDirList(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        matchType: match_MatchType,
    ) -> *mut uio_DirList;
    fn uio_DirList_free(dirList: *mut uio_DirList);
    fn uio_DirHandle_print(dirHandle: *const uio_DirHandle, out: *mut FILE);
    fn write(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn fork() -> pid_t;
    fn execvp(
        _: *const ::core::ffi::c_char,
        _: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(_: ::core::ffi::c_int) -> !;
    fn isatty(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uio_printMounts(outStream: *mut FILE, repository_0: *const uio_Repository);
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fgets(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        stream: *mut uio_Stream,
    ) -> *mut ::core::ffi::c_char;
    fn uio_fputc(c: ::core::ffi::c_int, stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fputs(
        s: *const ::core::ffi::c_char,
        stream: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_fwrite(
        buf: *const ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_ferror(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_clearerr(stream: *mut uio_Stream);
    fn uio_getStdioAccess(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        tempDir: *mut uio_DirHandle,
    ) -> *mut ::core::ffi::c_void;
    fn uio_StdioAccessHandle_getPath(
        handle: *mut ::core::ffi::c_void,
    ) -> *const ::core::ffi::c_char;
    fn uio_releaseStdioAccess(handle: *mut ::core::ffi::c_void);
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type FILE = _IO_FILE;
pub type time_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
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
pub type pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugContext {
    pub exit: uio_bool,
    pub r#in: *mut FILE,
    pub out: *mut FILE,
    pub err: *mut FILE,
    pub cwd: *mut uio_DirHandle,
}
pub type uio_bool = bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugCommand {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut DebugContext,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DirList {
    pub names: *mut *const ::core::ffi::c_char,
    pub numNames: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c: *mut ::core::ffi::c_void,
    pub cc: *const ::core::ffi::c_void,
}
pub type uio_StdioAccessHandle = ();
#[no_mangle]
pub static mut debugCommands: [DebugCommand; 15] = [
    DebugCommand {
        name: b"cat\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdCat
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"cd\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdCd
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"exec\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdExec
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"exit\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdExit
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"fwritetest\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdFwriteTest
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"ls\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdLs
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"mem\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdMem
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"mkdir\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdMkDir
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"mount\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdMount
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"mv\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdMv
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"pwd\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdPwd
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"rm\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdRm
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"rmdir\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdRmDir
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"stat\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdStat
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
    DebugCommand {
        name: b"writetest\0".as_ptr() as *const ::core::ffi::c_char,
        fun: Some(
            debugCmdWriteTest
                as unsafe extern "C" fn(
                    *mut DebugContext,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    },
];
#[no_mangle]
pub unsafe extern "C" fn uio_debugInteractive(
    mut r#in: *mut FILE,
    mut out: *mut FILE,
    mut err: *mut FILE,
) {
    let mut lineBuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut lineLen: size_t = 0;
    let mut argc: ::core::ffi::c_int = 0;
    let mut argv: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut debugContext: DebugContext = DebugContext {
        exit: false,
        r#in: ::core::ptr::null_mut::<FILE>(),
        out: ::core::ptr::null_mut::<FILE>(),
        err: ::core::ptr::null_mut::<FILE>(),
        cwd: ::core::ptr::null_mut::<uio_DirHandle>(),
    };
    let mut interactive: uio_bool = false;
    memset(
        &raw mut debugContext as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<DebugContext>(),
    );
    debugContext.exit = false;
    debugContext.r#in = r#in;
    debugContext.out = out;
    debugContext.err = err;
    debugContext.cwd = uqm_lua_checked!(uio_openDir(
        repository,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ), ());
    if debugContext.cwd.is_null() {
        fprintf(
            err,
            b"Fatal: Could not open working dir.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        abort();
    }
    interactive = isatty(fileno(r#in)) != 0;
    loop {
        if interactive {
            fprintf(out, b"> \0".as_ptr() as *const ::core::ffi::c_char);
        }
        if fgets(
                &raw mut lineBuf as *mut ::core::ffi::c_char,
                1024 as ::core::ffi::c_int,
                r#in,
            )
            .is_null()
        {
            if feof(r#in) != 0 {
                break;
            }
            clearerr(r#in);
        } else {
            lineLen = strlen(&raw mut lineBuf as *mut ::core::ffi::c_char);
            if lineBuf[lineLen.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                != '\n' as ::core::ffi::c_int
                && lineBuf[lineLen.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                    != '\r' as ::core::ffi::c_int
            {
                fprintf(
                    err,
                    b"Too long command line.\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                makeArgs(
                    &raw mut lineBuf as *mut ::core::ffi::c_char,
                    &raw mut argc,
                    &raw mut argv,
                );
                if argc == 0 as ::core::ffi::c_int {
                    free(argv as *mut ::core::ffi::c_void);
                } else {
                    uqm_lua_checked!(debugCallCommand(&raw mut debugContext, argc, argv), ());
                    free(argv as *mut ::core::ffi::c_void);
                }
            }
        }
        if debugContext.exit {
            break;
        }
    }
    if interactive {
        fprintf(out, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    uio_closeDir(debugContext.cwd);
}
unsafe extern "C" fn makeArgs(
    mut lineBuf: *mut ::core::ffi::c_char,
    mut argc: *mut ::core::ffi::c_int,
    mut argv: *mut *mut *mut ::core::ffi::c_char,
) {
    let mut numArg: ::core::ffi::c_int = 0;
    let mut args: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    numArg = 0 as ::core::ffi::c_int;
    ptr = lineBuf;
    loop {
        while isspace(*ptr as ::core::ffi::c_int) != 0 {
            ptr = ptr.offset(1);
        }
        if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        numArg += 1;
        while isspace(*ptr as ::core::ffi::c_int) == 0 {
            ptr = ptr.offset(1);
        }
    }
    args = malloc(
        ((numArg + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *mut ::core::ffi::c_char;
    numArg = 0 as ::core::ffi::c_int;
    ptr = lineBuf;
    loop {
        while isspace(*ptr as ::core::ffi::c_int) != 0 {
            ptr = ptr.offset(1);
        }
        if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        *args.offset(numArg as isize) = ptr;
        numArg += 1;
        while isspace(*ptr as ::core::ffi::c_int) == 0 {
            ptr = ptr.offset(1);
        }
        if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        *ptr = '\0' as ::core::ffi::c_char;
        ptr = ptr.offset(1);
    }
    *args.offset(numArg as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *argv = args;
    *argc = numArg;
}
unsafe extern "C" fn debugCallCommand(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut numDebugCommands: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    numDebugCommands = ::core::mem::size_of::<[DebugCommand; 15]>()
        .wrapping_div(::core::mem::size_of::<DebugCommand>()) as ::core::ffi::c_int;
    loop {
        if i == numDebugCommands {
            fprintf(
                (*debugContext).err,
                b"Invalid command.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(*argv.offset(0isize), debugCommands[i as usize].name)
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        i += 1;
    }
    return uqm_lua_checked!(debugCommands[i as usize]
        .fun
        .expect("non-null function pointer")(debugContext, argc, argv), 0);
}
unsafe extern "C" fn debugCmdCat(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let mut readBuf: [::core::ffi::c_char; 65536] = [0; 65536];
    let mut bufPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut numInBuf: ssize_t = 0;
    let mut numWritten: ssize_t = 0;
    let mut totalWritten: size_t = 0;
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    handle = uqm_lua_checked!(uio_open(
        (*debugContext).cwd,
        *argv.offset(1isize),
        0 as ::core::ffi::c_int,
        0 as mode_t,
    ), 0);
    if handle.is_null() {
        fprintf(
            (*debugContext).err,
            b"Could not open file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    totalWritten = 0 as size_t;
    loop {
        numInBuf = uqm_lua_checked!(uio_read(
            handle,
            &raw mut readBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0x10000 as ::core::ffi::c_int as size_t,
        ), 0);
        if numInBuf == -1 as ssize_t {
            if *__errno_location() == 27 as ::core::ffi::c_int {
                continue;
            }
            fprintf(
                (*debugContext).err,
                b"Could not read from file: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            uqm_lua_checked!(uio_close(handle), 0);
            return 1 as ::core::ffi::c_int;
        } else {
            if numInBuf == 0 as ssize_t {
                break;
            }
            bufPtr = &raw mut readBuf as *mut ::core::ffi::c_char;
            loop {
                numWritten = write(
                    fileno((*debugContext).out),
                    bufPtr as *const ::core::ffi::c_void,
                    numInBuf as size_t,
                );
                's_78: {
                    if numWritten == -1 as ssize_t {
                        if *__errno_location() == 27 as ::core::ffi::c_int {
                            break 's_78;
                        } else {
                            fprintf(
                                (*debugContext).err,
                                b"Could not read from file: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                strerror(*__errno_location()),
                            );
                            uqm_lua_checked!(uio_close(handle), 0);
                        }
                    }
                    numInBuf -= numWritten;
                    bufPtr = bufPtr.offset(numWritten as isize);
                    totalWritten = totalWritten.wrapping_add(numWritten as size_t);
                }
                if numInBuf <= 0 as ssize_t {
                    break;
                }
            }
        }
    }
    fprintf(
        (*debugContext).out,
        b"[%u bytes]\n\0".as_ptr() as *const ::core::ffi::c_char,
        totalWritten as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(uio_close(handle), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdCd(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut newWd: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    newWd = uqm_lua_checked!(uio_openDirRelative(
        (*debugContext).cwd,
        *argv.offset(1isize),
        0 as ::core::ffi::c_int,
    ), 0);
    if newWd.is_null() {
        fprintf(
            (*debugContext).err,
            b"Could not access new dir: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    uio_closeDir((*debugContext).cwd);
    (*debugContext).cwd = newWd;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdExec(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut newArgs: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
        *const ::core::ffi::c_char,
    >();
    let mut errCode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut handles: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut tempDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    if argc < 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    tempDir = uqm_lua_checked!(uio_openDirRelative(
        (*debugContext).cwd,
        b"/tmp\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ), 0);
    if tempDir.is_null() {
        fprintf(
            (*debugContext).err,
            b"Could not open temp dir: %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    newArgs = malloc(
        (argc as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *const ::core::ffi::c_char;
    *newArgs.offset(0isize) = *argv.offset(1isize);
    handles = malloc(
        (argc as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>()),
    ) as *mut *mut ::core::ffi::c_void;
    *handles.offset(0isize) = ::core::ptr::null_mut::<::core::ffi::c_void>()
        as *mut ::core::ffi::c_void;
    i = 2 as ::core::ffi::c_int;
    '_err: {
        while i < argc {
            *handles.offset((i - 1 as ::core::ffi::c_int) as isize) = uqm_lua_checked!(uio_getStdioAccess(
                (*debugContext).cwd,
                *argv.offset(i as isize),
                0 as ::core::ffi::c_int,
                tempDir,
            ), 0);
            if (*handles.offset((i - 1 as ::core::ffi::c_int) as isize)).is_null() {
                if *__errno_location() == 44 as ::core::ffi::c_int {
                    *newArgs.offset((i - 1 as ::core::ffi::c_int) as isize) = *argv
                        .offset(i as isize);
                } else {
                    fprintf(
                        (*debugContext).err,
                        b"Cannot execute: Cannot get stdio access to %s: %s.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        *argv.offset(i as isize),
                        strerror(*__errno_location()),
                    );
                    errCode = 1 as ::core::ffi::c_int;
                    argc = i + 1 as ::core::ffi::c_int;
                    break '_err;
                }
            } else {
                *newArgs.offset((i - 1 as ::core::ffi::c_int) as isize) = uio_StdioAccessHandle_getPath(
                    *handles.offset((i - 1 as ::core::ffi::c_int) as isize),
                );
            }
            i += 1;
        }
        *newArgs.offset((argc - 1 as ::core::ffi::c_int) as isize) = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        fprintf(
            (*debugContext).err,
            b"Executing: %s\0".as_ptr() as *const ::core::ffi::c_char,
            *newArgs.offset(0isize),
        );
        i = 1 as ::core::ffi::c_int;
        while i < argc - 1 as ::core::ffi::c_int {
            fprintf(
                (*debugContext).err,
                b" %s\0".as_ptr() as *const ::core::ffi::c_char,
                *newArgs.offset(i as isize),
            );
            i += 1;
        }
        fprintf((*debugContext).err, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        let mut pid: pid_t = 0;
        pid = fork();
        match pid {
            -1 => {
                fprintf(
                    (*debugContext).err,
                    b"Error: fork() failed: %s.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            0 => {
                execvp(
                    *newArgs.offset(0isize),
                    newArgs as *const *mut ::core::ffi::c_char,
                );
                fprintf(
                    (*debugContext).err,
                    b"Error: execvp() failed: %s.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                _exit(1 as ::core::ffi::c_int);
            }
            _ => {
                let mut status: ::core::ffi::c_int = 0;
                let mut retVal: pid_t = 0;
                loop {
                    retVal = waitpid(pid, &raw mut status, 0 as ::core::ffi::c_int);
                    if retVal != -1 as ::core::ffi::c_int {
                        break;
                    }
                    if *__errno_location() == 27 as ::core::ffi::c_int {
                        continue;
                    }
                    fprintf(
                        (*debugContext).err,
                        b"Error: waitpid() failed: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                    break;
                }
                if retVal != -1 as ::core::ffi::c_int {
                    if status & 0x7f as ::core::ffi::c_int == 0 {
                        fprintf(
                            (*debugContext).err,
                            b"Exit status: %d\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (status & 0xff00 as ::core::ffi::c_int)
                                >> 8 as ::core::ffi::c_int,
                        );
                    } else if ((status & 0xffff as ::core::ffi::c_int)
                        as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        < 0xff as ::core::ffi::c_uint
                    {
                        fprintf(
                            (*debugContext).err,
                            b"Terminated on signal %d.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            status & 0x7f as ::core::ffi::c_int,
                        );
                    } else {
                        fprintf(
                            (*debugContext).err,
                            b"Error: weird exit status.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i < argc - 1 as ::core::ffi::c_int {
        if !(*handles.offset(i as isize)).is_null() {
            uqm_lua_checked!(uio_releaseStdioAccess(*handles.offset(i as isize)), 0);
        }
        i += 1;
    }
    free(handles as *mut ::core::ffi::c_void);
    free(newArgs as *mut ::core::ffi::c_void);
    uio_closeDir(tempDir);
    return errCode;
}
unsafe extern "C" fn debugCmdExit(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*debugContext).exit = true;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdFwriteTest(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let testString: [::core::ffi::c_char; 12] = ::core::mem::transmute::<
        [u8; 12],
        [::core::ffi::c_char; 12],
    >(*b"0123456789\n\0");
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    stream = uqm_lua_checked!(uio_fopen(
        (*debugContext).cwd,
        *argv.offset(1isize),
        b"w+b\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    if stream.is_null() {
        fprintf(
            (*debugContext).err,
            b"Could not open file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else if uqm_lua_checked!(uio_fwrite(
        &raw const testString as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        strlen(&raw const testString as *const ::core::ffi::c_char),
        1 as size_t,
        stream,
    ), 0) != 1 as size_t
    {
        fprintf(
            (*debugContext).err,
            b"uio_fwrite() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else if uqm_lua_checked!(uio_fputs(&raw const testString as *const ::core::ffi::c_char, stream), 0)
        == -1 as ::core::ffi::c_int
    {
        fprintf(
            (*debugContext).err,
            b"uio_fputs() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else if uqm_lua_checked!(uio_fseek(stream, 15 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), 0)
        != 0 as ::core::ffi::c_int
    {
        fprintf(
            (*debugContext).err,
            b"uio_fseek() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else if uqm_lua_checked!(uio_fputc('A' as ::core::ffi::c_int, stream), 0) != 'A' as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"uio_fputc() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else if uqm_lua_checked!(uio_fseek(stream, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int), 0)
        != 0 as ::core::ffi::c_int
    {
        fprintf(
            (*debugContext).err,
            b"uio_fseek() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else {
        let mut buf: [::core::ffi::c_char; 6] = [0; 6];
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        loop {
            ptr = uqm_lua_checked!(uio_fgets(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as ::core::ffi::c_int,
                stream,
            ), 0);
            if ptr.is_null() {
                break;
            }
            fprintf(
                (*debugContext).out,
                b"%d: [%s]\n\0".as_ptr() as *const ::core::ffi::c_char,
                i,
                ptr,
            );
            i += 1;
        }
        if uio_ferror(stream) != 0 {
            fprintf(
                (*debugContext).err,
                b"uio_fgets() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        } else {
            uio_clearerr(stream);
            if uqm_lua_checked!(uio_fseek(stream, 4 as ::core::ffi::c_long, 2 as ::core::ffi::c_int), 0)
                != 0 as ::core::ffi::c_int
            {
                fprintf(
                    (*debugContext).err,
                    b"uio_fseek() failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            } else {
                let mut buf_0: [::core::ffi::c_char; 2000] = [0; 2000];
                memset(
                    &raw mut buf_0 as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    'Q' as ::core::ffi::c_int,
                    ::core::mem::size_of::<[::core::ffi::c_char; 2000]>(),
                );
                if uqm_lua_checked!(uio_fwrite(
                    &raw mut buf_0 as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    100 as size_t,
                    20 as size_t,
                    stream,
                ), 0) != 20 as size_t
                {
                    fprintf(
                        (*debugContext).err,
                        b"uio_fwrite() failed: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                } else if uqm_lua_checked!(uio_fseek(
                    stream,
                    5 as ::core::ffi::c_long,
                    0 as ::core::ffi::c_int,
                ), 0) != 0 as ::core::ffi::c_int
                {
                    fprintf(
                        (*debugContext).err,
                        b"uio_fseek() failed: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                } else if uqm_lua_checked!(uio_fputc('B' as ::core::ffi::c_int, stream), 0)
                    != 'B' as ::core::ffi::c_int
                {
                    fprintf(
                        (*debugContext).err,
                        b"uio_fputc() failed: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                } else {
                    uqm_lua_checked!(uio_fclose(stream), 0);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    uqm_lua_checked!(uio_fclose(stream), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn listOneDir(
    mut debugContext: *mut DebugContext,
    mut arg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut dirList: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    let mut i: ::core::ffi::c_int = 0;
    let mut pattern: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut cpath: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *arg.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        cpath = arg;
        pattern = b"*\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        pattern = strrchr(arg, '/' as ::core::ffi::c_int);
        if pattern.is_null() {
            cpath = b"\0".as_ptr() as *const ::core::ffi::c_char;
            pattern = arg;
        } else if *pattern.offset(1isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
        {
            cpath = arg;
            pattern = b"*\0".as_ptr() as *const ::core::ffi::c_char;
        } else {
            if pattern == arg {
                cpath = b"/\0".as_ptr() as *const ::core::ffi::c_char;
            } else {
                buf = malloc((pattern.offset_from(arg) + 1isize) as size_t)
                    as *mut ::core::ffi::c_char;
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    arg as *const ::core::ffi::c_void,
                    pattern.offset_from(arg) as size_t,
                );
                *buf.offset(pattern.offset_from(arg) as isize) = '\0'
                    as ::core::ffi::c_char;
                cpath = buf;
            }
            pattern = pattern.offset(1);
        }
    }
    if *pattern.offset(0isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        && *pattern.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        dirList = uqm_lua_checked!(uio_getDirList(
            (*debugContext).cwd,
            cpath,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            match_MatchType::match_MATCH_PREFIX,
        ), 0);
    } else {
        dirList = uqm_lua_checked!(uio_getDirList(
            (*debugContext).cwd,
            cpath,
            pattern,
            match_MatchType::match_MATCH_LITERAL,
        ), 0);
    }
    if dirList.is_null() {
        fprintf(
            (*debugContext).out,
            b"Error in uio_getDirList(): %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        if !buf.is_null() {
            free(buf as *mut ::core::ffi::c_void);
        }
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*dirList).numNames {
        fprintf(
            (*debugContext).out,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            *(*dirList).names.offset(i as isize),
        );
        i += 1;
    }
    uio_DirList_free(dirList);
    if !buf.is_null() {
        free(buf as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdLs(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut argI: ::core::ffi::c_int = 0;
    let mut retVal: ::core::ffi::c_int = 0;
    if argc == 1 as ::core::ffi::c_int {
        return uqm_lua_checked!(listOneDir(
            debugContext,
            unconst(
                b"\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
            ) as *const ::core::ffi::c_char,
        ), 0);
    }
    argI = 1 as ::core::ffi::c_int;
    while argI < argc {
        retVal = uqm_lua_checked!(listOneDir(debugContext, *argv.offset(argI as isize)), 0);
        if retVal != 0 as ::core::ffi::c_int {
            return retVal;
        }
        argI += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdMem(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fprintf(
        (*debugContext).out,
        b"Memory debugging not compiled in.\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdMkDir(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    retVal = uqm_lua_checked!(uio_mkdir((*debugContext).cwd, *argv.offset(1isize), 0o777 as mode_t), 0);
    if retVal == -1 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Could not create directory: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdMount(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if argc == 1 as ::core::ffi::c_int {
        uio_printMounts((*debugContext).out, repository);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdMv(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    if argc != 3 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    retVal = uqm_lua_checked!(uio_rename(
        (*debugContext).cwd,
        *argv.offset(1isize),
        (*debugContext).cwd,
        *argv.offset(2isize),
    ), 0);
    if retVal == -1 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Could not rename: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdPwd(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    uio_DirHandle_print((*debugContext).cwd, (*debugContext).out);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdRm(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    retVal = uqm_lua_checked!(uio_unlink((*debugContext).cwd, *argv.offset(1isize)), 0);
    if retVal == -1 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Could not remove file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdRmDir(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut retVal: ::core::ffi::c_int = 0;
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    retVal = uqm_lua_checked!(uio_rmdir((*debugContext).cwd, *argv.offset(1isize)), 0);
    if retVal == -1 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Could not remove directory: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdStat(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
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
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(uio_stat((*debugContext).cwd, *argv.offset(1isize), &raw mut statBuf), 0)
        == -1 as ::core::ffi::c_int
    {
        let mut savedErrno: ::core::ffi::c_int = 0;
        savedErrno = *__errno_location();
        fprintf(
            (*debugContext).err,
            b"Could not stat file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        *__errno_location() = savedErrno;
        return 1 as ::core::ffi::c_int;
    }
    fprintf(
        (*debugContext).out,
        b"size %ld bytes\nuid %d  gid %d  mode 0%o\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        statBuf.st_size as ::core::ffi::c_ulong,
        statBuf.st_uid,
        statBuf.st_gid,
        statBuf.st_mode
            & (0o700 as ::core::ffi::c_int | 0o70 as ::core::ffi::c_int
                | 0o7 as ::core::ffi::c_int | 0o4000 as ::core::ffi::c_int
                | 0o2000 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    fprintf(
        (*debugContext).out,
        b"last access:        %s\0".as_ptr() as *const ::core::ffi::c_char,
        ctime(&raw mut statBuf.st_atim.tv_sec),
    );
    fprintf(
        (*debugContext).out,
        b"last modification:  %s\0".as_ptr() as *const ::core::ffi::c_char,
        ctime(&raw mut statBuf.st_mtim.tv_sec),
    );
    fprintf(
        (*debugContext).out,
        b"last status change: %s\0".as_ptr() as *const ::core::ffi::c_char,
        ctime(&raw mut statBuf.st_ctim.tv_sec),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn debugCmdWriteTest(
    mut debugContext: *mut DebugContext,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut handle: *mut uio_Handle = ::core::ptr::null_mut::<uio_Handle>();
    let testString: [::core::ffi::c_char; 14] = ::core::mem::transmute::<
        [u8; 14],
        [::core::ffi::c_char; 14],
    >(*b"Hello world!\n\0");
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            (*debugContext).err,
            b"Invalid number of arguments.\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    handle = uqm_lua_checked!(uio_open(
        (*debugContext).cwd,
        *argv.offset(1isize),
        0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
            | 0o200 as ::core::ffi::c_int,
        0o644 as mode_t,
    ), 0);
    if handle.is_null() {
        fprintf(
            (*debugContext).err,
            b"Could not open file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(uio_write(
        handle,
        &raw const testString as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 14]>(),
    ), 0) == -1 as ssize_t
    {
        fprintf(
            (*debugContext).err,
            b"Write failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(uio_close(handle), 0);
    return 0 as ::core::ffi::c_int;
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
