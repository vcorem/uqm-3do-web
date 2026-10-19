/*	MikMod sound library
	(c) 1998, 1999, 2000 Miodrag Vallat and others - see file AUTHORS for
	complete list.

	This library is free software; you can redistribute it and/or modify
	it under the terms of the GNU Library General Public License as
	published by the Free Software Foundation; either version 2 of
	the License, or (at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU Library General Public License for more details.

	You should have received a copy of the GNU Library General Public
	License along with this library; if not, write to the Free Software
	Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA
	02111-1307, USA.
*/

/*==============================================================================

  $Id$

  Portable file I/O routines

==============================================================================*/

/*

	The way this module works:

	- _mm_fopen will call the errorhandler [see mmerror.c] in addition to
	  setting _mm_errno on exit.
	- _mm_iobase is for internal use.  It is used by Player_LoadFP to
	  ensure that it works properly with wad files.
	- _mm_read_I_* and _mm_read_M_* differ : the first is for reading data
	  written by a little endian (intel) machine, and the second is for reading
	  big endian (Mac, RISC, Alpha) machine data.
	- _mm_write functions work the same as the _mm_read functions.
	- _mm_read_string is for reading binary strings.  It is basically the same
	  as an fread of bytes.

*/

// Rust translation of src/libs/mikmod/mmio.c.
extern "C" {
    pub type _IO_FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn fread(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn fputc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut MikMod_errno: ::core::ffi::c_int;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    static mut _mm_errorhandler: MikMod_handler_t;
}
pub type size_t = usize;
pub type FILE = _IO_FILE;
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type UBYTE = ::core::ffi::c_uchar;
pub type SWORD = ::core::ffi::c_short;
pub type UWORD = ::core::ffi::c_ushort;
pub type SLONG = ::core::ffi::c_long;
pub type ULONG = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const MMERR_OPENING_FILE: Self = Self(1);
    pub const MMERR_OUT_OF_MEMORY: Self = Self(2);
    pub const MMERR_DYNAMIC_LINKING: Self = Self(3);
    pub const MMERR_SAMPLE_TOO_BIG: Self = Self(4);
    pub const MMERR_OUT_OF_HANDLES: Self = Self(5);
    pub const MMERR_UNKNOWN_WAVE_TYPE: Self = Self(6);
    pub const MMERR_LOADING_PATTERN: Self = Self(7);
    pub const MMERR_LOADING_TRACK: Self = Self(8);
    pub const MMERR_LOADING_HEADER: Self = Self(9);
    pub const MMERR_LOADING_SAMPLEINFO: Self = Self(10);
    pub const MMERR_NOT_A_MODULE: Self = Self(11);
    pub const MMERR_NOT_A_STREAM: Self = Self(12);
    pub const MMERR_MED_SYNTHSAMPLES: Self = Self(13);
    pub const MMERR_ITPACK_INVALID_DATA: Self = Self(14);
    pub const MMERR_DETECTING_DEVICE: Self = Self(15);
    pub const MMERR_INVALID_DEVICE: Self = Self(16);
    pub const MMERR_INITIALIZING_MIXER: Self = Self(17);
    pub const MMERR_OPENING_AUDIO: Self = Self(18);
    pub const MMERR_8BIT_ONLY: Self = Self(19);
    pub const MMERR_16BIT_ONLY: Self = Self(20);
    pub const MMERR_STEREO_ONLY: Self = Self(21);
    pub const MMERR_ULAW: Self = Self(22);
    pub const MMERR_NON_BLOCK: Self = Self(23);
    pub const MMERR_AF_AUDIO_PORT: Self = Self(24);
    pub const MMERR_AIX_CONFIG_INIT: Self = Self(25);
    pub const MMERR_AIX_CONFIG_CONTROL: Self = Self(26);
    pub const MMERR_AIX_CONFIG_START: Self = Self(27);
    pub const MMERR_GUS_SETTINGS: Self = Self(28);
    pub const MMERR_GUS_RESET: Self = Self(29);
    pub const MMERR_GUS_TIMER: Self = Self(30);
    pub const MMERR_HP_SETSAMPLESIZE: Self = Self(31);
    pub const MMERR_HP_SETSPEED: Self = Self(32);
    pub const MMERR_HP_CHANNELS: Self = Self(33);
    pub const MMERR_HP_AUDIO_OUTPUT: Self = Self(34);
    pub const MMERR_HP_AUDIO_DESC: Self = Self(35);
    pub const MMERR_HP_BUFFERSIZE: Self = Self(36);
    pub const MMERR_OSS_SETFRAGMENT: Self = Self(37);
    pub const MMERR_OSS_SETSAMPLESIZE: Self = Self(38);
    pub const MMERR_OSS_SETSTEREO: Self = Self(39);
    pub const MMERR_OSS_SETSPEED: Self = Self(40);
    pub const MMERR_SGI_SPEED: Self = Self(41);
    pub const MMERR_SGI_16BIT: Self = Self(42);
    pub const MMERR_SGI_8BIT: Self = Self(43);
    pub const MMERR_SGI_STEREO: Self = Self(44);
    pub const MMERR_SGI_MONO: Self = Self(45);
    pub const MMERR_SUN_INIT: Self = Self(46);
    pub const MMERR_OS2_MIXSETUP: Self = Self(47);
    pub const MMERR_OS2_SEMAPHORE: Self = Self(48);
    pub const MMERR_OS2_TIMER: Self = Self(49);
    pub const MMERR_OS2_THREAD: Self = Self(50);
    pub const MMERR_DS_PRIORITY: Self = Self(51);
    pub const MMERR_DS_BUFFER: Self = Self(52);
    pub const MMERR_DS_FORMAT: Self = Self(53);
    pub const MMERR_DS_NOTIFY: Self = Self(54);
    pub const MMERR_DS_EVENT: Self = Self(55);
    pub const MMERR_DS_THREAD: Self = Self(56);
    pub const MMERR_DS_UPDATE: Self = Self(57);
    pub const MMERR_WINMM_HANDLE: Self = Self(58);
    pub const MMERR_WINMM_ALLOCATED: Self = Self(59);
    pub const MMERR_WINMM_DEVICEID: Self = Self(60);
    pub const MMERR_WINMM_FORMAT: Self = Self(61);
    pub const MMERR_WINMM_UNKNOWN: Self = Self(62);
    pub const MMERR_MAC_SPEED: Self = Self(63);
    pub const MMERR_MAC_START: Self = Self(64);
    pub const MMERR_OSX_UNKNOWN_DEVICE: Self = Self(65);
    pub const MMERR_OSX_BAD_PROPERTY: Self = Self(66);
    pub const MMERR_OSX_UNSUPPORTED_FORMAT: Self = Self(67);
    pub const MMERR_OSX_SET_STEREO: Self = Self(68);
    pub const MMERR_OSX_BUFFER_ALLOC: Self = Self(69);
    pub const MMERR_OSX_ADD_IO_PROC: Self = Self(70);
    pub const MMERR_OSX_DEVICE_START: Self = Self(71);
    pub const MMERR_OSX_PTHREAD: Self = Self(72);
    pub const MMERR_DOSWSS_STARTDMA: Self = Self(73);
    pub const MMERR_DOSSB_STARTDMA: Self = Self(74);
    pub const MMERR_NO_FLOAT32: Self = Self(75);
    pub const MMERR_OPENAL_CREATECTX: Self = Self(76);
    pub const MMERR_OPENAL_CTXCURRENT: Self = Self(77);
    pub const MMERR_OPENAL_GENBUFFERS: Self = Self(78);
    pub const MMERR_OPENAL_GENSOURCES: Self = Self(79);
    pub const MMERR_OPENAL_SOURCE: Self = Self(80);
    pub const MMERR_OPENAL_QUEUEBUFFERS: Self = Self(81);
    pub const MMERR_OPENAL_UNQUEUEBUFFERS: Self = Self(82);
    pub const MMERR_OPENAL_BUFFERDATA: Self = Self(83);
    pub const MMERR_OPENAL_GETSOURCE: Self = Self(84);
    pub const MMERR_OPENAL_SOURCEPLAY: Self = Self(85);
    pub const MMERR_OPENAL_SOURCESTOP: Self = Self(86);
    pub const MMERR_ALSA_NOCONFIG: Self = Self(87);
    pub const MMERR_ALSA_SETPARAMS: Self = Self(88);
    pub const MMERR_ALSA_SETFORMAT: Self = Self(89);
    pub const MMERR_ALSA_SETRATE: Self = Self(90);
    pub const MMERR_ALSA_SETCHANNELS: Self = Self(91);
    pub const MMERR_ALSA_BUFFERSIZE: Self = Self(92);
    pub const MMERR_ALSA_PCM_START: Self = Self(93);
    pub const MMERR_ALSA_PCM_WRITE: Self = Self(94);
    pub const MMERR_ALSA_PCM_RECOVER: Self = Self(95);
    pub const MMERR_SNDIO_SETPARAMS: Self = Self(96);
    pub const MMERR_SNDIO_BADPARAMS: Self = Self(97);
    pub const MMERR_MAX: Self = Self(98);
}
pub type MikMod_handler = unsafe extern "C" fn() -> ();
pub type MikMod_handler_t = Option<MikMod_handler>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MREADER {
    pub Seek: Option<
        unsafe extern "C" fn(
            *mut MREADER,
            ::core::ffi::c_long,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub Tell: Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long>,
    pub Read: Option<
        unsafe extern "C" fn(*mut MREADER, *mut ::core::ffi::c_void, size_t) -> BOOL,
    >,
    pub Get: Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int>,
    pub Eof: Option<unsafe extern "C" fn(*mut MREADER) -> BOOL>,
    pub iobase: ::core::ffi::c_long,
    pub prev_iobase: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MWRITER {
    pub Seek: Option<
        unsafe extern "C" fn(
            *mut MWRITER,
            ::core::ffi::c_long,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub Tell: Option<unsafe extern "C" fn(*mut MWRITER) -> ::core::ffi::c_long>,
    pub Write: Option<
        unsafe extern "C" fn(*mut MWRITER, *const ::core::ffi::c_void, size_t) -> BOOL,
    >,
    pub Put: Option<
        unsafe extern "C" fn(*mut MWRITER, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MMEMREADER {
    pub core: MREADER,
    pub buffer: *const ::core::ffi::c_void,
    pub len: ::core::ffi::c_long,
    pub pos: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MFILEREADER {
    pub core: MREADER,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MFILEWRITER {
    pub core: MWRITER,
    pub file: *mut FILE,
}
#[no_mangle]
pub unsafe extern "C" fn _mm_fopen(
    mut fname: *const CHAR,
    mut attrib: *const CHAR,
) -> *mut FILE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = fopen(fname, attrib) as *mut FILE;
    if fp.is_null() {
        MikMod_errno = C2Rust_Unnamed::MMERR_OPENING_FILE.0 as ::core::ffi::c_int;
        if _mm_errorhandler.is_some() {
            _mm_errorhandler.expect("non-null function pointer")();
        }
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_FileExists(mut fname: *const CHAR) -> BOOL {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = fopen(fname, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        return 0 as BOOL;
    }
    fclose(fp);
    return 1 as BOOL;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_fclose(mut fp: *mut FILE) -> ::core::ffi::c_int {
    return fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_iobase_setcur(mut reader: *mut MREADER) {
    (*reader).prev_iobase = (*reader).iobase;
    (*reader).iobase = (*reader).Tell.expect("non-null function pointer")(reader);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_iobase_revert(mut reader: *mut MREADER) {
    (*reader).iobase = (*reader).prev_iobase;
}
unsafe extern "C" fn _mm_FileReader_Eof(mut reader: *mut MREADER) -> BOOL {
    return feof((*(reader as *mut MFILEREADER)).file);
}
unsafe extern "C" fn _mm_FileReader_Read(
    mut reader: *mut MREADER,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> BOOL {
    return (fread(ptr, size, 1 as size_t, (*(reader as *mut MFILEREADER)).file) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn _mm_FileReader_Get(mut reader: *mut MREADER) -> ::core::ffi::c_int {
    return fgetc((*(reader as *mut MFILEREADER)).file);
}
unsafe extern "C" fn _mm_FileReader_Seek(
    mut reader: *mut MREADER,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return fseek(
        (*(reader as *mut MFILEREADER)).file,
        if whence == 0 as ::core::ffi::c_int {
            offset + (*reader).iobase
        } else {
            offset
        },
        whence,
    );
}
unsafe extern "C" fn _mm_FileReader_Tell(
    mut reader: *mut MREADER,
) -> ::core::ffi::c_long {
    return ftell((*(reader as *mut MFILEREADER)).file) - (*reader).iobase;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_new_file_reader(mut fp: *mut FILE) -> *mut MREADER {
    let mut reader: *mut MFILEREADER = MikMod_calloc(
        1 as size_t,
        ::core::mem::size_of::<MFILEREADER>(),
    ) as *mut MFILEREADER;
    if !reader.is_null() {
        (*reader).core.Eof = Some(
            _mm_FileReader_Eof as unsafe extern "C" fn(*mut MREADER) -> BOOL,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> BOOL>;
        (*reader).core.Read = Some(
            _mm_FileReader_Read
                as unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
            >;
        (*reader).core.Get = Some(
            _mm_FileReader_Get
                as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int>;
        (*reader).core.Seek = Some(
            _mm_FileReader_Seek
                as unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*reader).core.Tell = Some(
            _mm_FileReader_Tell
                as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long>;
        (*reader).file = fp;
    }
    return reader as *mut MREADER;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_delete_file_reader(mut reader: *mut MREADER) {
    MikMod_free(reader as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn _mm_FileWriter_Seek(
    mut writer: *mut MWRITER,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return fseek((*(writer as *mut MFILEWRITER)).file, offset, whence);
}
unsafe extern "C" fn _mm_FileWriter_Tell(
    mut writer: *mut MWRITER,
) -> ::core::ffi::c_long {
    return ftell((*(writer as *mut MFILEWRITER)).file);
}
unsafe extern "C" fn _mm_FileWriter_Write(
    mut writer: *mut MWRITER,
    mut ptr: *const ::core::ffi::c_void,
    mut size: size_t,
) -> BOOL {
    return (fwrite(ptr, size, 1 as size_t, (*(writer as *mut MFILEWRITER)).file)
        as size_t == size) as ::core::ffi::c_int;
}
unsafe extern "C" fn _mm_FileWriter_Put(
    mut writer: *mut MWRITER,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return fputc(value, (*(writer as *mut MFILEWRITER)).file);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_new_file_writer(mut fp: *mut FILE) -> *mut MWRITER {
    let mut writer: *mut MFILEWRITER = MikMod_calloc(
        1 as size_t,
        ::core::mem::size_of::<MFILEWRITER>(),
    ) as *mut MFILEWRITER;
    if !writer.is_null() {
        (*writer).core.Seek = Some(
            _mm_FileWriter_Seek
                as unsafe extern "C" fn(
                    *mut MWRITER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MWRITER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*writer).core.Tell = Some(
            _mm_FileWriter_Tell
                as unsafe extern "C" fn(*mut MWRITER) -> ::core::ffi::c_long,
        ) as Option<unsafe extern "C" fn(*mut MWRITER) -> ::core::ffi::c_long>;
        (*writer).core.Write = Some(
            _mm_FileWriter_Write
                as unsafe extern "C" fn(
                    *mut MWRITER,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MWRITER,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
            >;
        (*writer).core.Put = Some(
            _mm_FileWriter_Put
                as unsafe extern "C" fn(
                    *mut MWRITER,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MWRITER,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*writer).file = fp;
    }
    return writer as *mut MWRITER;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_delete_file_writer(mut writer: *mut MWRITER) {
    MikMod_free(writer as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_delete_mem_reader(mut reader: *mut MREADER) {
    MikMod_free(reader as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_new_mem_reader(
    mut buffer: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_long,
) -> *mut MREADER {
    let mut reader: *mut MMEMREADER = MikMod_calloc(
        1 as size_t,
        ::core::mem::size_of::<MMEMREADER>(),
    ) as *mut MMEMREADER;
    if !reader.is_null() {
        (*reader).core.Eof = Some(
            _mm_MemReader_Eof as unsafe extern "C" fn(*mut MREADER) -> BOOL,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> BOOL>;
        (*reader).core.Read = Some(
            _mm_MemReader_Read
                as unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> BOOL,
            >;
        (*reader).core.Get = Some(
            _mm_MemReader_Get as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_int>;
        (*reader).core.Seek = Some(
            _mm_MemReader_Seek
                as unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut MREADER,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*reader).core.Tell = Some(
            _mm_MemReader_Tell
                as unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long,
        ) as Option<unsafe extern "C" fn(*mut MREADER) -> ::core::ffi::c_long>;
        (*reader).buffer = buffer;
        (*reader).len = len;
        (*reader).pos = 0 as ::core::ffi::c_long;
    }
    return reader as *mut MREADER;
}
unsafe extern "C" fn _mm_MemReader_Eof(mut reader: *mut MREADER) -> BOOL {
    let mut mr: *mut MMEMREADER = reader as *mut MMEMREADER;
    if mr.is_null() {
        return 1 as BOOL;
    }
    if (*mr).pos >= (*mr).len {
        return 1 as BOOL;
    }
    return 0 as BOOL;
}
unsafe extern "C" fn _mm_MemReader_Read(
    mut reader: *mut MREADER,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> BOOL {
    let mut d: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut s: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut mr: *mut MMEMREADER = ::core::ptr::null_mut::<MMEMREADER>();
    let mut siz: ::core::ffi::c_long = 0;
    let mut ret: BOOL = 0;
    if reader.is_null() || size == 0
        || size > 2147483647 as ::core::ffi::c_long as size_t
    {
        return 0 as BOOL;
    }
    mr = reader as *mut MMEMREADER;
    siz = size as ::core::ffi::c_long;
    if (*mr).pos >= (*mr).len {
        return 0 as BOOL;
    }
    if (*mr).pos + siz > (*mr).len {
        siz = (*mr).len - (*mr).pos;
        ret = 0 as ::core::ffi::c_int as BOOL;
    } else {
        ret = 1 as ::core::ffi::c_int as BOOL;
    }
    s = (*mr).buffer as *const ::core::ffi::c_uchar;
    s = s.offset((*mr).pos as isize);
    (*mr).pos += siz;
    d = ptr as *mut ::core::ffi::c_uchar;
    while siz != 0 {
        let c2rust_fresh0 = s;
        s = s.offset(1);
        let c2rust_fresh1 = d;
        d = d.offset(1);
        *c2rust_fresh1 = *c2rust_fresh0;
        siz -= 1;
    }
    return ret;
}
unsafe extern "C" fn _mm_MemReader_Get(mut reader: *mut MREADER) -> ::core::ffi::c_int {
    let mut mr: *mut MMEMREADER = ::core::ptr::null_mut::<MMEMREADER>();
    let mut c: ::core::ffi::c_int = 0;
    mr = reader as *mut MMEMREADER;
    if (*mr).pos >= (*mr).len {
        return -1 as ::core::ffi::c_int;
    }
    c = *((*mr).buffer as *const ::core::ffi::c_uchar).offset((*mr).pos as isize)
        as ::core::ffi::c_int;
    (*mr).pos += 1;
    return c;
}
unsafe extern "C" fn _mm_MemReader_Seek(
    mut reader: *mut MREADER,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mr: *mut MMEMREADER = ::core::ptr::null_mut::<MMEMREADER>();
    if reader.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    mr = reader as *mut MMEMREADER;
    match whence {
        1 => {
            (*mr).pos += offset;
        }
        0 => {
            (*mr).pos = (*reader).iobase + offset;
        }
        2 => {
            (*mr).pos = (*mr).len + offset;
        }
        _ => return -1 as ::core::ffi::c_int,
    }
    if (*mr).pos < (*reader).iobase {
        (*mr).pos = (*mr).core.iobase;
        return -1 as ::core::ffi::c_int;
    }
    if (*mr).pos > (*mr).len {
        (*mr).pos = (*mr).len;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mm_MemReader_Tell(
    mut reader: *mut MREADER,
) -> ::core::ffi::c_long {
    if !reader.is_null() {
        return (*(reader as *mut MMEMREADER)).pos - (*reader).iobase;
    }
    return 0 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_string(
    mut data: *const CHAR,
    mut writer: *mut MWRITER,
) {
    if !data.is_null() {
        (*writer)
            .Write
            .expect(
                "non-null function pointer",
            )(writer, data as *const ::core::ffi::c_void, strlen(data));
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_UWORD(mut data: UWORD, mut writer: *mut MWRITER) {
    (*writer)
        .Put
        .expect(
            "non-null function pointer",
        )(writer, data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int);
    (*writer)
        .Put
        .expect(
            "non-null function pointer",
        )(writer, data as ::core::ffi::c_int & 0xff as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_UWORD(mut data: UWORD, mut writer: *mut MWRITER) {
    (*writer)
        .Put
        .expect(
            "non-null function pointer",
        )(writer, data as ::core::ffi::c_int & 0xff as ::core::ffi::c_int);
    (*writer)
        .Put
        .expect(
            "non-null function pointer",
        )(writer, data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_ULONG(mut data: ULONG, mut writer: *mut MWRITER) {
    _mm_write_M_UWORD((data >> 16 as ::core::ffi::c_int) as UWORD, writer);
    _mm_write_M_UWORD((data & 0xffff as ULONG) as UWORD, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_ULONG(mut data: ULONG, mut writer: *mut MWRITER) {
    _mm_write_I_UWORD((data & 0xffff as ULONG) as UWORD, writer);
    _mm_write_I_UWORD((data >> 16 as ::core::ffi::c_int) as UWORD, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_SWORD(mut data: SWORD, mut writer: *mut MWRITER) {
    _mm_write_M_UWORD(data as UWORD, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_SWORD(mut data: SWORD, mut writer: *mut MWRITER) {
    _mm_write_I_UWORD(data as UWORD, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_SLONG(mut data: SLONG, mut writer: *mut MWRITER) {
    _mm_write_M_ULONG(data as ULONG, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_SLONG(mut data: SLONG, mut writer: *mut MWRITER) {
    _mm_write_I_ULONG(data as ULONG, writer);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_SWORDS(
    mut buffer: *mut SWORD,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh18 = cnt;
        cnt -= 1;
        if c2rust_fresh18 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh19 = buffer;
        buffer = buffer.offset(1);
        _mm_write_M_SWORD(*c2rust_fresh19, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_UWORDS(
    mut buffer: *mut UWORD,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh22 = cnt;
        cnt -= 1;
        if c2rust_fresh22 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh23 = buffer;
        buffer = buffer.offset(1);
        _mm_write_M_UWORD(*c2rust_fresh23, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_SWORDS(
    mut buffer: *mut SWORD,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh20 = cnt;
        cnt -= 1;
        if c2rust_fresh20 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh21 = buffer;
        buffer = buffer.offset(1);
        _mm_write_I_SWORD(*c2rust_fresh21, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_UWORDS(
    mut buffer: *mut UWORD,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh24 = cnt;
        cnt -= 1;
        if c2rust_fresh24 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh25 = buffer;
        buffer = buffer.offset(1);
        _mm_write_I_UWORD(*c2rust_fresh25, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_SLONGS(
    mut buffer: *mut SLONG,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh26 = cnt;
        cnt -= 1;
        if c2rust_fresh26 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh27 = buffer;
        buffer = buffer.offset(1);
        _mm_write_M_SLONG(*c2rust_fresh27, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_M_ULONGS(
    mut buffer: *mut ULONG,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh30 = cnt;
        cnt -= 1;
        if c2rust_fresh30 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh31 = buffer;
        buffer = buffer.offset(1);
        _mm_write_M_ULONG(*c2rust_fresh31, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_SLONGS(
    mut buffer: *mut SLONG,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh28 = cnt;
        cnt -= 1;
        if c2rust_fresh28 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh29 = buffer;
        buffer = buffer.offset(1);
        _mm_write_I_SLONG(*c2rust_fresh29, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_write_I_ULONGS(
    mut buffer: *mut ULONG,
    mut cnt: ::core::ffi::c_int,
    mut writer: *mut MWRITER,
) {
    loop {
        let c2rust_fresh32 = cnt;
        cnt -= 1;
        if c2rust_fresh32 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh33 = buffer;
        buffer = buffer.offset(1);
        _mm_write_I_ULONG(*c2rust_fresh33, writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_string(
    mut buffer: *mut CHAR,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    return (*reader)
        .Read
        .expect(
            "non-null function pointer",
        )(reader, buffer as *mut ::core::ffi::c_void, cnt as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_UWORD(mut reader: *mut MREADER) -> UWORD {
    let mut result: UWORD = (((*reader).Get.expect("non-null function pointer")(reader)
        as UBYTE as UWORD as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD;
    result = (result as ::core::ffi::c_int
        | (*reader).Get.expect("non-null function pointer")(reader) as UBYTE
            as ::core::ffi::c_int) as UWORD;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_UWORD(mut reader: *mut MREADER) -> UWORD {
    let mut result: UWORD = (*reader).Get.expect("non-null function pointer")(reader)
        as UBYTE as UWORD;
    result = (result as ::core::ffi::c_int
        | ((*reader).Get.expect("non-null function pointer")(reader) as UBYTE as UWORD
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_ULONG(mut reader: *mut MREADER) -> ULONG {
    let mut result: ULONG = (_mm_read_M_UWORD(reader) as ULONG)
        << 16 as ::core::ffi::c_int;
    result |= _mm_read_M_UWORD(reader) as ULONG;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_ULONG(mut reader: *mut MREADER) -> ULONG {
    let mut result: ULONG = _mm_read_I_UWORD(reader) as ULONG;
    result |= (_mm_read_I_UWORD(reader) as ULONG) << 16 as ::core::ffi::c_int;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_SWORD(mut reader: *mut MREADER) -> SWORD {
    return _mm_read_M_UWORD(reader) as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_SWORD(mut reader: *mut MREADER) -> SWORD {
    return _mm_read_I_UWORD(reader) as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_SLONG(mut reader: *mut MREADER) -> SLONG {
    return _mm_read_M_ULONG(reader) as SLONG;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_SLONG(mut reader: *mut MREADER) -> SLONG {
    return _mm_read_I_ULONG(reader) as SLONG;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_SWORDS(
    mut buffer: *mut SWORD,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh2 = cnt;
        cnt -= 1;
        if c2rust_fresh2 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh3 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh3 = _mm_read_M_SWORD(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_UWORDS(
    mut buffer: *mut UWORD,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh6 = cnt;
        cnt -= 1;
        if c2rust_fresh6 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh7 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh7 = _mm_read_M_UWORD(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_SWORDS(
    mut buffer: *mut SWORD,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh4 = cnt;
        cnt -= 1;
        if c2rust_fresh4 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh5 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh5 = _mm_read_I_SWORD(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_UWORDS(
    mut buffer: *mut UWORD,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh8 = cnt;
        cnt -= 1;
        if c2rust_fresh8 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh9 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh9 = _mm_read_I_UWORD(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_SLONGS(
    mut buffer: *mut SLONG,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh10 = cnt;
        cnt -= 1;
        if c2rust_fresh10 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh11 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh11 = _mm_read_M_SLONG(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_M_ULONGS(
    mut buffer: *mut ULONG,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh14 = cnt;
        cnt -= 1;
        if c2rust_fresh14 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh15 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh15 = _mm_read_M_ULONG(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_SLONGS(
    mut buffer: *mut SLONG,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh12 = cnt;
        cnt -= 1;
        if c2rust_fresh12 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh13 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh13 = _mm_read_I_SLONG(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mm_read_I_ULONGS(
    mut buffer: *mut ULONG,
    mut cnt: ::core::ffi::c_int,
    mut reader: *mut MREADER,
) -> BOOL {
    loop {
        let c2rust_fresh16 = cnt;
        cnt -= 1;
        if c2rust_fresh16 <= 0 as ::core::ffi::c_int {
            break;
        }
        let c2rust_fresh17 = buffer;
        buffer = buffer.offset(1);
        *c2rust_fresh17 = _mm_read_I_ULONG(reader);
    }
    return ((*reader).Eof.expect("non-null function pointer")(reader) == 0)
        as ::core::ffi::c_int;
}
