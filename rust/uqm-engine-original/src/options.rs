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

/*
 * Eventually this should include all configuration stuff,
 * for now there's few options which indicate 3do/pc flavors.
 */

// Rust translation of src/options.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_MountHandle;
    pub type uio_Repository;
    fn uio_mountDir(
        destRep: *mut uio_Repository,
        mountPoint: *const ::core::ffi::c_char,
        fsType: uio_FileSystemID,
        sourceDir: *mut uio_DirHandle,
        sourcePath: *const ::core::ffi::c_char,
        inPath: *const ::core::ffi::c_char,
        autoMount: *mut *mut uio_AutoMount,
        flags: ::core::ffi::c_int,
        relative: *mut uio_MountHandle,
    ) -> *mut uio_MountHandle;
    fn uio_transplantDir(
        mountPoint: *const ::core::ffi::c_char,
        sourceDir: *mut uio_DirHandle,
        flags: ::core::ffi::c_int,
        relative: *mut uio_MountHandle,
    ) -> *mut uio_MountHandle;
    fn uio_stat(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        statBuf: *mut stat,
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
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut repository: *mut uio_Repository;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn LoadResourceIndex(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    );
    fn mkdirhier(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn expandPath(
        dest: *mut ::core::ffi::c_char,
        len: size_t,
        src: *const ::core::ffi::c_char,
        what: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fileExists(name: *const ::core::ffi::c_char) -> bool;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn TFB_SetGamma(gamma: ::core::ffi::c_float) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type off_t = ::core::ffi::c_longlong;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
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
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uio_DirList {
    pub names: *mut *const ::core::ffi::c_char,
    pub numNames: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _input_template {
    pub name: [::core::ffi::c_char; 30],
}
pub type INPUT_TEMPLATE = _input_template;
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
#[no_mangle]
pub static mut optWhichCoarseScan: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optWhichMenu: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optWhichFonts: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optWhichIntro: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optWhichShield: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optSmoothScroll: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optMeleeScale: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut optAddons: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
    *const ::core::ffi::c_char,
>();
#[no_mangle]
pub static mut opt3doMusic: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optRemixMusic: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optSpeech: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optSubtitles: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optStereoSFX: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optKeepAspectRatio: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut optGamma: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut contentDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
#[no_mangle]
pub static mut configDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
#[no_mangle]
pub static mut saveDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
#[no_mangle]
pub static mut meleeDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
#[no_mangle]
pub static mut contentMountHandle: *mut uio_MountHandle = ::core::ptr::null_mut::<
    uio_MountHandle,
>();
#[no_mangle]
pub static mut baseContentPath: [::core::ffi::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut input_templates: [INPUT_TEMPLATE; 6] = [INPUT_TEMPLATE {
    name: [0; 30],
}; 6];
unsafe extern "C" fn findFileInDirs(
    mut locs: *mut *const ::core::ffi::c_char,
    mut numLocs: ::core::ffi::c_int,
    mut file: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut locI: ::core::ffi::c_int = 0;
    let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut fileLen: size_t = 0;
    fileLen = strlen(file);
    locI = 0 as ::core::ffi::c_int;
    while locI < numLocs {
        let mut locLen: size_t = 0;
        let mut loc: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut needSlash: bool = false;
        loc = *locs.offset(locI as isize);
        locLen = strlen(loc);
        needSlash = locLen != 0 as size_t
            && *loc.offset(locLen.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int != '/' as ::core::ffi::c_int;
        if locLen
            .wrapping_add(
                (if needSlash as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            )
            .wrapping_add(fileLen)
            .wrapping_add(1 as size_t)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
        {
            log_add(
                log_Level::log_Warning,
                b"Warning: path '%s' is ignored because it is too long.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                loc,
            );
        } else {
            snprintf(
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                loc,
                if needSlash as ::core::ffi::c_int != 0 {
                    b"/\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                file,
            );
            if fileExists(&raw mut path as *mut ::core::ffi::c_char) {
                return loc;
            }
        }
        locI += 1;
    }
    *__errno_location() = 44 as ::core::ffi::c_int;
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn prepareContentDir(
    mut contentDirName: *const ::core::ffi::c_char,
    mut addonDirName: *const ::core::ffi::c_char,
    mut execFile: *const ::core::ffi::c_char,
) {
    let mut testFile: *const ::core::ffi::c_char = b"version\0".as_ptr()
        as *const ::core::ffi::c_char;
    let mut loc: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if contentDirName.is_null() {
        let mut locs: [*const ::core::ffi::c_char; 4] = [
            b"/usr/local/games/share/uqm/content\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"content\0".as_ptr() as *const ::core::ffi::c_char,
            b"../../content\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        loc = findFileInDirs(
            &raw mut locs as *mut *const ::core::ffi::c_char,
            ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
                as ::core::ffi::c_int,
            testFile,
        );
    } else {
        loc = findFileInDirs(&raw mut contentDirName, 1 as ::core::ffi::c_int, testFile);
    }
    if loc.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not find content.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if expandPath(
        &raw mut baseContentPath as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        loc,
        1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 2 as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int | 32 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not expand path to content directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    log_add(
        log_Level::log_Debug,
        b"Using '%s' as base content dir.\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut baseContentPath as *mut ::core::ffi::c_char,
    );
    contentMountHandle = mountContentDir(
        repository,
        &raw mut baseContentPath as *mut ::core::ffi::c_char,
    );
    if !addonDirName.is_null() {
        log_add(
            log_Level::log_Debug,
            b"Using '%s' as addon dir.\0".as_ptr() as *const ::core::ffi::c_char,
            addonDirName,
        );
    }
    mountAddonDir(repository, contentMountHandle, addonDirName);
}
#[no_mangle]
pub unsafe extern "C" fn prepareConfigDir(
    mut configDirName: *const ::core::ffi::c_char,
) {
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    static mut autoMount: [*mut uio_AutoMount; 1] = [
        ::core::ptr::null_mut::<uio_AutoMount>(),
    ];
    let mut contentHandle: *mut uio_MountHandle = ::core::ptr::null_mut::<
        uio_MountHandle,
    >();
    if configDirName.is_null() {
        configDirName = getenv(
            b"UQM_CONFIG_DIR\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if configDirName.is_null() {
            configDirName = b"~/.uqm/\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if expandPath(
        &raw mut buf as *mut ::core::ffi::c_char,
        (4096 as ::core::ffi::c_int - 13 as ::core::ffi::c_int) as size_t,
        configDirName,
        1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 2 as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int | 32 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Invalid path to config files.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    configDirName = &raw mut buf as *mut ::core::ffi::c_char;
    log_add(
        log_Level::log_Debug,
        b"Using config dir '%s'\0".as_ptr() as *const ::core::ffi::c_char,
        configDirName,
    );
    setenv(
        b"UQM_CONFIG_DIR\0".as_ptr() as *const ::core::ffi::c_char,
        configDirName,
        1 as ::core::ffi::c_int,
    );
    if mkdirhier(configDirName) == -1 as ::core::ffi::c_int {
        exit(1 as ::core::ffi::c_int);
    }
    contentHandle = uio_mountDir(
        repository,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        1 as uio_FileSystemID,
        ::core::ptr::null_mut::<uio_DirHandle>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        configDirName,
        &raw mut autoMount as *mut *mut uio_AutoMount,
        C2Rust_Unnamed::uio_MOUNT_TOP.0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<uio_MountHandle>(),
    );
    if contentHandle.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not mount config dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    configDir = uio_openDir(
        repository,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if configDir.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not open config dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepareSaveDir() {
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut saveDirName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    saveDirName = getenv(b"UQM_SAVE_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    if saveDirName.is_null() {
        saveDirName = b"${UQM_CONFIG_DIR}/save/\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    if expandPath(
        &raw mut buf as *mut ::core::ffi::c_char,
        (4096 as ::core::ffi::c_int - 13 as ::core::ffi::c_int) as size_t,
        saveDirName,
        1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 2 as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int | 32 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Invalid path to config files.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    saveDirName = &raw mut buf as *mut ::core::ffi::c_char;
    setenv(
        b"UQM_SAVE_DIR\0".as_ptr() as *const ::core::ffi::c_char,
        saveDirName,
        1 as ::core::ffi::c_int,
    );
    if mkdirhier(saveDirName) == -1 as ::core::ffi::c_int {
        exit(1 as ::core::ffi::c_int);
    }
    log_add(
        log_Level::log_Debug,
        b"Saved games are kept in %s.\0".as_ptr() as *const ::core::ffi::c_char,
        saveDirName,
    );
    saveDir = uio_openDirRelative(
        configDir,
        b"save\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if saveDir.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not open save dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepareMeleeDir() {
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut meleeDirName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    meleeDirName = getenv(b"UQM_MELEE_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    if meleeDirName.is_null() {
        meleeDirName = b"${UQM_CONFIG_DIR}/teams/\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    if expandPath(
        &raw mut buf as *mut ::core::ffi::c_char,
        (4096 as ::core::ffi::c_int - 13 as ::core::ffi::c_int) as size_t,
        meleeDirName,
        1 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 2 as ::core::ffi::c_int
            | 8 as ::core::ffi::c_int | 32 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Invalid path to config files.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    meleeDirName = &raw mut buf as *mut ::core::ffi::c_char;
    setenv(
        b"UQM_MELEE_DIR\0".as_ptr() as *const ::core::ffi::c_char,
        meleeDirName,
        1 as ::core::ffi::c_int,
    );
    if mkdirhier(meleeDirName) == -1 as ::core::ffi::c_int {
        exit(1 as ::core::ffi::c_int);
    }
    meleeDir = uio_openDirRelative(
        configDir,
        b"teams\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if meleeDir.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not open melee teams dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn mountContentDir(
    mut repository_0: *mut uio_Repository,
    mut contentPath: *const ::core::ffi::c_char,
) -> *mut uio_MountHandle {
    let mut packagesDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    static mut autoMount: [*mut uio_AutoMount; 1] = [
        ::core::ptr::null_mut::<uio_AutoMount>(),
    ];
    let mut contentMountHandle_0: *mut uio_MountHandle = ::core::ptr::null_mut::<
        uio_MountHandle,
    >();
    contentMountHandle_0 = uio_mountDir(
        repository_0,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        1 as uio_FileSystemID,
        ::core::ptr::null_mut::<uio_DirHandle>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        contentPath,
        &raw mut autoMount as *mut *mut uio_AutoMount,
        C2Rust_Unnamed::uio_MOUNT_TOP.0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<uio_MountHandle>(),
    );
    if contentMountHandle_0.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not mount content dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    contentDir = uio_openDir(
        repository_0,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if contentDir.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Fatal error: Could not open content dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    packagesDir = uio_openDir(
        repository_0,
        b"/packages\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if !packagesDir.is_null() {
        mountDirZips(
            packagesDir,
            b"/\0".as_ptr() as *const ::core::ffi::c_char,
            C2Rust_Unnamed::uio_MOUNT_BELOW.0 as ::core::ffi::c_int,
            contentMountHandle_0,
        );
        uio_closeDir(packagesDir);
    }
    return contentMountHandle_0;
}
unsafe extern "C" fn mountAddonDir(
    mut repository_0: *mut uio_Repository,
    mut contentMountHandle_0: *mut uio_MountHandle,
    mut addonDirName: *const ::core::ffi::c_char,
) {
    let mut addonsDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    static mut autoMount: [*mut uio_AutoMount; 1] = [
        ::core::ptr::null_mut::<uio_AutoMount>(),
    ];
    let mut mountHandle: *mut uio_MountHandle = ::core::ptr::null_mut::<
        uio_MountHandle,
    >();
    let mut availableAddons: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    if !addonDirName.is_null() {
        mountHandle = uio_mountDir(
            repository_0,
            b"addons\0".as_ptr() as *const ::core::ffi::c_char,
            1 as uio_FileSystemID,
            ::core::ptr::null_mut::<uio_DirHandle>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            addonDirName,
            &raw mut autoMount as *mut *mut uio_AutoMount,
            C2Rust_Unnamed::uio_MOUNT_TOP.0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<uio_MountHandle>(),
        );
        if mountHandle.is_null() {
            log_add(
                log_Level::log_Warning,
                b"Warning: Could not mount addon directory: %s;\n\t'--addon' options are ignored.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return;
        }
    } else {
        mountHandle = contentMountHandle_0;
    }
    addonsDir = uio_openDirRelative(
        contentDir,
        b"addons\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if addonsDir.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: There's no 'addons' directory in the 'content' directory;\n\t'--addon' options are ignored.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    mountDirZips(
        addonsDir,
        b"addons\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::uio_MOUNT_BELOW.0 as ::core::ffi::c_int,
        mountHandle,
    );
    availableAddons = uio_getDirList(
        addonsDir,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        match_MatchType::match_MATCH_PREFIX,
    );
    if !availableAddons.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut count: ::core::ffi::c_int = 0;
        count = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*availableAddons).numNames {
            let mut sb: stat = stat {
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
            if *(*(*availableAddons).names.offset(i as isize)).offset(0isize)
                as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || uio_stat(
                    addonsDir,
                    *(*availableAddons).names.offset(i as isize),
                    &raw mut sb,
                ) == -1 as ::core::ffi::c_int
                || !(sb.st_mode & 0o170000 as mode_t == 0o40000 as mode_t)
            {
                *(*availableAddons).names.offset(i as isize) = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
            } else {
                count += 1;
            }
            i += 1;
        }
        log_add(
            log_Level::log_Info,
            b"%d available addon pack%s.\0".as_ptr() as *const ::core::ffi::c_char,
            count,
            if count == 1 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"s\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        count = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*availableAddons).numNames {
            let mut mountname: [::core::ffi::c_char; 128] = [0; 128];
            let mut addonDir: *mut uio_DirHandle = ::core::ptr::null_mut::<
                uio_DirHandle,
            >();
            let mut addon: *const ::core::ffi::c_char = *(*availableAddons)
                .names
                .offset(i as isize);
            if !addon.is_null() {
                count += 1;
                log_add(
                    log_Level::log_Info,
                    b"    %d. %s\0".as_ptr() as *const ::core::ffi::c_char,
                    count,
                    addon,
                );
                snprintf(
                    &raw mut mountname as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"addons/%s\0".as_ptr() as *const ::core::ffi::c_char,
                    addon,
                );
                addonDir = uio_openDirRelative(
                    addonsDir,
                    addon,
                    0 as ::core::ffi::c_int,
                );
                if addonDir.is_null() {
                    log_add(
                        log_Level::log_Warning,
                        b"Warning: directory 'addons/%s' not found; addon skipped.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        addon,
                    );
                } else {
                    mountDirZips(
                        addonDir,
                        &raw mut mountname as *mut ::core::ffi::c_char,
                        C2Rust_Unnamed::uio_MOUNT_BELOW.0 as ::core::ffi::c_int,
                        mountHandle,
                    );
                    uio_closeDir(addonDir);
                }
            }
            i += 1;
        }
    } else {
        log_add(
            log_Level::log_Info,
            b"0 available addon packs.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    uio_DirList_free(availableAddons);
    uio_closeDir(addonsDir);
}
unsafe extern "C" fn mountDirZips(
    mut dirHandle: *mut uio_DirHandle,
    mut mountPoint: *const ::core::ffi::c_char,
    mut relativeFlags: ::core::ffi::c_int,
    mut relativeHandle: *mut uio_MountHandle,
) {
    static mut autoMount: [*mut uio_AutoMount; 1] = [
        ::core::ptr::null_mut::<uio_AutoMount>(),
    ];
    let mut dirList: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    dirList = uio_getDirList(
        dirHandle,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\\.([zZ][iI][pP]|[uU][qQ][mM])$\0".as_ptr() as *const ::core::ffi::c_char,
        match_MatchType::match_MATCH_REGEX,
    );
    if !dirList.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*dirList).numNames {
            if uio_mountDir(
                    repository,
                    mountPoint,
                    2 as uio_FileSystemID,
                    dirHandle,
                    *(*dirList).names.offset(i as isize),
                    b"/\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut autoMount as *mut *mut uio_AutoMount,
                    relativeFlags | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                    relativeHandle,
                )
                .is_null()
            {
                log_add(
                    log_Level::log_Warning,
                    b"Warning: Could not mount '%s': %s.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *(*dirList).names.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            i += 1;
        }
    }
    uio_DirList_free(dirList);
}
#[no_mangle]
pub unsafe extern "C" fn loadIndices(mut dir: *mut uio_DirHandle) -> ::core::ffi::c_int {
    let mut indices: *mut uio_DirList = ::core::ptr::null_mut::<uio_DirList>();
    let mut numLoaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    indices = uio_getDirList(
        dir,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\\.[rR][mM][pP]$\0".as_ptr() as *const ::core::ffi::c_char,
        match_MatchType::match_MATCH_REGEX,
    );
    if !indices.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*indices).numNames {
            log_add(
                log_Level::log_Debug,
                b"Loading resource index '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                *(*indices).names.offset(i as isize),
            );
            LoadResourceIndex(
                dir,
                *(*indices).names.offset(i as isize),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            numLoaded += 1;
            i += 1;
        }
    }
    uio_DirList_free(indices);
    return numLoaded;
}
#[no_mangle]
pub unsafe extern "C" fn loadAddon(mut addon: *const ::core::ffi::c_char) -> BOOLEAN {
    let mut addonsDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut addonDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut numLoaded: ::core::ffi::c_int = 0;
    addonsDir = uio_openDirRelative(
        contentDir,
        b"addons\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if addonsDir.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: There's no 'addons' directory in the 'content' directory;\n\t'--addon' options are ignored.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    addonDir = uio_openDirRelative(addonsDir, addon, 0 as ::core::ffi::c_int);
    if addonDir.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: Addon '%s' not found\0".as_ptr() as *const ::core::ffi::c_char,
            addon,
        );
        uio_closeDir(addonsDir);
        return BOOLEAN::FALSE;
    }
    numLoaded = loadIndices(addonDir);
    if numLoaded == 0 {
        log_add(
            log_Level::log_Error,
            b"No RMP index files were loaded for addon '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            addon,
        );
    }
    uio_closeDir(addonDir);
    uio_closeDir(addonsDir);
    return BOOLEAN(
        (numLoaded > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn prepareShadowAddons(
    mut addons: *mut *const ::core::ffi::c_char,
) {
    let mut addonsDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
    let mut shadowDirName: *const ::core::ffi::c_char = b"shadow-content\0".as_ptr()
        as *const ::core::ffi::c_char;
    addonsDir = uio_openDirRelative(
        contentDir,
        b"addons\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if addonsDir.is_null() {
        return;
    }
    while !(*addons).is_null() {
        let mut addon: *const ::core::ffi::c_char = *addons;
        let mut addonDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
        let mut shadowDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
        addonDir = uio_openDirRelative(addonsDir, addon, 0 as ::core::ffi::c_int);
        if !addonDir.is_null() {
            shadowDir = uio_openDirRelative(
                addonDir,
                shadowDirName,
                0 as ::core::ffi::c_int,
            );
            if !shadowDir.is_null() {
                log_add(
                    log_Level::log_Debug,
                    b"Mounting shadow content of '%s' addon\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    addon,
                );
                mountDirZips(
                    shadowDir,
                    b"/\0".as_ptr() as *const ::core::ffi::c_char,
                    C2Rust_Unnamed::uio_MOUNT_ABOVE.0 as ::core::ffi::c_int,
                    contentMountHandle,
                );
                if uio_transplantDir(
                        b"/\0".as_ptr() as *const ::core::ffi::c_char,
                        shadowDir,
                        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                            | C2Rust_Unnamed::uio_MOUNT_ABOVE.0 as ::core::ffi::c_int,
                        contentMountHandle,
                    )
                    .is_null()
                {
                    log_add(
                        log_Level::log_Warning,
                        b"Warning: Could not mount shadow content of '%s': %s.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        addon,
                        strerror(*__errno_location()),
                    );
                }
                uio_closeDir(shadowDir);
            }
            uio_closeDir(addonDir);
        }
        addons = addons.offset(1);
    }
    uio_closeDir(addonsDir);
}
#[no_mangle]
pub unsafe extern "C" fn prepareAddons(mut addons: *mut *const ::core::ffi::c_char) {
    while !(*addons).is_null() {
        log_add(
            log_Level::log_Info,
            b"Loading addon '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            *addons,
        );
        if loadAddon(*addons).0 == 0 {
            break;
        }
        addons = addons.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unprepareAllDirs() {
    if !saveDir.is_null() {
        uio_closeDir(saveDir);
        saveDir = ::core::ptr::null_mut::<uio_DirHandle>();
    }
    if !meleeDir.is_null() {
        uio_closeDir(meleeDir);
        meleeDir = ::core::ptr::null_mut::<uio_DirHandle>();
    }
    if !contentDir.is_null() {
        uio_closeDir(contentDir);
        contentDir = ::core::ptr::null_mut::<uio_DirHandle>();
    }
    if !configDir.is_null() {
        uio_closeDir(configDir);
        configDir = ::core::ptr::null_mut::<uio_DirHandle>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn setGammaCorrection(mut gamma: ::core::ffi::c_float) -> bool {
    let mut set: bool = TFB_SetGamma(gamma);
    if set {
        log_add(
            log_Level::log_Info,
            b"Gamma correction set to %.4f.\0".as_ptr() as *const ::core::ffi::c_char,
            gamma as ::core::ffi::c_double,
        );
    } else {
        log_add(
            log_Level::log_Warning,
            b"Unable to set gamma correction.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return set;
}
