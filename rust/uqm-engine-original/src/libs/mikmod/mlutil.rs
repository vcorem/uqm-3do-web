/*	MikMod sound library
	(c) 1998, 1999, 2000, 2001 Miodrag Vallat and others - see file AUTHORS
	for complete list.

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

  Utility functions for the module loader

==============================================================================*/

// Rust translation of src/libs/mikmod/mlutil.c.
extern "C" {
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn MikMod_realloc(
        _: *mut ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    fn UniWriteByte(_: UBYTE);
    fn UniWriteWord(_: UWORD);
    static unioperands: [UWORD; 62];
    static mut of: MODULE;
    fn getlinearperiod(_: UWORD, _: ULONG) -> UWORD;
    fn getfrequency(_: UWORD, _: ULONG) -> ULONG;
}
pub type size_t = ::core::ffi::c_ulong;
pub type CHAR = ::core::ffi::c_char;
pub type BOOL = ::core::ffi::c_int;
pub type SBYTE = ::core::ffi::c_schar;
pub type UBYTE = ::core::ffi::c_uchar;
pub type SWORD = ::core::ffi::c_short;
pub type UWORD = ::core::ffi::c_ushort;
pub type SLONG = ::core::ffi::c_long;
pub type ULONG = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAMPLE {
    pub panning: SWORD,
    pub speed: ULONG,
    pub volume: UBYTE,
    pub inflags: UWORD,
    pub flags: UWORD,
    pub length: ULONG,
    pub loopstart: ULONG,
    pub loopend: ULONG,
    pub susbegin: ULONG,
    pub susend: ULONG,
    pub globvol: UBYTE,
    pub vibflags: UBYTE,
    pub vibtype: UBYTE,
    pub vibsweep: UBYTE,
    pub vibdepth: UBYTE,
    pub vibrate: UBYTE,
    pub samplename: *mut CHAR,
    pub avibpos: UWORD,
    pub divfactor: UBYTE,
    pub seekpos: ULONG,
    pub handle: SWORD,
    pub onfree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENVPT {
    pub pos: SWORD,
    pub val: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INSTRUMENT {
    pub insname: *mut CHAR,
    pub flags: UBYTE,
    pub samplenumber: [UWORD; 120],
    pub samplenote: [UBYTE; 120],
    pub nnatype: UBYTE,
    pub dca: UBYTE,
    pub dct: UBYTE,
    pub globvol: UBYTE,
    pub volfade: UWORD,
    pub panning: SWORD,
    pub pitpansep: UBYTE,
    pub pitpancenter: UBYTE,
    pub rvolvar: UBYTE,
    pub rpanvar: UBYTE,
    pub volflg: UBYTE,
    pub volpts: UBYTE,
    pub volsusbeg: UBYTE,
    pub volsusend: UBYTE,
    pub volbeg: UBYTE,
    pub volend: UBYTE,
    pub volenv: [ENVPT; 32],
    pub panflg: UBYTE,
    pub panpts: UBYTE,
    pub pansusbeg: UBYTE,
    pub pansusend: UBYTE,
    pub panbeg: UBYTE,
    pub panend: UBYTE,
    pub panenv: [ENVPT; 32],
    pub pitflg: UBYTE,
    pub pitpts: UBYTE,
    pub pitsusbeg: UBYTE,
    pub pitsusend: UBYTE,
    pub pitbeg: UBYTE,
    pub pitend: UBYTE,
    pub pitenv: [ENVPT; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP_CONTROL {
    pub main: MP_CHANNEL,
    pub slave: *mut MP_VOICE,
    pub slavechn: UBYTE,
    pub muted: UBYTE,
    pub ultoffset: UWORD,
    pub anote: UBYTE,
    pub oldnote: UBYTE,
    pub ownper: SWORD,
    pub ownvol: SWORD,
    pub dca: UBYTE,
    pub dct: UBYTE,
    pub row: *mut UBYTE,
    pub retrig: SBYTE,
    pub speed: ULONG,
    pub volume: SWORD,
    pub tmpvolume: SWORD,
    pub tmpperiod: UWORD,
    pub wantedperiod: UWORD,
    pub arpmem: UBYTE,
    pub pansspd: UBYTE,
    pub slidespeed: UWORD,
    pub portspeed: UWORD,
    pub s3mtremor: UBYTE,
    pub s3mtronof: UBYTE,
    pub s3mvolslide: UBYTE,
    pub sliding: SBYTE,
    pub s3mrtgspeed: UBYTE,
    pub s3mrtgslide: UBYTE,
    pub glissando: UBYTE,
    pub wavecontrol: UBYTE,
    pub vibpos: SBYTE,
    pub vibspd: UBYTE,
    pub vibdepth: UBYTE,
    pub trmpos: SBYTE,
    pub trmspd: UBYTE,
    pub trmdepth: UBYTE,
    pub fslideupspd: UBYTE,
    pub fslidednspd: UBYTE,
    pub fportupspd: UBYTE,
    pub fportdnspd: UBYTE,
    pub ffportupspd: UBYTE,
    pub ffportdnspd: UBYTE,
    pub hioffset: ULONG,
    pub soffset: UWORD,
    pub sseffect: UBYTE,
    pub ssdata: UBYTE,
    pub chanvolslide: UBYTE,
    pub panbwave: UBYTE,
    pub panbpos: UBYTE,
    pub panbspd: SBYTE,
    pub panbdepth: UBYTE,
    pub newsamp: UWORD,
    pub voleffect: UBYTE,
    pub voldata: UBYTE,
    pub pat_reppos: SWORD,
    pub pat_repcnt: UWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP_VOICE {
    pub main: MP_CHANNEL,
    pub venv: ENVPR,
    pub penv: ENVPR,
    pub cenv: ENVPR,
    pub avibpos: UWORD,
    pub aswppos: UWORD,
    pub totalvol: ULONG,
    pub mflag: BOOL,
    pub masterchn: SWORD,
    pub masterperiod: UWORD,
    pub master: *mut MP_CONTROL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENVPR {
    pub flg: UBYTE,
    pub pts: UBYTE,
    pub susbeg: UBYTE,
    pub susend: UBYTE,
    pub beg: UBYTE,
    pub end: UBYTE,
    pub p: SWORD,
    pub a: UWORD,
    pub b: UWORD,
    pub env: *mut ENVPT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP_CHANNEL {
    pub i: *mut INSTRUMENT,
    pub s: *mut SAMPLE,
    pub sample: UBYTE,
    pub note: UBYTE,
    pub outvolume: SWORD,
    pub chanvol: SBYTE,
    pub fadevol: UWORD,
    pub panning: SWORD,
    pub kick: UBYTE,
    pub kick_flag: UBYTE,
    pub period: UWORD,
    pub nna: UBYTE,
    pub volflg: UBYTE,
    pub panflg: UBYTE,
    pub pitflg: UBYTE,
    pub keyoff: UBYTE,
    pub handle: SWORD,
    pub notedelay: UBYTE,
    pub start: SLONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MODULE {
    pub songname: *mut CHAR,
    pub modtype: *mut CHAR,
    pub comment: *mut CHAR,
    pub flags: UWORD,
    pub numchn: UBYTE,
    pub numvoices: UBYTE,
    pub numpos: UWORD,
    pub numpat: UWORD,
    pub numins: UWORD,
    pub numsmp: UWORD,
    pub instruments: *mut INSTRUMENT,
    pub samples: *mut SAMPLE,
    pub realchn: UBYTE,
    pub totalchn: UBYTE,
    pub reppos: UWORD,
    pub initspeed: UBYTE,
    pub inittempo: UWORD,
    pub initvolume: UBYTE,
    pub panning: [UWORD; 64],
    pub chanvol: [UBYTE; 64],
    pub bpm: UWORD,
    pub sngspd: UWORD,
    pub volume: SWORD,
    pub extspd: BOOL,
    pub panflag: BOOL,
    pub wrap: BOOL,
    pub r#loop: BOOL,
    pub fadeout: BOOL,
    pub patpos: UWORD,
    pub sngpos: SWORD,
    pub sngtime: ULONG,
    pub relspd: SWORD,
    pub numtrk: UWORD,
    pub tracks: *mut *mut UBYTE,
    pub patterns: *mut UWORD,
    pub pattrows: *mut UWORD,
    pub positions: *mut UWORD,
    pub forbid: BOOL,
    pub numrow: UWORD,
    pub vbtick: UWORD,
    pub sngremainder: UWORD,
    pub control: *mut MP_CONTROL,
    pub voice: *mut MP_VOICE,
    pub globalslide: UBYTE,
    pub pat_repcrazy: UBYTE,
    pub patbrk: UWORD,
    pub patdly: UBYTE,
    pub patdly2: UBYTE,
    pub posjmp: SWORD,
    pub bpmlimit: UWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const UNI_NOTE: Self = Self(1);
    pub const UNI_INSTRUMENT: Self = Self(2);
    pub const UNI_PTEFFECT0: Self = Self(3);
    pub const UNI_PTEFFECT1: Self = Self(4);
    pub const UNI_PTEFFECT2: Self = Self(5);
    pub const UNI_PTEFFECT3: Self = Self(6);
    pub const UNI_PTEFFECT4: Self = Self(7);
    pub const UNI_PTEFFECT5: Self = Self(8);
    pub const UNI_PTEFFECT6: Self = Self(9);
    pub const UNI_PTEFFECT7: Self = Self(10);
    pub const UNI_PTEFFECT8: Self = Self(11);
    pub const UNI_PTEFFECT9: Self = Self(12);
    pub const UNI_PTEFFECTA: Self = Self(13);
    pub const UNI_PTEFFECTB: Self = Self(14);
    pub const UNI_PTEFFECTC: Self = Self(15);
    pub const UNI_PTEFFECTD: Self = Self(16);
    pub const UNI_PTEFFECTE: Self = Self(17);
    pub const UNI_PTEFFECTF: Self = Self(18);
    pub const UNI_S3MEFFECTA: Self = Self(19);
    pub const UNI_S3MEFFECTD: Self = Self(20);
    pub const UNI_S3MEFFECTE: Self = Self(21);
    pub const UNI_S3MEFFECTF: Self = Self(22);
    pub const UNI_S3MEFFECTI: Self = Self(23);
    pub const UNI_S3MEFFECTQ: Self = Self(24);
    pub const UNI_S3MEFFECTR: Self = Self(25);
    pub const UNI_S3MEFFECTT: Self = Self(26);
    pub const UNI_S3MEFFECTU: Self = Self(27);
    pub const UNI_KEYOFF: Self = Self(28);
    pub const UNI_KEYFADE: Self = Self(29);
    pub const UNI_VOLEFFECTS: Self = Self(30);
    pub const UNI_XMEFFECT4: Self = Self(31);
    pub const UNI_XMEFFECT6: Self = Self(32);
    pub const UNI_XMEFFECTA: Self = Self(33);
    pub const UNI_XMEFFECTE1: Self = Self(34);
    pub const UNI_XMEFFECTE2: Self = Self(35);
    pub const UNI_XMEFFECTEA: Self = Self(36);
    pub const UNI_XMEFFECTEB: Self = Self(37);
    pub const UNI_XMEFFECTG: Self = Self(38);
    pub const UNI_XMEFFECTH: Self = Self(39);
    pub const UNI_XMEFFECTL: Self = Self(40);
    pub const UNI_XMEFFECTP: Self = Self(41);
    pub const UNI_XMEFFECTX1: Self = Self(42);
    pub const UNI_XMEFFECTX2: Self = Self(43);
    pub const UNI_ITEFFECTG: Self = Self(44);
    pub const UNI_ITEFFECTH: Self = Self(45);
    pub const UNI_ITEFFECTI: Self = Self(46);
    pub const UNI_ITEFFECTM: Self = Self(47);
    pub const UNI_ITEFFECTN: Self = Self(48);
    pub const UNI_ITEFFECTP: Self = Self(49);
    pub const UNI_ITEFFECTT: Self = Self(50);
    pub const UNI_ITEFFECTU: Self = Self(51);
    pub const UNI_ITEFFECTW: Self = Self(52);
    pub const UNI_ITEFFECTY: Self = Self(53);
    pub const UNI_ITEFFECTZ: Self = Self(54);
    pub const UNI_ITEFFECTS0: Self = Self(55);
    pub const UNI_ULTEFFECT9: Self = Self(56);
    pub const UNI_MEDSPEED: Self = Self(57);
    pub const UNI_MEDEFFECTF1: Self = Self(58);
    pub const UNI_MEDEFFECTF2: Self = Self(59);
    pub const UNI_MEDEFFECTF3: Self = Self(60);
    pub const UNI_OKTARP: Self = Self(61);
    pub const UNI_LAST: Self = Self(62);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER {
    pub filter: UBYTE,
    pub inf: UBYTE,
}
#[no_mangle]
pub static mut STM_Signatures: [*const CHAR; 3] = [
    b"!Scream!\0".as_ptr() as *const CHAR,
    b"BMOD2STM\0".as_ptr() as *const CHAR,
    b"WUZAMOD!\0".as_ptr() as *const CHAR,
];
#[no_mangle]
pub static mut STM_Version: [*const CHAR; 3] = [
    b"Screamtracker 2\0".as_ptr() as *const CHAR,
    b"Converted by MOD2STM (STM format)\0".as_ptr() as *const CHAR,
    b"Wuzamod (STM format)\0".as_ptr() as *const CHAR,
];
#[no_mangle]
pub static mut remap: [SBYTE; 64] = [0; 64];
#[no_mangle]
pub static mut poslookup: *mut UBYTE = ::core::ptr::null_mut::<UBYTE>();
#[no_mangle]
pub static mut poslookupcnt: UWORD = 0;
#[no_mangle]
pub static mut origpositions: *mut UWORD = ::core::ptr::null_mut::<UWORD>();
#[no_mangle]
pub static mut filters: BOOL = 0;
#[no_mangle]
pub static mut activemacro: UBYTE = 0;
#[no_mangle]
pub static mut filtermacros: [UBYTE; 16] = [0; 16];
#[no_mangle]
pub static mut filtersettings: [FILTER; 256] = [FILTER { filter: 0, inf: 0 }; 256];
#[no_mangle]
pub static mut noteindex: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
    ::core::ffi::c_int,
>();
static mut noteindexcount: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn AllocLinear() -> *mut ::core::ffi::c_int {
    if of.numsmp as ::core::ffi::c_uint > noteindexcount {
        noteindexcount = of.numsmp as ::core::ffi::c_uint;
        noteindex = MikMod_realloc(
            noteindex as *mut ::core::ffi::c_void,
            (noteindexcount as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
    }
    return noteindex;
}
#[no_mangle]
pub unsafe extern "C" fn FreeLinear() {
    MikMod_free(noteindex as *mut ::core::ffi::c_void);
    noteindex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    noteindexcount = 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn speed_to_finetune(
    mut speed: ULONG,
    mut sample: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctmp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp: ::core::ffi::c_int = 0;
    let mut note: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut ft: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    speed >>= 1 as ::core::ffi::c_int;
    loop {
        tmp = getfrequency(
            of.flags,
            getlinearperiod((note << 1 as ::core::ffi::c_int) as UWORD, 0 as ULONG)
                as ULONG,
        ) as ::core::ffi::c_int;
        if (tmp as ULONG) >= speed {
            break;
        }
        ctmp = tmp;
        note += 1;
    }
    if tmp as ULONG != speed {
        if (tmp as ULONG).wrapping_sub(speed) < speed.wrapping_sub(ctmp as ULONG) {
            while tmp as ULONG > speed {
                ft -= 1;
                tmp = getfrequency(
                    of.flags,
                    getlinearperiod(
                        (note << 1 as ::core::ffi::c_int) as UWORD,
                        ft as ULONG,
                    ) as ULONG,
                ) as ::core::ffi::c_int;
            }
        } else {
            note -= 1;
            while (ctmp as ULONG) < speed {
                ft += 1;
                ctmp = getfrequency(
                    of.flags,
                    getlinearperiod(
                        (note << 1 as ::core::ffi::c_int) as UWORD,
                        ft as ULONG,
                    ) as ULONG,
                ) as ::core::ffi::c_int;
            }
        }
    }
    *noteindex.offset(sample as isize) = note
        - 4 as ::core::ffi::c_int * 12 as ::core::ffi::c_int;
    return ft;
}
#[no_mangle]
pub unsafe extern "C" fn S3MIT_CreateOrders(mut curious: BOOL) {
    let mut t: ::core::ffi::c_int = 0;
    of.numpos = 0 as UWORD;
    memset(
        of.positions as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (poslookupcnt as size_t).wrapping_mul(::core::mem::size_of::<UWORD>() as size_t),
    );
    memset(
        poslookup as *mut ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        256 as size_t,
    );
    t = 0 as ::core::ffi::c_int;
    while t < poslookupcnt as ::core::ffi::c_int {
        let mut order: ::core::ffi::c_int = *origpositions.offset(t as isize)
            as ::core::ffi::c_int;
        if order == 255 as ::core::ffi::c_int {
            order = -1 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int;
        }
        *of.positions.offset(of.numpos as isize) = order as UWORD;
        *poslookup.offset(t as isize) = of.numpos as UBYTE;
        if (*origpositions.offset(t as isize) as ::core::ffi::c_int)
            < 254 as ::core::ffi::c_int
        {
            of.numpos = of.numpos.wrapping_add(1);
        } else if order == -1 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
            && {
                let c2rust_fresh0 = curious;
                curious -= 1;
                c2rust_fresh0 == 0
            }
        {
            break;
        }
        t += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn S3MIT_ProcessCmd(
    mut cmd: UBYTE,
    mut inf: UBYTE,
    mut flags: ::core::ffi::c_uint,
) {
    let mut lo: UBYTE = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    if cmd as ::core::ffi::c_int != 255 as ::core::ffi::c_int {
        match cmd as ::core::ffi::c_int {
            1 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTA.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            2 => {
                if (inf as ::core::ffi::c_int) < poslookupcnt as ::core::ffi::c_int {
                    if (*poslookup.offset(inf as isize) as SBYTE as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                        && *origpositions.offset(inf as isize) as ::core::ffi::c_int
                            != 255 as ::core::ffi::c_int
                    {
                        S3MIT_CreateOrders(1 as BOOL);
                    }
                    if !((*poslookup.offset(inf as isize) as SBYTE as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int)
                    {
                        UniPTEffect(0xb as UBYTE, *poslookup.offset(inf as isize));
                    }
                }
            }
            3 => {
                if flags & 1 as ::core::ffi::c_uint != 0
                    && flags & 2 as ::core::ffi::c_uint == 0
                {
                    UniPTEffect(
                        0xd as UBYTE,
                        ((inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                            * 10 as ::core::ffi::c_int
                            + (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as UBYTE,
                    );
                } else {
                    UniPTEffect(0xd as UBYTE, inf);
                }
            }
            4 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTD.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            5 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTE.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            6 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTF.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            7 => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniPTEffect(0x3 as UBYTE, inf);
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTG.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            8 => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniPTEffect(0x4 as UBYTE, inf);
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTH.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            9 => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniEffect(
                        C2Rust_Unnamed::UNI_S3MEFFECTI.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTI.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            0xa => {
                UniPTEffect(0 as UBYTE, inf);
            }
            0xb => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniPTEffect(0x4 as UBYTE, 0 as UBYTE);
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTH.0 as ::core::ffi::c_int as UWORD,
                        0 as UWORD,
                    );
                }
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTD.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0xc => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniPTEffect(0x3 as UBYTE, 0 as UBYTE);
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTG.0 as ::core::ffi::c_int as UWORD,
                        0 as UWORD,
                    );
                }
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTD.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0xd => {
                UniEffect(
                    C2Rust_Unnamed::UNI_ITEFFECTM.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0xe => {
                UniEffect(
                    C2Rust_Unnamed::UNI_ITEFFECTN.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0xf => {
                UniPTEffect(0x9 as UBYTE, inf);
            }
            0x10 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_ITEFFECTP.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0x11 => {
                UniWriteByte(
                    C2Rust_Unnamed::UNI_S3MEFFECTQ.0 as ::core::ffi::c_int as UBYTE,
                );
                if inf as ::core::ffi::c_int != 0 && lo == 0
                    && flags & 1 as ::core::ffi::c_uint == 0
                {
                    UniWriteByte(1 as UBYTE);
                } else {
                    UniWriteByte(inf);
                }
            }
            0x12 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_S3MEFFECTR.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0x13 => {
                if inf as ::core::ffi::c_int >= 0xf0 as ::core::ffi::c_int {
                    if filters != 0
                        && inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                            != activemacro as ::core::ffi::c_int
                    {
                        activemacro = (inf as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as UBYTE;
                        inf = 0 as UBYTE;
                        while (inf as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                            filtersettings[inf as usize].filter = filtermacros[activemacro
                                as usize];
                            inf = inf.wrapping_add(1);
                        }
                    }
                } else if !(flags & 4 as ::core::ffi::c_uint != 0
                    && inf as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                        == 0xa0 as ::core::ffi::c_int)
                {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTS0.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            0x14 => {
                if inf as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int {
                    UniEffect(
                        C2Rust_Unnamed::UNI_S3MEFFECTT.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                } else if flags & 1 as ::core::ffi::c_uint == 0 {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTT.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            0x15 => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    UniEffect(
                        C2Rust_Unnamed::UNI_S3MEFFECTU.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                } else {
                    UniEffect(
                        C2Rust_Unnamed::UNI_ITEFFECTU.0 as ::core::ffi::c_int as UWORD,
                        inf as UWORD,
                    );
                }
            }
            0x16 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_XMEFFECTG.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0x17 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_ITEFFECTW.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0x18 => {
                if flags & 1 as ::core::ffi::c_uint != 0 {
                    if inf as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
                        UniEffect(
                            C2Rust_Unnamed::UNI_ITEFFECTS0.0 as ::core::ffi::c_int
                                as UWORD,
                            0x91 as UWORD,
                        );
                    } else {
                        UniPTEffect(
                            0x8 as UBYTE,
                            (if inf as ::core::ffi::c_int == 128 as ::core::ffi::c_int {
                                255 as ::core::ffi::c_int
                            } else {
                                (inf as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                            }) as UBYTE,
                        );
                    }
                } else {
                    UniPTEffect(0x8 as UBYTE, inf);
                }
            }
            0x19 => {
                UniEffect(
                    C2Rust_Unnamed::UNI_ITEFFECTY.0 as ::core::ffi::c_int as UWORD,
                    inf as UWORD,
                );
            }
            0x1a => {
                if filtersettings[inf as usize].filter != 0 {
                    UniWriteByte(
                        C2Rust_Unnamed::UNI_ITEFFECTZ.0 as ::core::ffi::c_int as UBYTE,
                    );
                    UniWriteByte(filtersettings[inf as usize].filter);
                    UniWriteByte(filtersettings[inf as usize].inf);
                }
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniEffect(mut eff: UWORD, mut dat: UWORD) {
    if eff == 0
        || eff as ::core::ffi::c_int >= C2Rust_Unnamed::UNI_LAST.0 as ::core::ffi::c_int
    {
        return;
    }
    UniWriteByte(eff as UBYTE);
    if unioperands[eff as usize] as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        UniWriteWord(dat);
    } else {
        UniWriteByte(dat as UBYTE);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UniPTEffect(mut eff: UBYTE, mut dat: UBYTE) {
    if eff as ::core::ffi::c_int != 0 || dat as ::core::ffi::c_int != 0
        || of.flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0
    {
        UniEffect(
            (C2Rust_Unnamed::UNI_PTEFFECT0.0 as ::core::ffi::c_int
                + eff as ::core::ffi::c_int) as UWORD,
            dat as UWORD,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn UniVolEffect(mut eff: UWORD, mut dat: UBYTE) {
    if eff as ::core::ffi::c_int != 0 || dat as ::core::ffi::c_int != 0 {
        UniWriteByte(C2Rust_Unnamed::UNI_VOLEFFECTS.0 as ::core::ffi::c_int as UBYTE);
        UniWriteByte(eff as UBYTE);
        UniWriteByte(dat);
    }
}
