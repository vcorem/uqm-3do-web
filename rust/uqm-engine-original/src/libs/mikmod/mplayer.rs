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

  The Protracker Player Driver

  The protracker driver supports all base Protracker 3.x commands and features.

==============================================================================*/

// Rust translation of src/libs/mikmod/mplayer.c.
extern "C" {
    fn rand() -> ::core::ffi::c_int;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn MikMod_Active() -> BOOL;
    fn MikMod_EnableOutput() -> ::core::ffi::c_int;
    fn MikMod_calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MikMod_free(_: *mut ::core::ffi::c_void);
    fn UniSetRow(_: *mut UBYTE);
    fn UniGetByte() -> UBYTE;
    fn UniGetWord() -> UWORD;
    fn UniFindRow(_: *mut UBYTE, _: UWORD) -> *mut UBYTE;
    fn UniSkipOpcode();
    static finetune: [UWORD; 16];
    static mut md_bpm: UWORD;
    static mut md_sngchn: UBYTE;
    static mut md_sfxchn: UBYTE;
    fn MikMod_DisableOutput_internal();
    fn Voice_Play_internal(_: SBYTE, _: *mut SAMPLE, _: ULONG);
    fn Voice_SetFrequency_internal(_: SBYTE, _: ULONG);
    fn Voice_SetPanning_internal(_: SBYTE, _: ULONG);
    fn Voice_SetVolume_internal(_: SBYTE, _: UWORD);
    fn Voice_Stop_internal(_: SBYTE);
    fn Voice_Stopped_internal(_: SBYTE) -> BOOL;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type size_t = ::core::ffi::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VOICEINFO {
    pub i: *mut INSTRUMENT,
    pub s: *mut SAMPLE,
    pub panning: SWORD,
    pub volume: SBYTE,
    pub period: UWORD,
    pub kick: UBYTE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SS_GLISSANDO: Self = Self(1);
    pub const SS_FINETUNE: Self = Self(2);
    pub const SS_VIBWAVE: Self = Self(3);
    pub const SS_TREMWAVE: Self = Self(4);
    pub const SS_PANWAVE: Self = Self(5);
    pub const SS_FRAMEDELAY: Self = Self(6);
    pub const SS_S7EFFECTS: Self = Self(7);
    pub const SS_PANNING: Self = Self(8);
    pub const SS_SURROUND: Self = Self(9);
    pub const SS_HIOFFSET: Self = Self(10);
    pub const SS_PATLOOP: Self = Self(11);
    pub const SS_NOTECUT: Self = Self(12);
    pub const SS_NOTEDELAY: Self = Self(13);
    pub const SS_PATDELAY: Self = Self(14);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const VOL_VOLUME: Self = Self(1);
    pub const VOL_PANNING: Self = Self(2);
    pub const VOL_VOLSLIDE: Self = Self(3);
    pub const VOL_PITCHSLIDEDN: Self = Self(4);
    pub const VOL_PITCHSLIDEUP: Self = Self(5);
    pub const VOL_PORTAMENTO: Self = Self(6);
    pub const VOL_VIBRATO: Self = Self(7);
}
pub type effect_func = Option<
    unsafe extern "C" fn(
        UWORD,
        UWORD,
        *mut MP_CONTROL,
        *mut MODULE,
        SWORD,
    ) -> ::core::ffi::c_int,
>;
#[no_mangle]
pub static mut pf: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
static mut oldperiods: [UWORD; 24] = [
    0x6b00 as UWORD,
    0x6800 as UWORD,
    0x6500 as UWORD,
    0x6220 as UWORD,
    0x5f50 as UWORD,
    0x5c80 as UWORD,
    0x5a00 as UWORD,
    0x5740 as UWORD,
    0x54d0 as UWORD,
    0x5260 as UWORD,
    0x5010 as UWORD,
    0x4dc0 as UWORD,
    0x4b90 as UWORD,
    0x4960 as UWORD,
    0x4750 as UWORD,
    0x4540 as UWORD,
    0x4350 as UWORD,
    0x4160 as UWORD,
    0x3f90 as UWORD,
    0x3dc0 as UWORD,
    0x3c10 as UWORD,
    0x3a40 as UWORD,
    0x38b0 as UWORD,
    0x3700 as UWORD,
];
static mut VibratoTable: [UBYTE; 32] = [
    0 as UBYTE,
    24 as UBYTE,
    49 as UBYTE,
    74 as UBYTE,
    97 as UBYTE,
    120 as UBYTE,
    141 as UBYTE,
    161 as UBYTE,
    180 as UBYTE,
    197 as UBYTE,
    212 as UBYTE,
    224 as UBYTE,
    235 as UBYTE,
    244 as UBYTE,
    250 as UBYTE,
    253 as UBYTE,
    255 as UBYTE,
    253 as UBYTE,
    250 as UBYTE,
    244 as UBYTE,
    235 as UBYTE,
    224 as UBYTE,
    212 as UBYTE,
    197 as UBYTE,
    180 as UBYTE,
    161 as UBYTE,
    141 as UBYTE,
    120 as UBYTE,
    97 as UBYTE,
    74 as UBYTE,
    49 as UBYTE,
    24 as UBYTE,
];
static mut avibtab: [UBYTE; 128] = [
    0 as UBYTE,
    1 as UBYTE,
    3 as UBYTE,
    4 as UBYTE,
    6 as UBYTE,
    7 as UBYTE,
    9 as UBYTE,
    10 as UBYTE,
    12 as UBYTE,
    14 as UBYTE,
    15 as UBYTE,
    17 as UBYTE,
    18 as UBYTE,
    20 as UBYTE,
    21 as UBYTE,
    23 as UBYTE,
    24 as UBYTE,
    25 as UBYTE,
    27 as UBYTE,
    28 as UBYTE,
    30 as UBYTE,
    31 as UBYTE,
    32 as UBYTE,
    34 as UBYTE,
    35 as UBYTE,
    36 as UBYTE,
    38 as UBYTE,
    39 as UBYTE,
    40 as UBYTE,
    41 as UBYTE,
    42 as UBYTE,
    44 as UBYTE,
    45 as UBYTE,
    46 as UBYTE,
    47 as UBYTE,
    48 as UBYTE,
    49 as UBYTE,
    50 as UBYTE,
    51 as UBYTE,
    52 as UBYTE,
    53 as UBYTE,
    54 as UBYTE,
    54 as UBYTE,
    55 as UBYTE,
    56 as UBYTE,
    57 as UBYTE,
    57 as UBYTE,
    58 as UBYTE,
    59 as UBYTE,
    59 as UBYTE,
    60 as UBYTE,
    60 as UBYTE,
    61 as UBYTE,
    61 as UBYTE,
    62 as UBYTE,
    62 as UBYTE,
    62 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    64 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    63 as UBYTE,
    62 as UBYTE,
    62 as UBYTE,
    62 as UBYTE,
    61 as UBYTE,
    61 as UBYTE,
    60 as UBYTE,
    60 as UBYTE,
    59 as UBYTE,
    59 as UBYTE,
    58 as UBYTE,
    57 as UBYTE,
    57 as UBYTE,
    56 as UBYTE,
    55 as UBYTE,
    54 as UBYTE,
    54 as UBYTE,
    53 as UBYTE,
    52 as UBYTE,
    51 as UBYTE,
    50 as UBYTE,
    49 as UBYTE,
    48 as UBYTE,
    47 as UBYTE,
    46 as UBYTE,
    45 as UBYTE,
    44 as UBYTE,
    42 as UBYTE,
    41 as UBYTE,
    40 as UBYTE,
    39 as UBYTE,
    38 as UBYTE,
    36 as UBYTE,
    35 as UBYTE,
    34 as UBYTE,
    32 as UBYTE,
    31 as UBYTE,
    30 as UBYTE,
    28 as UBYTE,
    27 as UBYTE,
    25 as UBYTE,
    24 as UBYTE,
    23 as UBYTE,
    21 as UBYTE,
    20 as UBYTE,
    18 as UBYTE,
    17 as UBYTE,
    15 as UBYTE,
    14 as UBYTE,
    12 as UBYTE,
    10 as UBYTE,
    9 as UBYTE,
    7 as UBYTE,
    6 as UBYTE,
    4 as UBYTE,
    3 as UBYTE,
    1 as UBYTE,
];
static mut lintab: [ULONG; 768] = [
    535232 as ULONG,
    534749 as ULONG,
    534266 as ULONG,
    533784 as ULONG,
    533303 as ULONG,
    532822 as ULONG,
    532341 as ULONG,
    531861 as ULONG,
    531381 as ULONG,
    530902 as ULONG,
    530423 as ULONG,
    529944 as ULONG,
    529466 as ULONG,
    528988 as ULONG,
    528511 as ULONG,
    528034 as ULONG,
    527558 as ULONG,
    527082 as ULONG,
    526607 as ULONG,
    526131 as ULONG,
    525657 as ULONG,
    525183 as ULONG,
    524709 as ULONG,
    524236 as ULONG,
    523763 as ULONG,
    523290 as ULONG,
    522818 as ULONG,
    522346 as ULONG,
    521875 as ULONG,
    521404 as ULONG,
    520934 as ULONG,
    520464 as ULONG,
    519994 as ULONG,
    519525 as ULONG,
    519057 as ULONG,
    518588 as ULONG,
    518121 as ULONG,
    517653 as ULONG,
    517186 as ULONG,
    516720 as ULONG,
    516253 as ULONG,
    515788 as ULONG,
    515322 as ULONG,
    514858 as ULONG,
    514393 as ULONG,
    513929 as ULONG,
    513465 as ULONG,
    513002 as ULONG,
    512539 as ULONG,
    512077 as ULONG,
    511615 as ULONG,
    511154 as ULONG,
    510692 as ULONG,
    510232 as ULONG,
    509771 as ULONG,
    509312 as ULONG,
    508852 as ULONG,
    508393 as ULONG,
    507934 as ULONG,
    507476 as ULONG,
    507018 as ULONG,
    506561 as ULONG,
    506104 as ULONG,
    505647 as ULONG,
    505191 as ULONG,
    504735 as ULONG,
    504280 as ULONG,
    503825 as ULONG,
    503371 as ULONG,
    502917 as ULONG,
    502463 as ULONG,
    502010 as ULONG,
    501557 as ULONG,
    501104 as ULONG,
    500652 as ULONG,
    500201 as ULONG,
    499749 as ULONG,
    499298 as ULONG,
    498848 as ULONG,
    498398 as ULONG,
    497948 as ULONG,
    497499 as ULONG,
    497050 as ULONG,
    496602 as ULONG,
    496154 as ULONG,
    495706 as ULONG,
    495259 as ULONG,
    494812 as ULONG,
    494366 as ULONG,
    493920 as ULONG,
    493474 as ULONG,
    493029 as ULONG,
    492585 as ULONG,
    492140 as ULONG,
    491696 as ULONG,
    491253 as ULONG,
    490809 as ULONG,
    490367 as ULONG,
    489924 as ULONG,
    489482 as ULONG,
    489041 as ULONG,
    488600 as ULONG,
    488159 as ULONG,
    487718 as ULONG,
    487278 as ULONG,
    486839 as ULONG,
    486400 as ULONG,
    485961 as ULONG,
    485522 as ULONG,
    485084 as ULONG,
    484647 as ULONG,
    484210 as ULONG,
    483773 as ULONG,
    483336 as ULONG,
    482900 as ULONG,
    482465 as ULONG,
    482029 as ULONG,
    481595 as ULONG,
    481160 as ULONG,
    480726 as ULONG,
    480292 as ULONG,
    479859 as ULONG,
    479426 as ULONG,
    478994 as ULONG,
    478562 as ULONG,
    478130 as ULONG,
    477699 as ULONG,
    477268 as ULONG,
    476837 as ULONG,
    476407 as ULONG,
    475977 as ULONG,
    475548 as ULONG,
    475119 as ULONG,
    474690 as ULONG,
    474262 as ULONG,
    473834 as ULONG,
    473407 as ULONG,
    472979 as ULONG,
    472553 as ULONG,
    472126 as ULONG,
    471701 as ULONG,
    471275 as ULONG,
    470850 as ULONG,
    470425 as ULONG,
    470001 as ULONG,
    469577 as ULONG,
    469153 as ULONG,
    468730 as ULONG,
    468307 as ULONG,
    467884 as ULONG,
    467462 as ULONG,
    467041 as ULONG,
    466619 as ULONG,
    466198 as ULONG,
    465778 as ULONG,
    465358 as ULONG,
    464938 as ULONG,
    464518 as ULONG,
    464099 as ULONG,
    463681 as ULONG,
    463262 as ULONG,
    462844 as ULONG,
    462427 as ULONG,
    462010 as ULONG,
    461593 as ULONG,
    461177 as ULONG,
    460760 as ULONG,
    460345 as ULONG,
    459930 as ULONG,
    459515 as ULONG,
    459100 as ULONG,
    458686 as ULONG,
    458272 as ULONG,
    457859 as ULONG,
    457446 as ULONG,
    457033 as ULONG,
    456621 as ULONG,
    456209 as ULONG,
    455797 as ULONG,
    455386 as ULONG,
    454975 as ULONG,
    454565 as ULONG,
    454155 as ULONG,
    453745 as ULONG,
    453336 as ULONG,
    452927 as ULONG,
    452518 as ULONG,
    452110 as ULONG,
    451702 as ULONG,
    451294 as ULONG,
    450887 as ULONG,
    450481 as ULONG,
    450074 as ULONG,
    449668 as ULONG,
    449262 as ULONG,
    448857 as ULONG,
    448452 as ULONG,
    448048 as ULONG,
    447644 as ULONG,
    447240 as ULONG,
    446836 as ULONG,
    446433 as ULONG,
    446030 as ULONG,
    445628 as ULONG,
    445226 as ULONG,
    444824 as ULONG,
    444423 as ULONG,
    444022 as ULONG,
    443622 as ULONG,
    443221 as ULONG,
    442821 as ULONG,
    442422 as ULONG,
    442023 as ULONG,
    441624 as ULONG,
    441226 as ULONG,
    440828 as ULONG,
    440430 as ULONG,
    440033 as ULONG,
    439636 as ULONG,
    439239 as ULONG,
    438843 as ULONG,
    438447 as ULONG,
    438051 as ULONG,
    437656 as ULONG,
    437261 as ULONG,
    436867 as ULONG,
    436473 as ULONG,
    436079 as ULONG,
    435686 as ULONG,
    435293 as ULONG,
    434900 as ULONG,
    434508 as ULONG,
    434116 as ULONG,
    433724 as ULONG,
    433333 as ULONG,
    432942 as ULONG,
    432551 as ULONG,
    432161 as ULONG,
    431771 as ULONG,
    431382 as ULONG,
    430992 as ULONG,
    430604 as ULONG,
    430215 as ULONG,
    429827 as ULONG,
    429439 as ULONG,
    429052 as ULONG,
    428665 as ULONG,
    428278 as ULONG,
    427892 as ULONG,
    427506 as ULONG,
    427120 as ULONG,
    426735 as ULONG,
    426350 as ULONG,
    425965 as ULONG,
    425581 as ULONG,
    425197 as ULONG,
    424813 as ULONG,
    424430 as ULONG,
    424047 as ULONG,
    423665 as ULONG,
    423283 as ULONG,
    422901 as ULONG,
    422519 as ULONG,
    422138 as ULONG,
    421757 as ULONG,
    421377 as ULONG,
    420997 as ULONG,
    420617 as ULONG,
    420237 as ULONG,
    419858 as ULONG,
    419479 as ULONG,
    419101 as ULONG,
    418723 as ULONG,
    418345 as ULONG,
    417968 as ULONG,
    417591 as ULONG,
    417214 as ULONG,
    416838 as ULONG,
    416462 as ULONG,
    416086 as ULONG,
    415711 as ULONG,
    415336 as ULONG,
    414961 as ULONG,
    414586 as ULONG,
    414212 as ULONG,
    413839 as ULONG,
    413465 as ULONG,
    413092 as ULONG,
    412720 as ULONG,
    412347 as ULONG,
    411975 as ULONG,
    411604 as ULONG,
    411232 as ULONG,
    410862 as ULONG,
    410491 as ULONG,
    410121 as ULONG,
    409751 as ULONG,
    409381 as ULONG,
    409012 as ULONG,
    408643 as ULONG,
    408274 as ULONG,
    407906 as ULONG,
    407538 as ULONG,
    407170 as ULONG,
    406803 as ULONG,
    406436 as ULONG,
    406069 as ULONG,
    405703 as ULONG,
    405337 as ULONG,
    404971 as ULONG,
    404606 as ULONG,
    404241 as ULONG,
    403876 as ULONG,
    403512 as ULONG,
    403148 as ULONG,
    402784 as ULONG,
    402421 as ULONG,
    402058 as ULONG,
    401695 as ULONG,
    401333 as ULONG,
    400970 as ULONG,
    400609 as ULONG,
    400247 as ULONG,
    399886 as ULONG,
    399525 as ULONG,
    399165 as ULONG,
    398805 as ULONG,
    398445 as ULONG,
    398086 as ULONG,
    397727 as ULONG,
    397368 as ULONG,
    397009 as ULONG,
    396651 as ULONG,
    396293 as ULONG,
    395936 as ULONG,
    395579 as ULONG,
    395222 as ULONG,
    394865 as ULONG,
    394509 as ULONG,
    394153 as ULONG,
    393798 as ULONG,
    393442 as ULONG,
    393087 as ULONG,
    392733 as ULONG,
    392378 as ULONG,
    392024 as ULONG,
    391671 as ULONG,
    391317 as ULONG,
    390964 as ULONG,
    390612 as ULONG,
    390259 as ULONG,
    389907 as ULONG,
    389556 as ULONG,
    389204 as ULONG,
    388853 as ULONG,
    388502 as ULONG,
    388152 as ULONG,
    387802 as ULONG,
    387452 as ULONG,
    387102 as ULONG,
    386753 as ULONG,
    386404 as ULONG,
    386056 as ULONG,
    385707 as ULONG,
    385359 as ULONG,
    385012 as ULONG,
    384664 as ULONG,
    384317 as ULONG,
    383971 as ULONG,
    383624 as ULONG,
    383278 as ULONG,
    382932 as ULONG,
    382587 as ULONG,
    382242 as ULONG,
    381897 as ULONG,
    381552 as ULONG,
    381208 as ULONG,
    380864 as ULONG,
    380521 as ULONG,
    380177 as ULONG,
    379834 as ULONG,
    379492 as ULONG,
    379149 as ULONG,
    378807 as ULONG,
    378466 as ULONG,
    378124 as ULONG,
    377783 as ULONG,
    377442 as ULONG,
    377102 as ULONG,
    376762 as ULONG,
    376422 as ULONG,
    376082 as ULONG,
    375743 as ULONG,
    375404 as ULONG,
    375065 as ULONG,
    374727 as ULONG,
    374389 as ULONG,
    374051 as ULONG,
    373714 as ULONG,
    373377 as ULONG,
    373040 as ULONG,
    372703 as ULONG,
    372367 as ULONG,
    372031 as ULONG,
    371695 as ULONG,
    371360 as ULONG,
    371025 as ULONG,
    370690 as ULONG,
    370356 as ULONG,
    370022 as ULONG,
    369688 as ULONG,
    369355 as ULONG,
    369021 as ULONG,
    368688 as ULONG,
    368356 as ULONG,
    368023 as ULONG,
    367691 as ULONG,
    367360 as ULONG,
    367028 as ULONG,
    366697 as ULONG,
    366366 as ULONG,
    366036 as ULONG,
    365706 as ULONG,
    365376 as ULONG,
    365046 as ULONG,
    364717 as ULONG,
    364388 as ULONG,
    364059 as ULONG,
    363731 as ULONG,
    363403 as ULONG,
    363075 as ULONG,
    362747 as ULONG,
    362420 as ULONG,
    362093 as ULONG,
    361766 as ULONG,
    361440 as ULONG,
    361114 as ULONG,
    360788 as ULONG,
    360463 as ULONG,
    360137 as ULONG,
    359813 as ULONG,
    359488 as ULONG,
    359164 as ULONG,
    358840 as ULONG,
    358516 as ULONG,
    358193 as ULONG,
    357869 as ULONG,
    357547 as ULONG,
    357224 as ULONG,
    356902 as ULONG,
    356580 as ULONG,
    356258 as ULONG,
    355937 as ULONG,
    355616 as ULONG,
    355295 as ULONG,
    354974 as ULONG,
    354654 as ULONG,
    354334 as ULONG,
    354014 as ULONG,
    353695 as ULONG,
    353376 as ULONG,
    353057 as ULONG,
    352739 as ULONG,
    352420 as ULONG,
    352103 as ULONG,
    351785 as ULONG,
    351468 as ULONG,
    351150 as ULONG,
    350834 as ULONG,
    350517 as ULONG,
    350201 as ULONG,
    349885 as ULONG,
    349569 as ULONG,
    349254 as ULONG,
    348939 as ULONG,
    348624 as ULONG,
    348310 as ULONG,
    347995 as ULONG,
    347682 as ULONG,
    347368 as ULONG,
    347055 as ULONG,
    346741 as ULONG,
    346429 as ULONG,
    346116 as ULONG,
    345804 as ULONG,
    345492 as ULONG,
    345180 as ULONG,
    344869 as ULONG,
    344558 as ULONG,
    344247 as ULONG,
    343936 as ULONG,
    343626 as ULONG,
    343316 as ULONG,
    343006 as ULONG,
    342697 as ULONG,
    342388 as ULONG,
    342079 as ULONG,
    341770 as ULONG,
    341462 as ULONG,
    341154 as ULONG,
    340846 as ULONG,
    340539 as ULONG,
    340231 as ULONG,
    339924 as ULONG,
    339618 as ULONG,
    339311 as ULONG,
    339005 as ULONG,
    338700 as ULONG,
    338394 as ULONG,
    338089 as ULONG,
    337784 as ULONG,
    337479 as ULONG,
    337175 as ULONG,
    336870 as ULONG,
    336566 as ULONG,
    336263 as ULONG,
    335959 as ULONG,
    335656 as ULONG,
    335354 as ULONG,
    335051 as ULONG,
    334749 as ULONG,
    334447 as ULONG,
    334145 as ULONG,
    333844 as ULONG,
    333542 as ULONG,
    333242 as ULONG,
    332941 as ULONG,
    332641 as ULONG,
    332341 as ULONG,
    332041 as ULONG,
    331741 as ULONG,
    331442 as ULONG,
    331143 as ULONG,
    330844 as ULONG,
    330546 as ULONG,
    330247 as ULONG,
    329950 as ULONG,
    329652 as ULONG,
    329355 as ULONG,
    329057 as ULONG,
    328761 as ULONG,
    328464 as ULONG,
    328168 as ULONG,
    327872 as ULONG,
    327576 as ULONG,
    327280 as ULONG,
    326985 as ULONG,
    326690 as ULONG,
    326395 as ULONG,
    326101 as ULONG,
    325807 as ULONG,
    325513 as ULONG,
    325219 as ULONG,
    324926 as ULONG,
    324633 as ULONG,
    324340 as ULONG,
    324047 as ULONG,
    323755 as ULONG,
    323463 as ULONG,
    323171 as ULONG,
    322879 as ULONG,
    322588 as ULONG,
    322297 as ULONG,
    322006 as ULONG,
    321716 as ULONG,
    321426 as ULONG,
    321136 as ULONG,
    320846 as ULONG,
    320557 as ULONG,
    320267 as ULONG,
    319978 as ULONG,
    319690 as ULONG,
    319401 as ULONG,
    319113 as ULONG,
    318825 as ULONG,
    318538 as ULONG,
    318250 as ULONG,
    317963 as ULONG,
    317676 as ULONG,
    317390 as ULONG,
    317103 as ULONG,
    316817 as ULONG,
    316532 as ULONG,
    316246 as ULONG,
    315961 as ULONG,
    315676 as ULONG,
    315391 as ULONG,
    315106 as ULONG,
    314822 as ULONG,
    314538 as ULONG,
    314254 as ULONG,
    313971 as ULONG,
    313688 as ULONG,
    313405 as ULONG,
    313122 as ULONG,
    312839 as ULONG,
    312557 as ULONG,
    312275 as ULONG,
    311994 as ULONG,
    311712 as ULONG,
    311431 as ULONG,
    311150 as ULONG,
    310869 as ULONG,
    310589 as ULONG,
    310309 as ULONG,
    310029 as ULONG,
    309749 as ULONG,
    309470 as ULONG,
    309190 as ULONG,
    308911 as ULONG,
    308633 as ULONG,
    308354 as ULONG,
    308076 as ULONG,
    307798 as ULONG,
    307521 as ULONG,
    307243 as ULONG,
    306966 as ULONG,
    306689 as ULONG,
    306412 as ULONG,
    306136 as ULONG,
    305860 as ULONG,
    305584 as ULONG,
    305308 as ULONG,
    305033 as ULONG,
    304758 as ULONG,
    304483 as ULONG,
    304208 as ULONG,
    303934 as ULONG,
    303659 as ULONG,
    303385 as ULONG,
    303112 as ULONG,
    302838 as ULONG,
    302565 as ULONG,
    302292 as ULONG,
    302019 as ULONG,
    301747 as ULONG,
    301475 as ULONG,
    301203 as ULONG,
    300931 as ULONG,
    300660 as ULONG,
    300388 as ULONG,
    300117 as ULONG,
    299847 as ULONG,
    299576 as ULONG,
    299306 as ULONG,
    299036 as ULONG,
    298766 as ULONG,
    298497 as ULONG,
    298227 as ULONG,
    297958 as ULONG,
    297689 as ULONG,
    297421 as ULONG,
    297153 as ULONG,
    296884 as ULONG,
    296617 as ULONG,
    296349 as ULONG,
    296082 as ULONG,
    295815 as ULONG,
    295548 as ULONG,
    295281 as ULONG,
    295015 as ULONG,
    294749 as ULONG,
    294483 as ULONG,
    294217 as ULONG,
    293952 as ULONG,
    293686 as ULONG,
    293421 as ULONG,
    293157 as ULONG,
    292892 as ULONG,
    292628 as ULONG,
    292364 as ULONG,
    292100 as ULONG,
    291837 as ULONG,
    291574 as ULONG,
    291311 as ULONG,
    291048 as ULONG,
    290785 as ULONG,
    290523 as ULONG,
    290261 as ULONG,
    289999 as ULONG,
    289737 as ULONG,
    289476 as ULONG,
    289215 as ULONG,
    288954 as ULONG,
    288693 as ULONG,
    288433 as ULONG,
    288173 as ULONG,
    287913 as ULONG,
    287653 as ULONG,
    287393 as ULONG,
    287134 as ULONG,
    286875 as ULONG,
    286616 as ULONG,
    286358 as ULONG,
    286099 as ULONG,
    285841 as ULONG,
    285583 as ULONG,
    285326 as ULONG,
    285068 as ULONG,
    284811 as ULONG,
    284554 as ULONG,
    284298 as ULONG,
    284041 as ULONG,
    283785 as ULONG,
    283529 as ULONG,
    283273 as ULONG,
    283017 as ULONG,
    282762 as ULONG,
    282507 as ULONG,
    282252 as ULONG,
    281998 as ULONG,
    281743 as ULONG,
    281489 as ULONG,
    281235 as ULONG,
    280981 as ULONG,
    280728 as ULONG,
    280475 as ULONG,
    280222 as ULONG,
    279969 as ULONG,
    279716 as ULONG,
    279464 as ULONG,
    279212 as ULONG,
    278960 as ULONG,
    278708 as ULONG,
    278457 as ULONG,
    278206 as ULONG,
    277955 as ULONG,
    277704 as ULONG,
    277453 as ULONG,
    277203 as ULONG,
    276953 as ULONG,
    276703 as ULONG,
    276453 as ULONG,
    276204 as ULONG,
    275955 as ULONG,
    275706 as ULONG,
    275457 as ULONG,
    275209 as ULONG,
    274960 as ULONG,
    274712 as ULONG,
    274465 as ULONG,
    274217 as ULONG,
    273970 as ULONG,
    273722 as ULONG,
    273476 as ULONG,
    273229 as ULONG,
    272982 as ULONG,
    272736 as ULONG,
    272490 as ULONG,
    272244 as ULONG,
    271999 as ULONG,
    271753 as ULONG,
    271508 as ULONG,
    271263 as ULONG,
    271018 as ULONG,
    270774 as ULONG,
    270530 as ULONG,
    270286 as ULONG,
    270042 as ULONG,
    269798 as ULONG,
    269555 as ULONG,
    269312 as ULONG,
    269069 as ULONG,
    268826 as ULONG,
    268583 as ULONG,
    268341 as ULONG,
    268099 as ULONG,
    267857 as ULONG,
];
static mut logtab: [UWORD; 104] = [
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 907 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 900 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 894 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 887 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 881 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 875 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 868 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 862 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 856 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 850 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 844 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 838 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 832 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 826 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 820 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 814 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 808 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 802 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 796 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 791 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 785 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 779 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 774 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 768 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 762 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 757 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 752 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 746 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 741 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 736 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 730 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 725 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 720 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 715 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 709 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 704 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 699 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 694 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 689 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 684 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 678 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 675 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 670 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 665 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 660 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 655 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 651 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 646 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 640 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 636 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 632 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 628 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 623 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 619 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 614 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 610 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 604 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 601 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 597 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 592 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 588 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 584 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 580 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 575 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 570 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 567 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 563 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 559 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 555 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 551 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 547 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 543 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 538 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 535 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 532 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 528 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 524 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 520 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 516 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 513 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 508 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 505 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 502 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 498 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 494 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 491 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 487 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 484 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 480 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 477 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 474 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 470 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 467 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 463 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 460 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 457 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 453 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 450 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 447 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 443 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 440 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 437 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 434 as ::core::ffi::c_int)
        as UWORD,
    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * 431 as ::core::ffi::c_int)
        as UWORD,
];
static mut PanbrelloTable: [SBYTE; 256] = [
    0 as SBYTE,
    2 as SBYTE,
    3 as SBYTE,
    5 as SBYTE,
    6 as SBYTE,
    8 as SBYTE,
    9 as SBYTE,
    11 as SBYTE,
    12 as SBYTE,
    14 as SBYTE,
    16 as SBYTE,
    17 as SBYTE,
    19 as SBYTE,
    20 as SBYTE,
    22 as SBYTE,
    23 as SBYTE,
    24 as SBYTE,
    26 as SBYTE,
    27 as SBYTE,
    29 as SBYTE,
    30 as SBYTE,
    32 as SBYTE,
    33 as SBYTE,
    34 as SBYTE,
    36 as SBYTE,
    37 as SBYTE,
    38 as SBYTE,
    39 as SBYTE,
    41 as SBYTE,
    42 as SBYTE,
    43 as SBYTE,
    44 as SBYTE,
    45 as SBYTE,
    46 as SBYTE,
    47 as SBYTE,
    48 as SBYTE,
    49 as SBYTE,
    50 as SBYTE,
    51 as SBYTE,
    52 as SBYTE,
    53 as SBYTE,
    54 as SBYTE,
    55 as SBYTE,
    56 as SBYTE,
    56 as SBYTE,
    57 as SBYTE,
    58 as SBYTE,
    59 as SBYTE,
    59 as SBYTE,
    60 as SBYTE,
    60 as SBYTE,
    61 as SBYTE,
    61 as SBYTE,
    62 as SBYTE,
    62 as SBYTE,
    62 as SBYTE,
    63 as SBYTE,
    63 as SBYTE,
    63 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    64 as SBYTE,
    63 as SBYTE,
    63 as SBYTE,
    63 as SBYTE,
    62 as SBYTE,
    62 as SBYTE,
    62 as SBYTE,
    61 as SBYTE,
    61 as SBYTE,
    60 as SBYTE,
    60 as SBYTE,
    59 as SBYTE,
    59 as SBYTE,
    58 as SBYTE,
    57 as SBYTE,
    56 as SBYTE,
    56 as SBYTE,
    55 as SBYTE,
    54 as SBYTE,
    53 as SBYTE,
    52 as SBYTE,
    51 as SBYTE,
    50 as SBYTE,
    49 as SBYTE,
    48 as SBYTE,
    47 as SBYTE,
    46 as SBYTE,
    45 as SBYTE,
    44 as SBYTE,
    43 as SBYTE,
    42 as SBYTE,
    41 as SBYTE,
    39 as SBYTE,
    38 as SBYTE,
    37 as SBYTE,
    36 as SBYTE,
    34 as SBYTE,
    33 as SBYTE,
    32 as SBYTE,
    30 as SBYTE,
    29 as SBYTE,
    27 as SBYTE,
    26 as SBYTE,
    24 as SBYTE,
    23 as SBYTE,
    22 as SBYTE,
    20 as SBYTE,
    19 as SBYTE,
    17 as SBYTE,
    16 as SBYTE,
    14 as SBYTE,
    12 as SBYTE,
    11 as SBYTE,
    9 as SBYTE,
    8 as SBYTE,
    6 as SBYTE,
    5 as SBYTE,
    3 as SBYTE,
    2 as SBYTE,
    0 as SBYTE,
    -2 as SBYTE,
    -3 as SBYTE,
    -5 as SBYTE,
    -6 as SBYTE,
    -8 as SBYTE,
    -9 as SBYTE,
    -11 as SBYTE,
    -12 as SBYTE,
    -14 as SBYTE,
    -16 as SBYTE,
    -17 as SBYTE,
    -19 as SBYTE,
    -20 as SBYTE,
    -22 as SBYTE,
    -23 as SBYTE,
    -24 as SBYTE,
    -26 as SBYTE,
    -27 as SBYTE,
    -29 as SBYTE,
    -30 as SBYTE,
    -32 as SBYTE,
    -33 as SBYTE,
    -34 as SBYTE,
    -36 as SBYTE,
    -37 as SBYTE,
    -38 as SBYTE,
    -39 as SBYTE,
    -41 as SBYTE,
    -42 as SBYTE,
    -43 as SBYTE,
    -44 as SBYTE,
    -45 as SBYTE,
    -46 as SBYTE,
    -47 as SBYTE,
    -48 as SBYTE,
    -49 as SBYTE,
    -50 as SBYTE,
    -51 as SBYTE,
    -52 as SBYTE,
    -53 as SBYTE,
    -54 as SBYTE,
    -55 as SBYTE,
    -56 as SBYTE,
    -56 as SBYTE,
    -57 as SBYTE,
    -58 as SBYTE,
    -59 as SBYTE,
    -59 as SBYTE,
    -60 as SBYTE,
    -60 as SBYTE,
    -61 as SBYTE,
    -61 as SBYTE,
    -62 as SBYTE,
    -62 as SBYTE,
    -62 as SBYTE,
    -63 as SBYTE,
    -63 as SBYTE,
    -63 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -64 as SBYTE,
    -63 as SBYTE,
    -63 as SBYTE,
    -63 as SBYTE,
    -62 as SBYTE,
    -62 as SBYTE,
    -62 as SBYTE,
    -61 as SBYTE,
    -61 as SBYTE,
    -60 as SBYTE,
    -60 as SBYTE,
    -59 as SBYTE,
    -59 as SBYTE,
    -58 as SBYTE,
    -57 as SBYTE,
    -56 as SBYTE,
    -56 as SBYTE,
    -55 as SBYTE,
    -54 as SBYTE,
    -53 as SBYTE,
    -52 as SBYTE,
    -51 as SBYTE,
    -50 as SBYTE,
    -49 as SBYTE,
    -48 as SBYTE,
    -47 as SBYTE,
    -46 as SBYTE,
    -45 as SBYTE,
    -44 as SBYTE,
    -43 as SBYTE,
    -42 as SBYTE,
    -41 as SBYTE,
    -39 as SBYTE,
    -38 as SBYTE,
    -37 as SBYTE,
    -36 as SBYTE,
    -34 as SBYTE,
    -33 as SBYTE,
    -32 as SBYTE,
    -30 as SBYTE,
    -29 as SBYTE,
    -27 as SBYTE,
    -26 as SBYTE,
    -24 as SBYTE,
    -23 as SBYTE,
    -22 as SBYTE,
    -20 as SBYTE,
    -19 as SBYTE,
    -17 as SBYTE,
    -16 as SBYTE,
    -14 as SBYTE,
    -12 as SBYTE,
    -11 as SBYTE,
    -9 as SBYTE,
    -8 as SBYTE,
    -6 as SBYTE,
    -5 as SBYTE,
    -3 as SBYTE,
    -2 as SBYTE,
];
unsafe extern "C" fn getrandom(mut ceilval: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return ((rand() * ceilval) as ::core::ffi::c_double
        / (0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_double + 1.0f64))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn MP_FindEmptyChannel(mut r#mod: *mut MODULE) -> ::core::ffi::c_int {
    let mut a: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
    let mut t: ULONG = 0;
    let mut k: ULONG = 0;
    let mut tvol: ULONG = 0;
    let mut pp: ULONG = 0;
    t = 0 as ULONG;
    while t
        < (if (md_sngchn as ::core::ffi::c_int)
            < (*r#mod).numvoices as ::core::ffi::c_int
        {
            md_sngchn as ::core::ffi::c_int
        } else {
            (*r#mod).numvoices as ::core::ffi::c_int
        }) as ULONG
    {
        if ((*(*r#mod).voice.offset(t as isize)).main.kick as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (*(*r#mod).voice.offset(t as isize)).main.kick as ::core::ffi::c_int
                == 4 as ::core::ffi::c_int) && Voice_Stopped_internal(t as SBYTE) != 0
        {
            return t as ::core::ffi::c_int;
        }
        t = t.wrapping_add(1);
    }
    tvol = 0xffffff as ::core::ffi::c_ulong as ULONG;
    t = -1 as ::core::ffi::c_int as ULONG;
    a = (*r#mod).voice as *mut MP_VOICE;
    k = 0 as ULONG;
    while k
        < (if (md_sngchn as ::core::ffi::c_int)
            < (*r#mod).numvoices as ::core::ffi::c_int
        {
            md_sngchn as ::core::ffi::c_int
        } else {
            (*r#mod).numvoices as ::core::ffi::c_int
        }) as ULONG
    {
        if (*a).main.s.is_null() {
            return k as ::core::ffi::c_int;
        }
        if (*a).main.kick as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*a).main.kick as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        {
            pp = (*a).totalvol
                << if (*(*a).main.s).flags as ::core::ffi::c_int
                    & 0x100 as ::core::ffi::c_int != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            if !(*a).master.is_null() && a == (*(*a).master).slave {
                pp <<= 2 as ::core::ffi::c_int;
            }
            if pp < tvol {
                tvol = pp;
                t = k;
            }
        }
        k = k.wrapping_add(1);
        a = a.offset(1);
    }
    if tvol > (8000 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as ULONG {
        return -1 as ::core::ffi::c_int;
    }
    return t as ::core::ffi::c_int;
}
unsafe extern "C" fn Interpolate(
    mut p: SWORD,
    mut p1: SWORD,
    mut p2: SWORD,
    mut v1: SWORD,
    mut v2: SWORD,
) -> SWORD {
    if p1 as ::core::ffi::c_int == p2 as ::core::ffi::c_int
        || p as ::core::ffi::c_int == p1 as ::core::ffi::c_int
    {
        return v1;
    }
    return (v1 as SLONG
        + ((p as ::core::ffi::c_int - p1 as ::core::ffi::c_int)
            * (v2 as ::core::ffi::c_int - v1 as ::core::ffi::c_int)) as SLONG
            / (p2 as ::core::ffi::c_int - p1 as ::core::ffi::c_int) as SLONG) as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getlinearperiod(mut note: UWORD, mut fine: ULONG) -> UWORD {
    let mut t: UWORD = 0;
    t = ((((20 as ::core::ffi::c_long
        + (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        * 12 as ::core::ffi::c_long + 2 as ::core::ffi::c_long
        - note as ::core::ffi::c_long) * 32 as ::core::ffi::c_long) as ULONG)
        .wrapping_sub(fine >> 1 as ::core::ffi::c_int) as UWORD;
    return t;
}
unsafe extern "C" fn getlogperiod(mut note: UWORD, mut fine: ULONG) -> UWORD {
    let mut n: UWORD = 0;
    let mut o: UWORD = 0;
    let mut p1: UWORD = 0;
    let mut p2: UWORD = 0;
    let mut i: ULONG = 0;
    n = (note as ::core::ffi::c_int
        % (2 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)) as UWORD;
    o = (note as ::core::ffi::c_int
        / (2 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)) as UWORD;
    i = (((n as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ULONG)
        .wrapping_add(fine >> 4 as ::core::ffi::c_int);
    p1 = logtab[i as usize];
    p2 = logtab[i.wrapping_add(1 as ULONG) as usize];
    return (Interpolate(
        (fine >> 4 as ::core::ffi::c_int) as SWORD,
        0 as SWORD,
        15 as SWORD,
        p1 as SWORD,
        p2 as SWORD,
    ) as ::core::ffi::c_int >> o as ::core::ffi::c_int) as UWORD;
}
unsafe extern "C" fn getoldperiod(mut note: UWORD, mut speed: ULONG) -> UWORD {
    let mut n: UWORD = 0;
    let mut o: UWORD = 0;
    if speed == 0 {
        return 4242 as UWORD;
    }
    n = (note as ::core::ffi::c_int
        % (2 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)) as UWORD;
    o = (note as ::core::ffi::c_int
        / (2 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)) as UWORD;
    return ((8363 as ULONG).wrapping_mul(oldperiods[n as usize] as ULONG)
        >> o as ::core::ffi::c_int)
        .wrapping_div(speed) as UWORD;
}
unsafe extern "C" fn GetPeriod(
    mut flags: UWORD,
    mut note: UWORD,
    mut speed: ULONG,
) -> UWORD {
    if flags as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        if flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
            return getlinearperiod(note, speed)
        } else {
            return getlogperiod(note, speed)
        }
    } else {
        return getoldperiod(note, speed)
    };
}
unsafe extern "C" fn InterpolateEnv(
    mut p: SWORD,
    mut a: *mut ENVPT,
    mut b: *mut ENVPT,
) -> SWORD {
    return Interpolate(p, (*a).pos, (*b).pos, (*a).val, (*b).val);
}
unsafe extern "C" fn DoPan(mut envpan: SWORD, mut pan: SWORD) -> SWORD {
    let mut newpan: ::core::ffi::c_int = 0;
    newpan = pan as ::core::ffi::c_int
        + (envpan as ::core::ffi::c_int - 128 as ::core::ffi::c_int)
            * (128 as ::core::ffi::c_int
                - abs(pan as ::core::ffi::c_int - 128 as ::core::ffi::c_int))
            / 128 as ::core::ffi::c_int;
    return (if newpan < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else if newpan > 255 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else {
        newpan
    }) as SWORD;
}
unsafe extern "C" fn StartEnvelope(
    mut t: *mut ENVPR,
    mut flg: UBYTE,
    mut pts: UBYTE,
    mut susbeg: UBYTE,
    mut susend: UBYTE,
    mut beg: UBYTE,
    mut end: UBYTE,
    mut p: *mut ENVPT,
    mut keyoff: UBYTE,
) -> SWORD {
    (*t).flg = flg;
    (*t).pts = pts;
    (*t).susbeg = susbeg;
    (*t).susend = susend;
    (*t).beg = beg;
    (*t).end = end;
    (*t).env = p;
    (*t).p = 0 as SWORD;
    (*t).a = 0 as UWORD;
    (*t).b = (if (*t).flg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
        && keyoff as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as UWORD;
    if (*t).pts as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
        && (*(*t).env.offset(0isize)).pos as ::core::ffi::c_int
            == (*(*t).env.offset(1isize)).pos as ::core::ffi::c_int
    {
        (*t).a = (*t).a.wrapping_add(1);
        (*t).b = (*t).b.wrapping_add(1);
    }
    if (*t).a as ::core::ffi::c_int >= (*t).pts as ::core::ffi::c_int {
        (*t).a = ((*t).pts as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
    }
    if (*t).b as ::core::ffi::c_int >= (*t).pts as ::core::ffi::c_int {
        (*t).b = ((*t).pts as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UWORD;
    }
    return (*(*t).env.offset((*t).a as isize)).val;
}
unsafe extern "C" fn ProcessEnvelope(
    mut aout: *mut MP_VOICE,
    mut t: *mut ENVPR,
    mut v: SWORD,
) -> SWORD {
    if (*t).flg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
        let mut a: UBYTE = 0;
        let mut b: UBYTE = 0;
        let mut p: UWORD = 0;
        a = (*t).a as UBYTE;
        b = (*t).b as UBYTE;
        p = (*t).p as UWORD;
        if (*t).flg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
            && (*t).susbeg as ::core::ffi::c_int == (*t).susend as ::core::ffi::c_int
            && ((*aout).main.keyoff as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                && p as ::core::ffi::c_int
                    == (*(*t).env.offset((*t).susbeg as isize)).pos
                        as ::core::ffi::c_int)
        {
            v = (*(*t).env.offset((*t).susbeg as isize)).val;
        } else {
            if (*t).flg as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
                && (*aout).main.keyoff as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    == 0 && a as ::core::ffi::c_int >= (*t).susend as ::core::ffi::c_int
            {
                a = (*t).susbeg;
                b = (if (*t).susbeg as ::core::ffi::c_int
                    == (*t).susend as ::core::ffi::c_int
                {
                    a as ::core::ffi::c_int
                } else {
                    a as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                }) as UBYTE;
                p = (*(*t).env.offset(a as isize)).pos as UWORD;
                v = (*(*t).env.offset(a as isize)).val;
            } else if (*t).flg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0
                && a as ::core::ffi::c_int >= (*t).end as ::core::ffi::c_int
            {
                a = (*t).beg;
                b = (if (*t).beg as ::core::ffi::c_int == (*t).end as ::core::ffi::c_int
                {
                    a as ::core::ffi::c_int
                } else {
                    a as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                }) as UBYTE;
                p = (*(*t).env.offset(a as isize)).pos as UWORD;
                v = (*(*t).env.offset(a as isize)).val;
            } else if a as ::core::ffi::c_int != b as ::core::ffi::c_int {
                v = InterpolateEnv(
                    p as SWORD,
                    (*t).env.offset(a as isize),
                    (*t).env.offset(b as isize),
                );
            } else {
                v = (*(*t).env.offset(a as isize)).val;
            }
            if p as ::core::ffi::c_int
                >= (*(*t)
                    .env
                    .offset(
                        ((*t).pts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .pos as ::core::ffi::c_int
            {
                if (*t).flg as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
                    (*aout).main.keyoff = ((*aout).main.keyoff as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int) as UBYTE;
                    if v == 0 {
                        (*aout).main.fadevol = 0 as UWORD;
                    }
                }
            } else {
                p = p.wrapping_add(1);
                if p as ::core::ffi::c_int
                    >= (*(*t).env.offset(b as isize)).pos as ::core::ffi::c_int
                {
                    let c2rust_fresh0 = b;
                    b = b.wrapping_add(1);
                    a = c2rust_fresh0;
                }
            }
            (*t).a = a as UWORD;
            (*t).b = b as UWORD;
            (*t).p = p as SWORD;
        }
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn getfrequency(mut flags: UWORD, mut period: ULONG) -> ULONG {
    if flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
        let mut shift: SLONG = period as SLONG / 768 as SLONG - 2 as SLONG;
        if shift >= 0 as SLONG {
            return lintab[period.wrapping_rem(768 as ULONG) as usize] >> shift
        } else {
            return lintab[period.wrapping_rem(768 as ULONG) as usize] << -shift
        }
    } else {
        return ((8363 as ::core::ffi::c_long * 1712 as ::core::ffi::c_long) as ULONG)
            .wrapping_div(if period != 0 { period } else { 1 as ULONG })
    };
}
unsafe extern "C" fn DoArpeggio(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut style: UBYTE,
) {
    let mut note: UBYTE = (*a).main.note;
    if (*a).arpmem != 0 {
        match style as ::core::ffi::c_int {
            0 => {
                match tick as ::core::ffi::c_int % 3 as ::core::ffi::c_int {
                    1 => {
                        note = (note as ::core::ffi::c_int
                            + ((*a).arpmem as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int)) as UBYTE;
                    }
                    2 => {
                        note = (note as ::core::ffi::c_int
                            + ((*a).arpmem as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int)) as UBYTE;
                    }
                    _ => {}
                }
            }
            3 => {
                match tick as ::core::ffi::c_int % 3 as ::core::ffi::c_int {
                    0 => {
                        note = (note as ::core::ffi::c_int
                            - ((*a).arpmem as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int)) as UBYTE;
                    }
                    2 => {
                        note = (note as ::core::ffi::c_int
                            + ((*a).arpmem as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int)) as UBYTE;
                    }
                    _ => {}
                }
            }
            4 => {
                match tick as ::core::ffi::c_int % 4 as ::core::ffi::c_int {
                    1 => {
                        note = (note as ::core::ffi::c_int
                            + ((*a).arpmem as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int)) as UBYTE;
                    }
                    3 => {
                        note = (note as ::core::ffi::c_int
                            - ((*a).arpmem as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int)) as UBYTE;
                    }
                    _ => {}
                }
            }
            5 => {
                if tick != 0 {
                    match tick as ::core::ffi::c_int % 3 as ::core::ffi::c_int {
                        1 => {
                            note = (note as ::core::ffi::c_int
                                - ((*a).arpmem as ::core::ffi::c_int
                                    >> 4 as ::core::ffi::c_int)) as UBYTE;
                        }
                        2 => {
                            note = (note as ::core::ffi::c_int
                                + ((*a).arpmem as ::core::ffi::c_int
                                    & 0xf as ::core::ffi::c_int)) as UBYTE;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        (*a).main.period = GetPeriod(
            flags,
            ((note as UWORD as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as UWORD,
            (*a).speed,
        );
        (*a).ownper = 1 as SWORD;
    }
}
unsafe extern "C" fn DoPTEffect0(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat == 0 && flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
            dat = (*a).arpmem;
        } else {
            (*a).arpmem = dat;
        }
    }
    if (*a).main.period != 0 {
        DoArpeggio(tick, flags, a, 0 as UBYTE);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect1(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 && dat as ::core::ffi::c_int != 0 {
        (*a).slidespeed = ((dat as UWORD as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as UWORD;
    }
    if (*a).main.period != 0 {
        if tick != 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - (*a).slidespeed as ::core::ffi::c_int) as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect2(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 && dat as ::core::ffi::c_int != 0 {
        (*a).slidespeed = ((dat as UWORD as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as UWORD;
    }
    if (*a).main.period != 0 {
        if tick != 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + (*a).slidespeed as ::core::ffi::c_int) as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoToneSlide(mut tick: UWORD, mut a: *mut MP_CONTROL) {
    if (*a).main.fadevol == 0 {
        (*a).main.kick = (if (*a).main.kick as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as UBYTE;
    } else {
        (*a).main.kick = (if (*a).main.kick as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as UBYTE;
    }
    if tick as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut dist: ::core::ffi::c_int = 0;
        dist = (*a).main.period as ::core::ffi::c_int
            - (*a).wantedperiod as ::core::ffi::c_int;
        if dist == 0 as ::core::ffi::c_int
            || (*a).portspeed as ::core::ffi::c_int > abs(dist)
        {
            (*a).main.period = (*a).wantedperiod;
            (*a).tmpperiod = (*a).main.period;
        } else if dist > 0 as ::core::ffi::c_int {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - (*a).portspeed as ::core::ffi::c_int) as UWORD;
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                - (*a).portspeed as ::core::ffi::c_int) as UWORD;
        } else {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + (*a).portspeed as ::core::ffi::c_int) as UWORD;
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                + (*a).portspeed as ::core::ffi::c_int) as UWORD;
        }
    } else {
        (*a).tmpperiod = (*a).main.period;
    }
    (*a).ownper = 1 as SWORD;
}
unsafe extern "C" fn DoPTEffect3(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 && dat as ::core::ffi::c_int != 0 {
        (*a).portspeed = ((dat as UWORD as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as UWORD;
    }
    if (*a).main.period != 0 {
        DoToneSlide(tick, a);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoVibrato(mut tick: UWORD, mut a: *mut MP_CONTROL) {
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    if tick == 0 {
        return;
    }
    q = ((*a).vibpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int) as UBYTE;
    match (*a).wavecontrol as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
        0 => {
            temp = VibratoTable[q as usize] as UWORD;
        }
        1 => {
            q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
            if ((*a).vibpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
            }
            temp = q as UWORD;
        }
        2 => {
            temp = 255 as UWORD;
        }
        3 => {
            temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
        }
        _ => {}
    }
    temp = (temp as ::core::ffi::c_int * (*a).vibdepth as ::core::ffi::c_int) as UWORD;
    temp = (temp as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as UWORD;
    temp = ((temp as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD;
    if (*a).vibpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
            + temp as ::core::ffi::c_int) as UWORD;
    } else {
        (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
            - temp as ::core::ffi::c_int) as UWORD;
    }
    (*a).ownper = 1 as SWORD;
    if tick as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        (*a).vibpos = ((*a).vibpos as ::core::ffi::c_int
            + (*a).vibspd as ::core::ffi::c_int) as SBYTE;
    }
}
unsafe extern "C" fn DoPTEffect4(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).vibdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).vibspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    }
    if (*a).main.period != 0 {
        DoVibrato(tick, a);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoVolSlide(mut a: *mut MP_CONTROL, mut dat: UBYTE) {
    if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
        (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
            - (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)) as SWORD;
        if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*a).tmpvolume = 0 as SWORD;
        }
    } else {
        (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
            + (dat as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as SWORD;
        if (*a).tmpvolume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
            (*a).tmpvolume = 64 as SWORD;
        }
    };
}
unsafe extern "C" fn DoPTEffect5(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if (*a).main.period != 0 {
        DoToneSlide(tick, a);
    }
    if tick != 0 {
        DoVolSlide(a, dat);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect7(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).trmdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).trmspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    }
    if (*a).main.period != 0 {
        q = ((*a).trmpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) as UBYTE;
        match (*a).wavecontrol as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int
        {
            0 => {
                temp = VibratoTable[q as usize] as UWORD;
            }
            1 => {
                q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
                if ((*a).trmpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
                }
                temp = q as UWORD;
            }
            2 => {
                temp = 255 as UWORD;
            }
            3 => {
                temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
            }
            _ => {}
        }
        temp = (temp as ::core::ffi::c_int * (*a).trmdepth as ::core::ffi::c_int)
            as UWORD;
        temp = (temp as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as UWORD;
        if (*a).trmpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            (*a).volume = ((*a).tmpvolume as ::core::ffi::c_int
                + temp as ::core::ffi::c_int) as SWORD;
            if (*a).volume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                (*a).volume = 64 as SWORD;
            }
        } else {
            (*a).volume = ((*a).tmpvolume as ::core::ffi::c_int
                - temp as ::core::ffi::c_int) as SWORD;
            if ((*a).volume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                (*a).volume = 0 as SWORD;
            }
        }
        (*a).ownvol = 1 as SWORD;
        if tick != 0 {
            (*a).trmpos = ((*a).trmpos as ::core::ffi::c_int
                + (*a).trmspd as ::core::ffi::c_int) as SBYTE;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect8(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if (*r#mod).panflag != 0 {
        (*r#mod).panning[channel as usize] = dat as UWORD;
        (*a).main.panning = (*r#mod).panning[channel as usize] as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect9(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat != 0 {
            (*a).soffset = ((dat as UWORD as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as UWORD;
        }
        (*a).main.start = ((*a).hioffset | (*a).soffset as ULONG) as SLONG;
        if !(*a).main.s.is_null() && (*a).main.start as ULONG > (*(*a).main.s).length {
            (*a).main.start = (if (*(*a).main.s).flags as ::core::ffi::c_int
                & (0x100 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) != 0
            {
                (*(*a).main.s).loopstart
            } else {
                (*(*a).main.s).length
            }) as SLONG;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffectA(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick != 0 {
        DoVolSlide(a, dat);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffect6(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    if (*a).main.period != 0 {
        DoVibrato(tick, a);
    }
    DoPTEffectA(tick, flags, a, r#mod, channel);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffectB(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick as ::core::ffi::c_int != 0 || (*r#mod).patdly2 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if dat as ::core::ffi::c_int == (*r#mod).sngpos as ::core::ffi::c_int
        && (*r#mod).patbrk as ::core::ffi::c_int == (*r#mod).patpos as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*r#mod).r#loop == 0 && (*r#mod).patbrk == 0
        && ((dat as ::core::ffi::c_int) < (*r#mod).sngpos as ::core::ffi::c_int
            || (*r#mod).sngpos as ::core::ffi::c_int
                == (*r#mod).numpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                && (*r#mod).patbrk == 0
            || dat as ::core::ffi::c_int == (*r#mod).sngpos as ::core::ffi::c_int
                && flags as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0)
    {
        (*r#mod).posjmp = 3 as SWORD;
    } else {
        if (*r#mod).sngpos as ::core::ffi::c_int
            == (*r#mod).numpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            (*r#mod).volume = (if (*r#mod).initvolume as ::core::ffi::c_int
                > 128 as ::core::ffi::c_int
            {
                128 as ::core::ffi::c_int
            } else {
                (*r#mod).initvolume as ::core::ffi::c_int
            }) as SWORD;
        }
        (*r#mod).sngpos = dat as SWORD;
        (*r#mod).posjmp = 2 as SWORD;
        (*r#mod).patpos = 0 as UWORD;
        if flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0 {
            (*r#mod).patbrk = 0 as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffectC(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if dat as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as UBYTE as ::core::ffi::c_int
    {
        dat = 0 as UBYTE;
        (*a).anote = dat;
    } else if dat as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        dat = 64 as UBYTE;
    }
    (*a).tmpvolume = dat as SWORD;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffectD(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick as ::core::ffi::c_int != 0 || (*r#mod).patdly2 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *(*r#mod).positions.offset((*r#mod).sngpos as isize) as ::core::ffi::c_int
        != -1 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
        && dat as ::core::ffi::c_int
            > *(*r#mod)
                .pattrows
                .offset(*(*r#mod).positions.offset((*r#mod).sngpos as isize) as isize)
                as ::core::ffi::c_int
    {
        dat = *(*r#mod)
            .pattrows
            .offset(*(*r#mod).positions.offset((*r#mod).sngpos as isize) as isize)
            as UBYTE;
    }
    (*r#mod).patbrk = dat as UWORD;
    if (*r#mod).posjmp == 0 {
        if (*r#mod).sngpos as ::core::ffi::c_int
            == (*r#mod).numpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            && dat as ::core::ffi::c_int != 0
            && ((*r#mod).r#loop != 0
                || *(*r#mod).positions.offset((*r#mod).sngpos as isize)
                    as ::core::ffi::c_int
                    == (*r#mod).numpat as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    && flags as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0)
        {
            (*r#mod).sngpos = 0 as SWORD;
            (*r#mod).posjmp = 2 as SWORD;
        } else {
            (*r#mod).posjmp = 3 as SWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoEEffects(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
    mut dat: UBYTE,
) {
    let mut nib: UBYTE = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
        as UBYTE;
    match dat as ::core::ffi::c_int >> 4 as ::core::ffi::c_int {
        0x1 => {
            if (*a).main.period != 0 {
                if tick == 0 {
                    (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                        - ((nib as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as UWORD;
                }
            }
        }
        0x2 => {
            if (*a).main.period != 0 {
                if tick == 0 {
                    (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                        + ((nib as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                        as UWORD;
                }
            }
        }
        0x3 => {
            (*a).glissando = nib;
        }
        0x4 => {
            (*a).wavecontrol = ((*a).wavecontrol as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int) as UBYTE;
            (*a).wavecontrol = ((*a).wavecontrol as ::core::ffi::c_int
                | nib as ::core::ffi::c_int) as UBYTE;
        }
        0x5 => {
            if (*a).main.period != 0 {
                if flags as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                    (*a).speed = (nib as ::core::ffi::c_int + 128 as ::core::ffi::c_int)
                        as ULONG;
                } else {
                    (*a).speed = finetune[nib as usize] as ULONG;
                }
                (*a).tmpperiod = GetPeriod(
                    flags,
                    (((*a).main.note as UWORD as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as UWORD,
                    (*a).speed,
                );
            }
        }
        0x6 => {
            if tick == 0 {
                if nib != 0 {
                    if (*a).pat_repcnt != 0 {
                        (*a).pat_repcnt = (*a).pat_repcnt.wrapping_sub(1);
                    } else {
                        (*a).pat_repcnt = nib as UWORD;
                    }
                    if (*a).pat_repcnt != 0 {
                        if (*a).pat_reppos as ::core::ffi::c_int
                            == -2 as ::core::ffi::c_int
                        {
                            (*a).pat_reppos = ((*r#mod).patpos as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as SWORD;
                        }
                        if (*a).pat_reppos as ::core::ffi::c_int
                            == -1 as ::core::ffi::c_int
                        {
                            (*r#mod).pat_repcrazy = 1 as UBYTE;
                            (*r#mod).patpos = 0 as UWORD;
                        } else {
                            (*r#mod).patpos = (*a).pat_reppos as UWORD;
                        }
                    } else {
                        (*a).pat_reppos = -2 as SWORD;
                    }
                } else {
                    (*a).pat_reppos = ((*r#mod).patpos as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as SWORD;
                    if flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0 {
                        (*r#mod).patbrk = (*r#mod).patpos;
                    }
                }
            }
        }
        0x7 => {
            (*a).wavecontrol = ((*a).wavecontrol as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as UBYTE;
            (*a).wavecontrol = ((*a).wavecontrol as ::core::ffi::c_int
                | (nib as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UBYTE;
        }
        0x8 => {
            if (*r#mod).panflag != 0 {
                if nib as ::core::ffi::c_int <= 8 as ::core::ffi::c_int {
                    nib = ((nib as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as UBYTE;
                } else {
                    nib = (nib as ::core::ffi::c_int * 17 as ::core::ffi::c_int)
                        as UBYTE;
                }
                (*r#mod).panning[channel as usize] = nib as UWORD;
                (*a).main.panning = (*r#mod).panning[channel as usize] as SWORD;
            }
        }
        0x9 => {
            if !(tick == 0
                && !(flags as ::core::ffi::c_int & 0x200 as ::core::ffi::c_int != 0
                    && nib == 0))
            {
                if nib as ::core::ffi::c_int != 0 || tick == 0 {
                    if (*a).retrig == 0 {
                        if (*a).main.period != 0 {
                            (*a).main.kick = 1 as UBYTE;
                        }
                        (*a).retrig = nib as SBYTE;
                    }
                    (*a).retrig -= 1;
                }
            }
        }
        0xa => {
            if tick == 0 {
                (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                    + nib as ::core::ffi::c_int) as SWORD;
                if (*a).tmpvolume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                    (*a).tmpvolume = 64 as SWORD;
                }
            }
        }
        0xb => {
            if tick == 0 {
                (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                    - nib as ::core::ffi::c_int) as SWORD;
                if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    (*a).tmpvolume = 0 as SWORD;
                }
            }
        }
        0xc => {
            if tick as ::core::ffi::c_int >= nib as ::core::ffi::c_int {
                (*a).tmpvolume = 0 as SWORD;
            }
        }
        0xd => {
            if tick == 0 {
                (*a).main.notedelay = nib;
            } else if (*a).main.notedelay != 0 {
                (*a).main.notedelay = (*a).main.notedelay.wrapping_sub(1);
            }
        }
        0xe => {
            if tick == 0 {
                if (*r#mod).patdly2 == 0 {
                    (*r#mod).patdly = (nib as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as UBYTE;
                }
            }
        }
        0 | 0xf | _ => {}
    };
}
unsafe extern "C" fn DoPTEffectE(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoEEffects(tick, flags, a, r#mod, channel, UniGetByte());
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoPTEffectF(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick as ::core::ffi::c_int != 0 || (*r#mod).patdly2 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*r#mod).extspd != 0
        && dat as ::core::ffi::c_int >= (*r#mod).bpmlimit as ::core::ffi::c_int
    {
        (*r#mod).bpm = dat as UWORD;
    } else if dat != 0 {
        (*r#mod).sngspd = (if dat as ::core::ffi::c_int
            >= (*r#mod).bpmlimit as ::core::ffi::c_int
        {
            (*r#mod).bpmlimit as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        } else {
            dat as ::core::ffi::c_int
        }) as UWORD;
        (*r#mod).vbtick = 0 as UWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectA(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut speed: UBYTE = 0;
    speed = UniGetByte();
    if tick as ::core::ffi::c_int != 0 || (*r#mod).patdly2 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if speed as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        speed = (speed as ::core::ffi::c_int - 128 as ::core::ffi::c_int) as UBYTE;
    }
    if speed != 0 {
        (*r#mod).sngspd = speed as UWORD;
        (*r#mod).vbtick = 0 as UWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MVolSlide(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut inf: UBYTE,
) {
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    if inf != 0 {
        (*a).s3mvolslide = inf;
    } else {
        inf = (*a).s3mvolslide;
    }
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    if lo == 0 {
        if tick as ::core::ffi::c_int != 0
            || flags as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0
        {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                + hi as ::core::ffi::c_int) as SWORD;
        }
    } else if hi == 0 {
        if tick as ::core::ffi::c_int != 0
            || flags as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0
        {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as SWORD;
        }
    } else if lo as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                + if hi as ::core::ffi::c_int != 0 {
                    hi as ::core::ffi::c_int
                } else {
                    0xf as ::core::ffi::c_int
                }) as SWORD;
        }
    } else if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                - if lo as ::core::ffi::c_int != 0 {
                    lo as ::core::ffi::c_int
                } else {
                    0xf as ::core::ffi::c_int
                }) as SWORD;
        }
    } else {
        return
    }
    if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*a).tmpvolume = 0 as SWORD;
    } else if (*a).tmpvolume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        (*a).tmpvolume = 64 as SWORD;
    }
}
unsafe extern "C" fn DoS3MEffectD(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoS3MVolSlide(tick, flags, a, UniGetByte());
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MSlideDn(
    mut tick: UWORD,
    mut a: *mut MP_CONTROL,
    mut inf: UBYTE,
) {
    let mut hi: UBYTE = 0;
    let mut lo: UBYTE = 0;
    if inf != 0 {
        (*a).slidespeed = inf as UWORD;
    } else {
        inf = (*a).slidespeed as UBYTE;
    }
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + ((lo as UWORD as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        }
    } else if hi as ::core::ffi::c_int == 0xe as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + lo as ::core::ffi::c_int) as UWORD;
        }
    } else if tick != 0 {
        (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
            + ((inf as UWORD as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
            as UWORD;
    }
}
unsafe extern "C" fn DoS3MEffectE(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if (*a).main.period != 0 {
        DoS3MSlideDn(tick, a, dat);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MSlideUp(
    mut tick: UWORD,
    mut a: *mut MP_CONTROL,
    mut inf: UBYTE,
) {
    let mut hi: UBYTE = 0;
    let mut lo: UBYTE = 0;
    if inf != 0 {
        (*a).slidespeed = inf as UWORD;
    } else {
        inf = (*a).slidespeed as UBYTE;
    }
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - ((lo as UWORD as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        }
    } else if hi as ::core::ffi::c_int == 0xe as ::core::ffi::c_int {
        if tick == 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as UWORD;
        }
    } else if tick != 0 {
        (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
            - ((inf as UWORD as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
            as UWORD;
    }
}
unsafe extern "C" fn DoS3MEffectF(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if (*a).main.period != 0 {
        DoS3MSlideUp(tick, a, dat);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectI(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut on: UBYTE = 0;
    let mut off: UBYTE = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*a).s3mtronof = inf;
    } else {
        inf = (*a).s3mtronof;
        if inf == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if tick == 0 {
        return 0 as ::core::ffi::c_int;
    }
    on = ((inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as UBYTE;
    off = ((inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as UBYTE;
    (*a).s3mtremor = ((*a).s3mtremor as ::core::ffi::c_int
        % (on as ::core::ffi::c_int + off as ::core::ffi::c_int)) as UBYTE;
    (*a).volume = (if ((*a).s3mtremor as ::core::ffi::c_int) < on as ::core::ffi::c_int {
        (*a).tmpvolume as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as SWORD;
    (*a).ownvol = 1 as SWORD;
    (*a).s3mtremor = (*a).s3mtremor.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectQ(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    inf = UniGetByte();
    if (*a).main.period != 0 {
        if inf != 0 {
            (*a).s3mrtgslide = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as UBYTE;
            (*a).s3mrtgspeed = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if (*a).s3mrtgspeed as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (*a).retrig == 0 {
                if (*a).main.kick as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
                    (*a).main.kick = 2 as UBYTE;
                }
                (*a).retrig = (*a).s3mrtgspeed as SBYTE;
                if tick as ::core::ffi::c_int != 0
                    || flags as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0
                {
                    match (*a).s3mrtgslide as ::core::ffi::c_int {
                        1 | 2 | 3 | 4 | 5 => {
                            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                                - ((1 as ::core::ffi::c_int)
                                    << (*a).s3mrtgslide as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)) as SWORD;
                        }
                        6 => {
                            (*a).tmpvolume = (2 as ::core::ffi::c_int
                                * (*a).tmpvolume as ::core::ffi::c_int
                                / 3 as ::core::ffi::c_int) as SWORD;
                        }
                        7 => {
                            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as SWORD;
                        }
                        9 | 0xa | 0xb | 0xc | 0xd => {
                            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int)
                                    << (*a).s3mrtgslide as ::core::ffi::c_int
                                        - 9 as ::core::ffi::c_int)) as SWORD;
                        }
                        0xe => {
                            (*a).tmpvolume = (3 as ::core::ffi::c_int
                                * (*a).tmpvolume as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as SWORD;
                        }
                        0xf => {
                            (*a).tmpvolume = (((*a).tmpvolume as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as SWORD;
                        }
                        _ => {}
                    }
                    if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        (*a).tmpvolume = 0 as SWORD;
                    } else if (*a).tmpvolume as ::core::ffi::c_int
                        > 64 as ::core::ffi::c_int
                    {
                        (*a).tmpvolume = 64 as SWORD;
                    }
                }
            }
            (*a).retrig -= 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectR(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).trmdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).trmspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    }
    q = ((*a).trmpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int) as UBYTE;
    match (*a).wavecontrol as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
        & 3 as ::core::ffi::c_int
    {
        0 => {
            temp = VibratoTable[q as usize] as UWORD;
        }
        1 => {
            q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
            if ((*a).trmpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
            }
            temp = q as UWORD;
        }
        2 => {
            temp = 255 as UWORD;
        }
        3 => {
            temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
        }
        _ => {}
    }
    temp = (temp as ::core::ffi::c_int * (*a).trmdepth as ::core::ffi::c_int) as UWORD;
    temp = (temp as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as UWORD;
    if (*a).trmpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        (*a).volume = ((*a).tmpvolume as ::core::ffi::c_int + temp as ::core::ffi::c_int)
            as SWORD;
        if (*a).volume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
            (*a).volume = 64 as SWORD;
        }
    } else {
        (*a).volume = ((*a).tmpvolume as ::core::ffi::c_int - temp as ::core::ffi::c_int)
            as SWORD;
        if ((*a).volume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*a).volume = 0 as SWORD;
        }
    }
    (*a).ownvol = 1 as SWORD;
    if tick != 0 {
        (*a).trmpos = ((*a).trmpos as ::core::ffi::c_int
            + (*a).trmspd as ::core::ffi::c_int) as SBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectT(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut tempo: UBYTE = 0;
    tempo = UniGetByte();
    if tick as ::core::ffi::c_int != 0 || (*r#mod).patdly2 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*r#mod).bpm = (if (tempo as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        32 as ::core::ffi::c_int
    } else {
        tempo as ::core::ffi::c_int
    }) as UWORD;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoS3MEffectU(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).vibdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).vibspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    } else if (*a).main.period != 0 {
        q = ((*a).vibpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) as UBYTE;
        match (*a).wavecontrol as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
            0 => {
                temp = VibratoTable[q as usize] as UWORD;
            }
            1 => {
                q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
                if ((*a).vibpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
                }
                temp = q as UWORD;
            }
            2 => {
                temp = 255 as UWORD;
            }
            3 => {
                temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
            }
            _ => {}
        }
        temp = (temp as ::core::ffi::c_int * (*a).vibdepth as ::core::ffi::c_int)
            as UWORD;
        temp = (temp as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as UWORD;
        if (*a).vibpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
                + temp as ::core::ffi::c_int) as UWORD;
        } else {
            (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
                - temp as ::core::ffi::c_int) as UWORD;
        }
        (*a).ownper = 1 as SWORD;
        (*a).vibpos = ((*a).vibpos as ::core::ffi::c_int
            + (*a).vibspd as ::core::ffi::c_int) as SBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoKeyOff(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    (*a).main.keyoff = ((*a).main.keyoff as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
        as UBYTE;
    if (*a).main.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
        || (*a).main.volflg as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0
    {
        (*a).main.keyoff = (1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as UBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoKeyFade(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick as ::core::ffi::c_int >= dat as ::core::ffi::c_int
        || tick as ::core::ffi::c_int
            == (*r#mod).sngspd as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        (*a).main.keyoff = (1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as UBYTE;
        if (*a).main.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
            (*a).main.fadevol = 0 as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectA(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*a).s3mvolslide = inf;
    } else {
        inf = (*a).s3mvolslide;
    }
    if tick != 0 {
        lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
        hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
        if hi == 0 {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as SWORD;
            if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                (*a).tmpvolume = 0 as SWORD;
            }
        } else {
            (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
                + hi as ::core::ffi::c_int) as SWORD;
            if (*a).tmpvolume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                (*a).tmpvolume = 64 as SWORD;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffect6(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    if (*a).main.period != 0 {
        DoVibrato(tick, a);
    }
    return DoXMEffectA(tick, flags, a, r#mod, channel);
}
unsafe extern "C" fn DoXMEffectE1(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat != 0 {
            (*a).fportupspd = dat;
        }
        if (*a).main.period != 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - (((*a).fportupspd as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectE2(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat != 0 {
            (*a).fportdnspd = dat;
        }
        if (*a).main.period != 0 {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + (((*a).fportdnspd as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectEA(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat != 0 {
            (*a).fslideupspd = dat;
        }
    }
    (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
        + (*a).fslideupspd as ::core::ffi::c_int) as SWORD;
    if (*a).tmpvolume as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        (*a).tmpvolume = 64 as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectEB(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 {
        if dat != 0 {
            (*a).fslidednspd = dat;
        }
    }
    (*a).tmpvolume = ((*a).tmpvolume as ::core::ffi::c_int
        - (*a).fslidednspd as ::core::ffi::c_int) as SWORD;
    if ((*a).tmpvolume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*a).tmpvolume = 0 as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectG(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    (*r#mod).volume = ((UniGetByte() as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        as SWORD;
    if (*r#mod).volume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        (*r#mod).volume = 128 as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectH(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    inf = UniGetByte();
    if tick != 0 {
        if inf != 0 {
            (*r#mod).globalslide = inf;
        } else {
            inf = (*r#mod).globalslide;
        }
        if inf as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            inf = (inf as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int) as UBYTE;
        }
        (*r#mod).volume = ((*r#mod).volume as ::core::ffi::c_int
            + ((inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                - (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                * 2 as ::core::ffi::c_int) as SWORD;
        if ((*r#mod).volume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*r#mod).volume = 0 as SWORD;
        } else if (*r#mod).volume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
            (*r#mod).volume = 128 as SWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectL(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if tick == 0 && !(*a).main.i.is_null() {
        let mut points: UWORD = 0;
        let mut i: *mut INSTRUMENT = (*a).main.i;
        let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
        aout = (*a).slave as *mut MP_VOICE;
        if !aout.is_null() {
            if !(*aout).venv.env.is_null() {
                points = (*i)
                    .volenv[((*i).volpts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as usize]
                    .pos as UWORD;
                (*aout).venv.p = (*(*aout)
                    .venv
                    .env
                    .offset(
                        (if dat as ::core::ffi::c_int > points as ::core::ffi::c_int {
                            points as ::core::ffi::c_int
                        } else {
                            dat as ::core::ffi::c_int
                        }) as isize,
                    ))
                    .pos;
            }
            if !(*aout).penv.env.is_null() {
                points = (*i)
                    .panenv[((*i).panpts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as usize]
                    .pos as UWORD;
                (*aout).penv.p = (*(*aout)
                    .penv
                    .env
                    .offset(
                        (if dat as ::core::ffi::c_int > points as ::core::ffi::c_int {
                            points as ::core::ffi::c_int
                        } else {
                            dat as ::core::ffi::c_int
                        }) as isize,
                    ))
                    .pos;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectP(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    let mut pan: SWORD = 0;
    inf = UniGetByte();
    if (*r#mod).panflag == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if inf != 0 {
        (*a).pansspd = inf;
    } else {
        inf = (*a).pansspd;
    }
    if tick != 0 {
        lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
        hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
        if hi != 0 {
            lo = 0 as UBYTE;
        }
        pan = (if (*a).main.panning as ::core::ffi::c_int == 512 as ::core::ffi::c_int {
            128 as ::core::ffi::c_int
        } else {
            (*a).main.panning as ::core::ffi::c_int
        } + hi as ::core::ffi::c_int - lo as ::core::ffi::c_int) as SWORD;
        (*a).main.panning = (if (pan as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else if pan as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
            255 as ::core::ffi::c_int
        } else {
            pan as ::core::ffi::c_int
        }) as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectX1(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if dat != 0 {
        (*a).ffportupspd = dat;
    } else {
        dat = (*a).ffportupspd;
    }
    if (*a).main.period != 0 {
        if tick == 0 {
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                - dat as ::core::ffi::c_int) as UWORD;
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - dat as ::core::ffi::c_int) as UWORD;
            (*a).ownper = 1 as SWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoXMEffectX2(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    dat = UniGetByte();
    if dat != 0 {
        (*a).ffportdnspd = dat;
    } else {
        dat = (*a).ffportdnspd;
    }
    if (*a).main.period != 0 {
        if tick == 0 {
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                + dat as ::core::ffi::c_int) as UWORD;
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + dat as ::core::ffi::c_int) as UWORD;
            (*a).ownper = 1 as SWORD;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITToneSlide(
    mut tick: UWORD,
    mut a: *mut MP_CONTROL,
    mut dat: UBYTE,
) {
    if dat != 0 {
        (*a).portspeed = dat as UWORD;
    }
    if (*a).oldnote == 0 || (*a).main.period == 0 {
        return;
    }
    if tick == 0 && (*a).newsamp as ::core::ffi::c_int != 0 {
        (*a).main.kick = 1 as UBYTE;
        (*a).main.start = -1 as SLONG;
    } else {
        (*a).main.kick = (if (*a).main.kick as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as UBYTE;
    }
    if tick != 0 {
        let mut dist: ::core::ffi::c_int = 0;
        dist = (*a).main.period as ::core::ffi::c_int
            - (*a).wantedperiod as ::core::ffi::c_int;
        if dist == 0
            || ((*a).portspeed as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                > abs(dist)
        {
            (*a).main.period = (*a).wantedperiod;
            (*a).tmpperiod = (*a).main.period;
        } else if dist > 0 as ::core::ffi::c_int {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                - (((*a).portspeed as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                - (((*a).portspeed as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        } else {
            (*a).tmpperiod = ((*a).tmpperiod as ::core::ffi::c_int
                + (((*a).portspeed as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
            (*a).main.period = ((*a).main.period as ::core::ffi::c_int
                + (((*a).portspeed as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as UWORD;
        }
    } else {
        (*a).tmpperiod = (*a).main.period;
    }
    (*a).ownper = 1 as SWORD;
}
unsafe extern "C" fn DoITEffectG(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoITToneSlide(tick, a, UniGetByte());
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITVibrato(
    mut tick: UWORD,
    mut a: *mut MP_CONTROL,
    mut dat: UBYTE,
) {
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).vibdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).vibspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    }
    if (*a).main.period == 0 {
        return;
    }
    q = ((*a).vibpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int) as UBYTE;
    match (*a).wavecontrol as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
        0 => {
            temp = VibratoTable[q as usize] as UWORD;
        }
        1 => {
            temp = 255 as UWORD;
        }
        2 => {
            q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
            if ((*a).vibpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
            }
            temp = q as UWORD;
        }
        3 => {
            temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
        }
        _ => {}
    }
    temp = (temp as ::core::ffi::c_int * (*a).vibdepth as ::core::ffi::c_int) as UWORD;
    temp = (temp as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as UWORD;
    temp = ((temp as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD;
    if (*a).vibpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
            + temp as ::core::ffi::c_int) as UWORD;
    } else {
        (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
            - temp as ::core::ffi::c_int) as UWORD;
    }
    (*a).ownper = 1 as SWORD;
    (*a).vibpos = ((*a).vibpos as ::core::ffi::c_int + (*a).vibspd as ::core::ffi::c_int)
        as SBYTE;
}
unsafe extern "C" fn DoITEffectH(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoITVibrato(tick, a, UniGetByte());
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectI(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut on: UBYTE = 0;
    let mut off: UBYTE = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*a).s3mtronof = inf;
    } else {
        inf = (*a).s3mtronof;
        if inf == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    on = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    off = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    (*a).s3mtremor = ((*a).s3mtremor as ::core::ffi::c_int
        % (on as ::core::ffi::c_int + off as ::core::ffi::c_int)) as UBYTE;
    (*a).volume = (if ((*a).s3mtremor as ::core::ffi::c_int) < on as ::core::ffi::c_int {
        (*a).tmpvolume as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as SWORD;
    (*a).ownvol = 1 as SWORD;
    (*a).s3mtremor = (*a).s3mtremor.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectM(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    (*a).main.chanvol = UniGetByte() as SBYTE;
    if (*a).main.chanvol as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        (*a).main.chanvol = 64 as SBYTE;
    } else if ((*a).main.chanvol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*a).main.chanvol = 0 as SBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectN(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*a).chanvolslide = inf;
    } else {
        inf = (*a).chanvolslide;
    }
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    if hi == 0 {
        (*a).main.chanvol = ((*a).main.chanvol as ::core::ffi::c_int
            - lo as ::core::ffi::c_int) as SBYTE;
    } else if lo == 0 {
        (*a).main.chanvol = ((*a).main.chanvol as ::core::ffi::c_int
            + hi as ::core::ffi::c_int) as SBYTE;
    } else if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).main.chanvol = ((*a).main.chanvol as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as SBYTE;
        }
    } else if lo as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*a).main.chanvol = ((*a).main.chanvol as ::core::ffi::c_int
                + hi as ::core::ffi::c_int) as SBYTE;
        }
    }
    if ((*a).main.chanvol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*a).main.chanvol = 0 as SBYTE;
    } else if (*a).main.chanvol as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
        (*a).main.chanvol = 64 as SBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectP(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    let mut pan: SWORD = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*a).pansspd = inf;
    } else {
        inf = (*a).pansspd;
    }
    if (*r#mod).panflag == 0 {
        return 0 as ::core::ffi::c_int;
    }
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    pan = (if (*a).main.panning as ::core::ffi::c_int == 512 as ::core::ffi::c_int {
        128 as ::core::ffi::c_int
    } else {
        (*a).main.panning as ::core::ffi::c_int
    }) as SWORD;
    if hi == 0 {
        pan = (pan as ::core::ffi::c_int
            + ((lo as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SWORD;
    } else if lo == 0 {
        pan = (pan as ::core::ffi::c_int
            - ((hi as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SWORD;
    } else if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            pan = (pan as ::core::ffi::c_int
                + ((lo as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SWORD;
        }
    } else if lo as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            pan = (pan as ::core::ffi::c_int
                - ((hi as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SWORD;
        }
    }
    (*a).main.panning = (if (pan as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else if pan as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else {
        pan as ::core::ffi::c_int
    }) as SWORD;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectT(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut tempo: UBYTE = 0;
    let mut temp: SWORD = 0;
    tempo = UniGetByte();
    if (*r#mod).patdly2 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    temp = (*r#mod).bpm as SWORD;
    if tempo as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
        temp = (temp as ::core::ffi::c_int
            + (tempo as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)) as SWORD;
    } else {
        temp = (temp as ::core::ffi::c_int - tempo as ::core::ffi::c_int) as SWORD;
    }
    (*r#mod).bpm = (if temp as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else if (temp as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        temp as ::core::ffi::c_int
    }) as UWORD;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectU(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut q: UBYTE = 0;
    let mut temp: UWORD = 0 as UWORD;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).vibdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).vibspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as UBYTE;
        }
    }
    if (*a).main.period != 0 {
        q = ((*a).vibpos as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) as UBYTE;
        match (*a).wavecontrol as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
            0 => {
                temp = VibratoTable[q as usize] as UWORD;
            }
            1 => {
                temp = 255 as UWORD;
            }
            2 => {
                q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
                if ((*a).vibpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    q = (255 as ::core::ffi::c_int - q as ::core::ffi::c_int) as UBYTE;
                }
                temp = q as UWORD;
            }
            3 => {
                temp = getrandom(256 as ::core::ffi::c_int) as UWORD;
            }
            _ => {}
        }
        temp = (temp as ::core::ffi::c_int * (*a).vibdepth as ::core::ffi::c_int)
            as UWORD;
        temp = (temp as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as UWORD;
        if (*a).vibpos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
                + temp as ::core::ffi::c_int) as UWORD;
        } else {
            (*a).main.period = ((*a).tmpperiod as ::core::ffi::c_int
                - temp as ::core::ffi::c_int) as UWORD;
        }
        (*a).ownper = 1 as SWORD;
        (*a).vibpos = ((*a).vibpos as ::core::ffi::c_int
            + (*a).vibspd as ::core::ffi::c_int) as SBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectW(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut inf: UBYTE = 0;
    let mut lo: UBYTE = 0;
    let mut hi: UBYTE = 0;
    inf = UniGetByte();
    if inf != 0 {
        (*r#mod).globalslide = inf;
    } else {
        inf = (*r#mod).globalslide;
    }
    lo = (inf as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    hi = (inf as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    if lo == 0 {
        if tick != 0 {
            (*r#mod).volume = ((*r#mod).volume as ::core::ffi::c_int
                + hi as ::core::ffi::c_int) as SWORD;
        }
    } else if hi == 0 {
        if tick != 0 {
            (*r#mod).volume = ((*r#mod).volume as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as SWORD;
        }
    } else if lo as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*r#mod).volume = ((*r#mod).volume as ::core::ffi::c_int
                + hi as ::core::ffi::c_int) as SWORD;
        }
    } else if hi as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        if tick == 0 {
            (*r#mod).volume = ((*r#mod).volume as ::core::ffi::c_int
                - lo as ::core::ffi::c_int) as SWORD;
        }
    }
    if ((*r#mod).volume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*r#mod).volume = 0 as SWORD;
    } else if (*r#mod).volume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
        (*r#mod).volume = 128 as SWORD;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectY(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut q: UBYTE = 0;
    let mut temp: SLONG = 0 as SLONG;
    dat = UniGetByte();
    if tick == 0 {
        if dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != 0 {
            (*a).panbdepth = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as UBYTE;
        }
        if dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int != 0 {
            (*a).panbspd = ((dat as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                >> 4 as ::core::ffi::c_int) as SBYTE;
        }
    }
    if (*r#mod).panflag != 0 {
        q = (*a).panbpos;
        match (*a).panbwave as ::core::ffi::c_int {
            0 => {
                temp = PanbrelloTable[q as usize] as SLONG;
            }
            1 => {
                temp = (if (q as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                    64 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as SLONG;
            }
            2 => {
                q = ((q as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UBYTE;
                temp = q as SLONG;
            }
            3 => {
                temp = getrandom(256 as ::core::ffi::c_int) as SLONG;
            }
            _ => {}
        }
        temp *= (*a).panbdepth as SLONG;
        temp = temp / 8 as SLONG + (*r#mod).panning[channel as usize] as SLONG;
        (*a).main.panning = (if temp < 0 as SLONG {
            0 as SLONG
        } else if temp > 255 as SLONG {
            255 as SLONG
        } else {
            temp
        }) as SWORD;
        (*a).panbpos = ((*a).panbpos as ::core::ffi::c_int
            + (*a).panbspd as ::core::ffi::c_int) as UBYTE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoITEffectS0(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut inf: UBYTE = 0;
    let mut c: UBYTE = 0;
    dat = UniGetByte();
    inf = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    c = (dat as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as UBYTE;
    if dat == 0 {
        c = (*a).sseffect;
        inf = (*a).ssdata;
    } else {
        (*a).sseffect = c;
        (*a).ssdata = inf;
    }
    match c as ::core::ffi::c_int {
        1 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x30 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        2 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x50 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        3 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x40 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        4 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x70 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        5 => {
            (*a).panbwave = inf;
        }
        6 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0xe0 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        7 => {
            DoNNAEffects(r#mod, a, inf);
        }
        8 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x80 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        9 => {
            if (*r#mod).panflag != 0 {
                (*r#mod).panning[channel as usize] = 512 as UWORD;
                (*a).main.panning = (*r#mod).panning[channel as usize] as SWORD;
            }
        }
        10 => {
            if tick == 0 {
                (*a).hioffset = ((inf as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                    as ULONG;
                (*a).main.start = ((*a).hioffset | (*a).soffset as ULONG) as SLONG;
                if !(*a).main.s.is_null()
                    && (*a).main.start as ULONG > (*(*a).main.s).length
                {
                    (*a).main.start = (if (*(*a).main.s).flags as ::core::ffi::c_int
                        & (0x100 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int)
                        != 0
                    {
                        (*(*a).main.s).loopstart
                    } else {
                        (*(*a).main.s).length
                    }) as SLONG;
                }
            }
        }
        11 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0x60 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        12 => {
            if inf == 0 {
                inf = 1 as UBYTE;
            }
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0xc0 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        13 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0xd0 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        14 => {
            DoEEffects(
                tick,
                flags,
                a,
                r#mod,
                channel,
                (0xe0 as ::core::ffi::c_int | inf as ::core::ffi::c_int) as UBYTE,
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoVolEffects(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut c: UBYTE = 0;
    let mut inf: UBYTE = 0;
    c = UniGetByte();
    inf = UniGetByte();
    if c == 0 && inf == 0 {
        c = (*a).voleffect;
        inf = (*a).voldata;
    } else {
        (*a).voleffect = c;
        (*a).voldata = inf;
    }
    if c != 0 {
        match c as ::core::ffi::c_int {
            1 => {
                if tick == 0 {
                    if inf as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                        inf = 64 as UBYTE;
                    }
                    (*a).tmpvolume = inf as SWORD;
                }
            }
            2 => {
                if (*r#mod).panflag != 0 {
                    (*a).main.panning = inf as SWORD;
                }
            }
            3 => {
                DoS3MVolSlide(tick, flags, a, inf);
                return 1 as ::core::ffi::c_int;
            }
            4 => {
                if (*a).main.period != 0 {
                    DoS3MSlideDn(tick, a, inf);
                }
            }
            5 => {
                if (*a).main.period != 0 {
                    DoS3MSlideUp(tick, a, inf);
                }
            }
            6 => {
                DoITToneSlide(tick, a, inf);
            }
            7 => {
                DoITVibrato(tick, a, inf);
            }
            _ => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoULTEffect9(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut offset: UWORD = UniGetWord();
    if offset != 0 {
        (*a).ultoffset = offset;
    }
    (*a).main.start = (((*a).ultoffset as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        as SLONG;
    if !(*a).main.s.is_null() && (*a).main.start as ULONG > (*(*a).main.s).length {
        (*a).main.start = (if (*(*a).main.s).flags as ::core::ffi::c_int
            & (0x100 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int) != 0
        {
            (*(*a).main.s).loopstart
        } else {
            (*(*a).main.s).length
        }) as SLONG;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoMEDSpeed(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut speed: UWORD = UniGetWord();
    (*r#mod).bpm = speed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoMEDEffectF1(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoEEffects(
        tick,
        flags,
        a,
        r#mod,
        channel,
        (0x90 as ::core::ffi::c_int
            | (*r#mod).sngspd as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as UBYTE,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoMEDEffectF2(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoEEffects(
        tick,
        flags,
        a,
        r#mod,
        channel,
        (0xd0 as ::core::ffi::c_int
            | (*r#mod).sngspd as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as UBYTE,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoMEDEffectF3(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    DoEEffects(
        tick,
        flags,
        a,
        r#mod,
        channel,
        (0x90 as ::core::ffi::c_int
            | (*r#mod).sngspd as ::core::ffi::c_int / 3 as ::core::ffi::c_int) as UBYTE,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoOktArp(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    let mut dat: UBYTE = 0;
    let mut dat2: UBYTE = 0;
    dat2 = UniGetByte();
    dat = UniGetByte();
    if tick == 0 {
        if dat == 0 && flags as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int != 0 {
            dat = (*a).arpmem;
        } else {
            (*a).arpmem = dat;
        }
    }
    if (*a).main.period != 0 {
        DoArpeggio(tick, flags, a, dat2);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn DoNothing(
    mut tick: UWORD,
    mut flags: UWORD,
    mut a: *mut MP_CONTROL,
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
) -> ::core::ffi::c_int {
    UniSkipOpcode();
    return 0 as ::core::ffi::c_int;
}
static mut effects: [effect_func; 62] = [
    Some(
        DoNothing
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoNothing
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoNothing
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect0
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect1
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect2
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect3
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect4
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect5
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect6
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect7
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect8
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect9
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectA
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectB
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectC
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectD
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectE
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffectF
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectA
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectD
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectE
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectF
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectI
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectQ
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectR
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectT
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoS3MEffectU
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoKeyOff
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoKeyFade
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoVolEffects
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoPTEffect4
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffect6
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectA
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectE1
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectE2
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectEA
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectEB
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectG
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectH
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectL
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectP
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectX1
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoXMEffectX2
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectG
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectH
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectI
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectM
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectN
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectP
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectT
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectU
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectW
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectY
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoNothing
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoITEffectS0
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoULTEffect9
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoMEDSpeed
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoMEDEffectF1
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoMEDEffectF2
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoMEDEffectF3
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        DoOktArp
            as unsafe extern "C" fn(
                UWORD,
                UWORD,
                *mut MP_CONTROL,
                *mut MODULE,
                SWORD,
            ) -> ::core::ffi::c_int,
    ),
];
unsafe extern "C" fn pt_playeffects(
    mut r#mod: *mut MODULE,
    mut channel: SWORD,
    mut a: *mut MP_CONTROL,
) -> ::core::ffi::c_int {
    let mut tick: UWORD = (*r#mod).vbtick;
    let mut flags: UWORD = (*r#mod).flags;
    let mut c: UBYTE = 0;
    let mut explicitslides: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: effect_func = None;
    loop {
        c = UniGetByte();
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        f = effects[c as usize];
        if f
            != Some(
                DoNothing
                    as unsafe extern "C" fn(
                        UWORD,
                        UWORD,
                        *mut MP_CONTROL,
                        *mut MODULE,
                        SWORD,
                    ) -> ::core::ffi::c_int,
            )
        {
            (*a).sliding = 0 as SBYTE;
        }
        explicitslides
            |= f.expect("non-null function pointer")(tick, flags, a, r#mod, channel);
    }
    return explicitslides;
}
unsafe extern "C" fn DoNNAEffects(
    mut r#mod: *mut MODULE,
    mut a: *mut MP_CONTROL,
    mut dat: UBYTE,
) {
    let mut t: ::core::ffi::c_int = 0;
    let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
    dat = (dat as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as UBYTE;
    aout = (if !(*a).slave.is_null() {
        (*a).slave
    } else {
        ::core::ptr::null_mut::<MP_VOICE>()
    }) as *mut MP_VOICE;
    match dat as ::core::ffi::c_int {
        0 => {
            t = 0 as ::core::ffi::c_int;
            while t
                < if (md_sngchn as ::core::ffi::c_int)
                    < (*r#mod).numvoices as ::core::ffi::c_int
                {
                    md_sngchn as ::core::ffi::c_int
                } else {
                    (*r#mod).numvoices as ::core::ffi::c_int
                }
            {
                if (*(*r#mod).voice.offset(t as isize)).master == a {
                    (*(*r#mod).voice.offset(t as isize)).main.fadevol = 0 as UWORD;
                }
                t += 1;
            }
        }
        0x1 => {
            t = 0 as ::core::ffi::c_int;
            while t
                < if (md_sngchn as ::core::ffi::c_int)
                    < (*r#mod).numvoices as ::core::ffi::c_int
                {
                    md_sngchn as ::core::ffi::c_int
                } else {
                    (*r#mod).numvoices as ::core::ffi::c_int
                }
            {
                if (*(*r#mod).voice.offset(t as isize)).master == a {
                    (*(*r#mod).voice.offset(t as isize)).main.keyoff = ((*(*r#mod)
                        .voice
                        .offset(t as isize))
                        .main
                        .keyoff as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                        as UBYTE;
                    if (*(*r#mod).voice.offset(t as isize)).venv.flg
                        as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                        || (*(*r#mod).voice.offset(t as isize)).venv.flg
                            as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0
                    {
                        (*(*r#mod).voice.offset(t as isize)).main.keyoff = (1
                            as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as UBYTE;
                    }
                }
                t += 1;
            }
        }
        0x2 => {
            t = 0 as ::core::ffi::c_int;
            while t
                < if (md_sngchn as ::core::ffi::c_int)
                    < (*r#mod).numvoices as ::core::ffi::c_int
                {
                    md_sngchn as ::core::ffi::c_int
                } else {
                    (*r#mod).numvoices as ::core::ffi::c_int
                }
            {
                if (*(*r#mod).voice.offset(t as isize)).master == a {
                    (*(*r#mod).voice.offset(t as isize)).main.keyoff = ((*(*r#mod)
                        .voice
                        .offset(t as isize))
                        .main
                        .keyoff as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
                        as UBYTE;
                }
                t += 1;
            }
        }
        0x3 => {
            (*a).main.nna = ((*a).main.nna as ::core::ffi::c_int
                & !(3 as ::core::ffi::c_int) | 0 as ::core::ffi::c_int) as UBYTE;
        }
        0x4 => {
            (*a).main.nna = ((*a).main.nna as ::core::ffi::c_int
                & !(3 as ::core::ffi::c_int) | 1 as ::core::ffi::c_int) as UBYTE;
        }
        0x5 => {
            (*a).main.nna = ((*a).main.nna as ::core::ffi::c_int
                & !(3 as ::core::ffi::c_int) | 2 as ::core::ffi::c_int) as UBYTE;
        }
        0x6 => {
            (*a).main.nna = ((*a).main.nna as ::core::ffi::c_int
                & !(3 as ::core::ffi::c_int) | 3 as ::core::ffi::c_int) as UBYTE;
        }
        0x7 => {
            if !aout.is_null() {
                (*aout).main.volflg = ((*aout).main.volflg as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int)) as UBYTE;
            }
        }
        0x8 => {
            if !aout.is_null() {
                (*aout).main.volflg = ((*aout).main.volflg as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_int) as UBYTE;
            }
        }
        0x9 => {
            if !aout.is_null() {
                (*aout).main.panflg = ((*aout).main.panflg as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int)) as UBYTE;
            }
        }
        0xa => {
            if !aout.is_null() {
                (*aout).main.panflg = ((*aout).main.panflg as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_int) as UBYTE;
            }
        }
        0xb => {
            if !aout.is_null() {
                (*aout).main.pitflg = ((*aout).main.pitflg as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int)) as UBYTE;
            }
        }
        0xc => {
            if !aout.is_null() {
                (*aout).main.pitflg = ((*aout).main.pitflg as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_int) as UBYTE;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn pt_UpdateVoices(
    mut r#mod: *mut MODULE,
    mut max_volume: ::core::ffi::c_int,
) {
    let mut envpan: SWORD = 0;
    let mut envvol: SWORD = 0;
    let mut envpit: SWORD = 0;
    let mut channel: SWORD = 0;
    let mut playperiod: UWORD = 0;
    let mut vibval: SLONG = 0;
    let mut vibdpt: SLONG = 0;
    let mut tmpvol: ULONG = 0;
    let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
    let mut i: *mut INSTRUMENT = ::core::ptr::null_mut::<INSTRUMENT>();
    let mut s: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
    (*r#mod).realchn = 0 as UBYTE;
    (*r#mod).totalchn = (*r#mod).realchn;
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int)
        < if (md_sngchn as ::core::ffi::c_int) < (*r#mod).numvoices as ::core::ffi::c_int
        {
            md_sngchn as ::core::ffi::c_int
        } else {
            (*r#mod).numvoices as ::core::ffi::c_int
        }
    {
        aout = (*r#mod).voice.offset(channel as isize) as *mut MP_VOICE;
        i = (*aout).main.i;
        s = (*aout).main.s;
        if !(s.is_null() || (*s).length == 0) {
            if ((*aout).main.period as ::core::ffi::c_int) < 40 as ::core::ffi::c_int {
                (*aout).main.period = 40 as UWORD;
            } else if (*aout).main.period as ::core::ffi::c_int
                > 50000 as ::core::ffi::c_int
            {
                (*aout).main.period = 50000 as UWORD;
            }
            if (*aout).main.kick as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*aout).main.kick as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            {
                Voice_Play_internal(
                    channel as SBYTE,
                    s,
                    if (*aout).main.start == -1 as SLONG {
                        if (*s).flags as ::core::ffi::c_int
                            & 0x2000 as ::core::ffi::c_int != 0
                        {
                            (*s).loopstart
                        } else {
                            0 as ULONG
                        }
                    } else {
                        (*aout).main.start as ULONG
                    },
                );
                (*aout).main.fadevol = 32768 as UWORD;
                (*aout).aswppos = 0 as UWORD;
            }
            envvol = 256 as SWORD;
            envpan = 128 as SWORD;
            envpit = 32 as SWORD;
            if !i.is_null()
                && ((*aout).main.kick as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    || (*aout).main.kick as ::core::ffi::c_int
                        == 4 as ::core::ffi::c_int)
            {
                if (*aout).main.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envvol = StartEnvelope(
                        &raw mut (*aout).venv,
                        (*aout).main.volflg,
                        (*i).volpts,
                        (*i).volsusbeg,
                        (*i).volsusend,
                        (*i).volbeg,
                        (*i).volend,
                        &raw mut (*i).volenv as *mut ENVPT,
                        (*aout).main.keyoff,
                    );
                }
                if (*aout).main.panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envpan = StartEnvelope(
                        &raw mut (*aout).penv,
                        (*aout).main.panflg,
                        (*i).panpts,
                        (*i).pansusbeg,
                        (*i).pansusend,
                        (*i).panbeg,
                        (*i).panend,
                        &raw mut (*i).panenv as *mut ENVPT,
                        (*aout).main.keyoff,
                    );
                }
                if (*aout).main.pitflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envpit = StartEnvelope(
                        &raw mut (*aout).cenv,
                        (*aout).main.pitflg,
                        (*i).pitpts,
                        (*i).pitsusbeg,
                        (*i).pitsusend,
                        (*i).pitbeg,
                        (*i).pitend,
                        &raw mut (*i).pitenv as *mut ENVPT,
                        (*aout).main.keyoff,
                    );
                }
                if (*aout).cenv.flg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                {
                    (*aout).masterperiod = GetPeriod(
                        (*r#mod).flags,
                        (((*aout).main.note as UWORD as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int) as UWORD,
                        (*(*aout).master).speed,
                    );
                }
            } else {
                if (*aout).main.volflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envvol = ProcessEnvelope(aout, &raw mut (*aout).venv, 256 as SWORD);
                }
                if (*aout).main.panflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envpan = ProcessEnvelope(aout, &raw mut (*aout).penv, 128 as SWORD);
                }
                if (*aout).main.pitflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    != 0
                {
                    envpit = ProcessEnvelope(aout, &raw mut (*aout).cenv, 32 as SWORD);
                }
            }
            if (*aout).main.kick as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                (*aout).main.kick_flag = 1 as UBYTE;
            }
            (*aout).main.kick = 0 as UBYTE;
            tmpvol = (*aout).main.fadevol as ULONG;
            tmpvol = tmpvol.wrapping_mul((*aout).main.chanvol as ULONG);
            tmpvol = tmpvol.wrapping_mul((*aout).main.outvolume as ULONG);
            tmpvol = tmpvol
                .wrapping_div(
                    (256 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as ULONG,
                );
            (*aout).totalvol = tmpvol >> 2 as ::core::ffi::c_int;
            tmpvol = tmpvol.wrapping_mul(envvol as ULONG);
            tmpvol = tmpvol.wrapping_mul((*r#mod).volume as ULONG);
            tmpvol = tmpvol
                .wrapping_div(
                    (128 as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                        * 128 as ::core::ffi::c_int) as ULONG,
                );
            if (*r#mod).sngpos as ::core::ffi::c_int
                >= (*r#mod).numpos as ::core::ffi::c_int
            {
                tmpvol = 0 as ULONG;
            } else {
                tmpvol = tmpvol
                    .wrapping_mul(max_volume as ULONG)
                    .wrapping_div(128 as ULONG);
            }
            if (*aout).masterchn as ::core::ffi::c_int != -1 as ::core::ffi::c_int
                && (*(*r#mod).control.offset((*aout).masterchn as isize)).muted
                    as ::core::ffi::c_int != 0
            {
                Voice_SetVolume_internal(channel as SBYTE, 0 as UWORD);
            } else {
                Voice_SetVolume_internal(channel as SBYTE, tmpvol as UWORD);
                if tmpvol != 0 && !(*aout).master.is_null()
                    && (*(*aout).master).slave == aout
                {
                    (*r#mod).realchn = (*r#mod).realchn.wrapping_add(1);
                }
                (*r#mod).totalchn = (*r#mod).totalchn.wrapping_add(1);
            }
            if (*aout).main.panning as ::core::ffi::c_int == 512 as ::core::ffi::c_int {
                Voice_SetPanning_internal(channel as SBYTE, 512 as ULONG);
            } else if (*r#mod).panflag != 0
                && (*aout).penv.flg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
            {
                Voice_SetPanning_internal(
                    channel as SBYTE,
                    DoPan(envpan, (*aout).main.panning) as ULONG,
                );
            } else {
                Voice_SetPanning_internal(
                    channel as SBYTE,
                    (*aout).main.panning as ULONG,
                );
            }
            if (*aout).main.period as ::core::ffi::c_int != 0
                && (*s).vibdepth as ::core::ffi::c_int != 0
            {
                match (*s).vibtype as ::core::ffi::c_int {
                    0 => {
                        vibval = avibtab[((*s).avibpos as ::core::ffi::c_int
                            & 127 as ::core::ffi::c_int) as usize] as SLONG;
                        if (*aout).avibpos as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int != 0
                        {
                            vibval = -vibval;
                        }
                    }
                    1 => {
                        vibval = 64 as SLONG;
                        if (*aout).avibpos as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int != 0
                        {
                            vibval = -vibval;
                        }
                    }
                    2 => {
                        vibval = (63 as ::core::ffi::c_int
                            - (((*aout).avibpos as ::core::ffi::c_int
                                + 128 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                                >> 1 as ::core::ffi::c_int)) as SLONG;
                    }
                    _ => {
                        vibval = ((((*aout).avibpos as ::core::ffi::c_int
                            + 128 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                            >> 1 as ::core::ffi::c_int) - 64 as ::core::ffi::c_int)
                            as SLONG;
                    }
                }
            } else {
                vibval = 0 as SLONG;
            }
            if (*s).vibflags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                if ((*aout).aswppos as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    < (*s).vibdepth as ::core::ffi::c_int
                {
                    (*aout).aswppos = ((*aout).aswppos as ::core::ffi::c_int
                        + (*s).vibsweep as ::core::ffi::c_int) as UWORD;
                    vibdpt = (*aout).aswppos as SLONG;
                } else {
                    vibdpt = (((*s).vibdepth as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as SLONG;
                }
                vibval = vibval * vibdpt >> 16 as ::core::ffi::c_int;
                if (*aout).mflag != 0 {
                    if (*r#mod).flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                        == 0
                    {
                        vibval >>= 1 as ::core::ffi::c_int;
                    }
                    (*aout).main.period = ((*aout).main.period as SLONG - vibval)
                        as UWORD;
                }
            } else {
                if (*aout).main.keyoff as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                    == 0
                {
                    if ((*aout).aswppos as ::core::ffi::c_int)
                        < (*s).vibsweep as ::core::ffi::c_int
                    {
                        vibdpt = ((*aout).aswppos as ::core::ffi::c_int
                            * (*s).vibdepth as ::core::ffi::c_int
                            / (*s).vibsweep as ::core::ffi::c_int) as SLONG;
                        (*aout).aswppos = (*aout).aswppos.wrapping_add(1);
                    } else {
                        vibdpt = (*s).vibdepth as SLONG;
                    }
                } else if (*aout).aswppos as ::core::ffi::c_int
                    >= (*s).vibsweep as ::core::ffi::c_int
                {
                    vibdpt = (*s).vibdepth as SLONG;
                } else {
                    vibdpt = 0 as SLONG;
                }
                vibval = vibval * vibdpt >> 8 as ::core::ffi::c_int;
                (*aout).main.period = ((*aout).main.period as SLONG - vibval) as UWORD;
            }
            (*aout).avibpos = ((*aout).avibpos as ::core::ffi::c_int
                + (*s).vibrate as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as UWORD;
            playperiod = (*aout).main.period;
            if (*aout).main.pitflg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                && envpit as ::core::ffi::c_int != 32 as ::core::ffi::c_int
            {
                let mut p1: ::core::ffi::c_long = 0;
                envpit = (envpit as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
                    as SWORD;
                if (((*aout).main.note as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    + envpit as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
                {
                    envpit = -(((*aout).main.note as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as SWORD;
                }
                p1 = (GetPeriod(
                    (*r#mod).flags,
                    ((((*aout).main.note as UWORD as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) + envpit as ::core::ffi::c_int)
                        as UWORD,
                    (*(*aout).master).speed,
                ) as ::core::ffi::c_int - (*aout).masterperiod as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
                if p1 > 0 as ::core::ffi::c_long {
                    if (playperiod as ::core::ffi::c_long + p1) as UWORD
                        as ::core::ffi::c_int <= playperiod as ::core::ffi::c_int
                    {
                        p1 = 0 as ::core::ffi::c_long;
                        (*aout).main.keyoff = ((*aout).main.keyoff as ::core::ffi::c_int
                            | 1 as ::core::ffi::c_int) as UBYTE;
                    }
                } else if p1 < 0 as ::core::ffi::c_long {
                    if (playperiod as ::core::ffi::c_long + p1) as UWORD
                        as ::core::ffi::c_int >= playperiod as ::core::ffi::c_int
                    {
                        p1 = 0 as ::core::ffi::c_long;
                        (*aout).main.keyoff = ((*aout).main.keyoff as ::core::ffi::c_int
                            | 1 as ::core::ffi::c_int) as UBYTE;
                    }
                }
                playperiod = (playperiod as ::core::ffi::c_long + p1) as UWORD;
            }
            if (*aout).main.fadevol == 0 {
                Voice_Stop_internal(channel as SBYTE);
                (*r#mod).totalchn = (*r#mod).totalchn.wrapping_sub(1);
                if tmpvol != 0 && !(*aout).master.is_null()
                    && (*(*aout).master).slave == aout
                {
                    (*r#mod).realchn = (*r#mod).realchn.wrapping_sub(1);
                }
            } else {
                Voice_SetFrequency_internal(
                    channel as SBYTE,
                    getfrequency((*r#mod).flags, playperiod as ULONG),
                );
                if !i.is_null()
                    && (*aout).main.keyoff as ::core::ffi::c_int
                        & 2 as ::core::ffi::c_int != 0
                {
                    if (*aout).main.fadevol as ::core::ffi::c_int
                        >= (*i).volfade as ::core::ffi::c_int
                    {
                        (*aout).main.fadevol = ((*aout).main.fadevol
                            as ::core::ffi::c_int - (*i).volfade as ::core::ffi::c_int)
                            as UWORD;
                    } else {
                        (*aout).main.fadevol = 0 as UWORD;
                    }
                }
            }
            md_bpm = ((*r#mod).bpm as ::core::ffi::c_int
                + (*r#mod).relspd as ::core::ffi::c_int) as UWORD;
            if (md_bpm as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                md_bpm = 32 as UWORD;
            } else if (*r#mod).flags as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int
                == 0 && md_bpm as ::core::ffi::c_int > 255 as ::core::ffi::c_int
            {
                md_bpm = 255 as UWORD;
            }
        }
        channel += 1;
    }
}
unsafe extern "C" fn pt_Notes(mut r#mod: *mut MODULE) {
    let mut channel: SWORD = 0;
    let mut a: *mut MP_CONTROL = ::core::ptr::null_mut::<MP_CONTROL>();
    let mut c: UBYTE = 0;
    let mut inst: UBYTE = 0;
    let mut tr: ::core::ffi::c_int = 0;
    let mut funky: ::core::ffi::c_int = 0;
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int) < (*r#mod).numchn as ::core::ffi::c_int {
        a = (*r#mod).control.offset(channel as isize) as *mut MP_CONTROL;
        if (*r#mod).sngpos as ::core::ffi::c_int >= (*r#mod).numpos as ::core::ffi::c_int
        {
            tr = (*r#mod).numtrk as ::core::ffi::c_int;
            (*r#mod).numrow = 0 as UWORD;
        } else {
            tr = *(*r#mod)
                .patterns
                .offset(
                    (*(*r#mod).positions.offset((*r#mod).sngpos as isize)
                        as ::core::ffi::c_int * (*r#mod).numchn as ::core::ffi::c_int
                        + channel as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int;
            (*r#mod).numrow = *(*r#mod)
                .pattrows
                .offset(*(*r#mod).positions.offset((*r#mod).sngpos as isize) as isize);
        }
        (*a).row = if tr < (*r#mod).numtrk as ::core::ffi::c_int {
            UniFindRow(*(*r#mod).tracks.offset(tr as isize), (*r#mod).patpos)
        } else {
            ::core::ptr::null_mut::<UBYTE>()
        };
        (*a).newsamp = 0 as UWORD;
        if (*r#mod).vbtick == 0 {
            (*a).main.notedelay = 0 as UBYTE;
        }
        's_12: {
            if !(*a).row.is_null() {
                UniSetRow((*a).row);
                funky = 0 as ::core::ffi::c_int;
                loop {
                    c = UniGetByte();
                    if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        break;
                    }
                    match c as ::core::ffi::c_int {
                        1 => {
                            funky |= 1 as ::core::ffi::c_int;
                            (*a).oldnote = (*a).anote;
                            (*a).anote = UniGetByte();
                            (*a).main.kick = 1 as UBYTE;
                            (*a).main.start = -1 as SLONG;
                            (*a).sliding = 0 as SBYTE;
                            if (*a).wavecontrol as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int == 0
                            {
                                (*a).trmpos = 0 as SBYTE;
                            }
                            if (*a).wavecontrol as ::core::ffi::c_int
                                & 0x8 as ::core::ffi::c_int == 0
                            {
                                (*a).vibpos = 0 as SBYTE;
                            }
                            if (*a).panbwave == 0 {
                                (*a).panbpos = 0 as UBYTE;
                            }
                        }
                        2 => {
                            inst = UniGetByte();
                            if (inst as ::core::ffi::c_int)
                                < (*r#mod).numins as ::core::ffi::c_int
                            {
                                funky |= 2 as ::core::ffi::c_int;
                                (*a).main.i = (if (*r#mod).flags as ::core::ffi::c_int
                                    & 0x4 as ::core::ffi::c_int != 0
                                {
                                    (*r#mod).instruments.offset(inst as isize)
                                } else {
                                    ::core::ptr::null_mut::<INSTRUMENT>()
                                }) as *mut INSTRUMENT;
                                (*a).retrig = 0 as SBYTE;
                                (*a).s3mtremor = 0 as UBYTE;
                                (*a).ultoffset = 0 as UWORD;
                                (*a).main.sample = inst;
                            }
                        }
                        _ => {
                            UniSkipOpcode();
                        }
                    }
                }
                if funky != 0 {
                    let mut i: *mut INSTRUMENT = ::core::ptr::null_mut::<INSTRUMENT>();
                    let mut s: *mut SAMPLE = ::core::ptr::null_mut::<SAMPLE>();
                    i = (*a).main.i;
                    if !i.is_null() {
                        if (*i).samplenumber[(*a).anote as usize] as ::core::ffi::c_int
                            >= (*r#mod).numsmp as ::core::ffi::c_int
                        {
                            break 's_12;
                        } else {
                            s = (*r#mod)
                                .samples
                                .offset((*i).samplenumber[(*a).anote as usize] as isize)
                                as *mut SAMPLE;
                            (*a).main.note = (*i).samplenote[(*a).anote as usize];
                        }
                    } else {
                        (*a).main.note = (*a).anote;
                        s = (*r#mod).samples.offset((*a).main.sample as isize)
                            as *mut SAMPLE;
                    }
                    if (*a).main.s != s {
                        (*a).main.s = s;
                        (*a).newsamp = (*a).main.period;
                    }
                    (*a).main.panning = (*r#mod).panning[channel as usize] as SWORD;
                    if (*s).flags as ::core::ffi::c_int & 0x1000 as ::core::ffi::c_int
                        != 0
                    {
                        (*a).main.panning = (*s).panning;
                    } else if !i.is_null()
                        && (*i).flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                            != 0
                    {
                        (*a).main.panning = (*i).panning;
                    }
                    (*a).main.handle = (*s).handle;
                    (*a).speed = (*s).speed;
                    if !i.is_null() {
                        if (*r#mod).panflag != 0
                            && (*i).flags as ::core::ffi::c_int & 2 as ::core::ffi::c_int
                                != 0
                            && (*a).main.panning as ::core::ffi::c_int
                                != 512 as ::core::ffi::c_int
                        {
                            (*a).main.panning = ((*a).main.panning as ::core::ffi::c_int
                                + ((*a).anote as ::core::ffi::c_int
                                    - (*i).pitpancenter as ::core::ffi::c_int)
                                    * (*i).pitpansep as ::core::ffi::c_int
                                    / 8 as ::core::ffi::c_int) as SWORD;
                            if ((*a).main.panning as ::core::ffi::c_int)
                                < 0 as ::core::ffi::c_int
                            {
                                (*a).main.panning = 0 as SWORD;
                            } else if (*a).main.panning as ::core::ffi::c_int
                                > 255 as ::core::ffi::c_int
                            {
                                (*a).main.panning = 255 as SWORD;
                            }
                        }
                        (*a).main.pitflg = (*i).pitflg;
                        (*a).main.volflg = (*i).volflg;
                        (*a).main.panflg = (*i).panflg;
                        (*a).main.nna = (*i).nnatype;
                        (*a).dca = (*i).dca;
                        (*a).dct = (*i).dct;
                    } else {
                        (*a).main.panflg = 0 as UBYTE;
                        (*a).main.volflg = (*a).main.panflg;
                        (*a).main.pitflg = (*a).main.volflg;
                        (*a).dca = 0 as UBYTE;
                        (*a).main.nna = (*a).dca;
                        (*a).dct = 0 as UBYTE;
                    }
                    if funky & 2 as ::core::ffi::c_int != 0 {
                        (*a).tmpvolume = (*s).volume as SWORD;
                        (*a).volume = (*a).tmpvolume;
                        if !s.is_null() && !i.is_null() {
                            if (*i).rvolvar != 0 {
                                (*a).tmpvolume = ((*s).volume as SLONG
                                    + (*s).volume as SLONG
                                        * ((*i).rvolvar as SLONG
                                            * getrandom(512 as ::core::ffi::c_int) as SLONG)
                                        / 25600 as SLONG) as SWORD;
                                (*a).volume = (*a).tmpvolume;
                                if ((*a).volume as ::core::ffi::c_int)
                                    < 0 as ::core::ffi::c_int
                                {
                                    (*a).tmpvolume = 0 as SWORD;
                                    (*a).volume = (*a).tmpvolume;
                                } else if (*a).volume as ::core::ffi::c_int
                                    > 64 as ::core::ffi::c_int
                                {
                                    (*a).tmpvolume = 64 as SWORD;
                                    (*a).volume = (*a).tmpvolume;
                                }
                            }
                            if (*r#mod).panflag != 0
                                && (*a).main.panning as ::core::ffi::c_int
                                    != 512 as ::core::ffi::c_int
                            {
                                (*a).main.panning = ((*a).main.panning as SLONG
                                    + (*a).main.panning as SLONG
                                        * ((*i).rpanvar as SLONG
                                            * getrandom(512 as ::core::ffi::c_int) as SLONG)
                                        / 25600 as SLONG) as SWORD;
                                if ((*a).main.panning as ::core::ffi::c_int)
                                    < 0 as ::core::ffi::c_int
                                {
                                    (*a).main.panning = 0 as SWORD;
                                } else if (*a).main.panning as ::core::ffi::c_int
                                    > 255 as ::core::ffi::c_int
                                {
                                    (*a).main.panning = 255 as SWORD;
                                }
                            }
                        }
                    }
                    (*a).tmpperiod = GetPeriod(
                        (*r#mod).flags,
                        (((*a).main.note as UWORD as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int) as UWORD,
                        (*a).speed,
                    );
                    (*a).wantedperiod = (*a).tmpperiod;
                    (*a).main.keyoff = 0 as UBYTE;
                }
            }
        }
        channel += 1;
    }
}
unsafe extern "C" fn pt_EffectsPass1(mut r#mod: *mut MODULE) {
    let mut channel: SWORD = 0;
    let mut a: *mut MP_CONTROL = ::core::ptr::null_mut::<MP_CONTROL>();
    let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
    let mut explicitslides: ::core::ffi::c_int = 0;
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int) < (*r#mod).numchn as ::core::ffi::c_int {
        a = (*r#mod).control.offset(channel as isize) as *mut MP_CONTROL;
        aout = (*a).slave as *mut MP_VOICE;
        if !aout.is_null() {
            (*a).main.fadevol = (*aout).main.fadevol;
            (*a).main.period = (*aout).main.period;
            if (*a).main.kick as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                (*a).main.keyoff = (*aout).main.keyoff;
            }
        }
        if !(*a).row.is_null() {
            UniSetRow((*a).row);
            (*a).ownvol = 0 as SWORD;
            (*a).ownper = (*a).ownvol;
            explicitslides = pt_playeffects(r#mod, channel, a);
            if (*r#mod).flags as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                if explicitslides == 0 && (*a).sliding as ::core::ffi::c_int != 0 {
                    DoS3MVolSlide((*r#mod).vbtick, (*r#mod).flags, a, 0 as UBYTE);
                } else if (*a).tmpvolume != 0 {
                    (*a).sliding = explicitslides as SBYTE;
                }
            }
            if (*a).ownper == 0 {
                (*a).main.period = (*a).tmpperiod;
            }
            if (*a).ownvol == 0 {
                (*a).volume = (*a).tmpvolume;
            }
            if !(*a).main.s.is_null() {
                if !(*a).main.i.is_null() {
                    (*a).main.outvolume = ((*a).volume as ::core::ffi::c_int
                        * (*(*a).main.s).globvol as ::core::ffi::c_int
                        * (*(*a).main.i).globvol as ::core::ffi::c_int
                        >> 10 as ::core::ffi::c_int) as SWORD;
                } else {
                    (*a).main.outvolume = ((*a).volume as ::core::ffi::c_int
                        * (*(*a).main.s).globvol as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int) as SWORD;
                }
                if (*a).main.outvolume as ::core::ffi::c_int > 256 as ::core::ffi::c_int
                {
                    (*a).main.outvolume = 256 as SWORD;
                } else if ((*a).main.outvolume as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    (*a).main.outvolume = 0 as SWORD;
                }
            }
        }
        channel += 1;
    }
}
unsafe extern "C" fn pt_NNA(mut r#mod: *mut MODULE) {
    let mut channel: SWORD = 0;
    let mut a: *mut MP_CONTROL = ::core::ptr::null_mut::<MP_CONTROL>();
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int) < (*r#mod).numchn as ::core::ffi::c_int {
        a = (*r#mod).control.offset(channel as isize) as *mut MP_CONTROL;
        if (*a).main.kick as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut kill: BOOL = 0 as BOOL;
            if !(*a).slave.is_null() {
                let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
                aout = (*a).slave as *mut MP_VOICE;
                if (*aout).main.nna as ::core::ffi::c_int & 3 as ::core::ffi::c_int != 0
                {
                    (*a).slave = ::core::ptr::null_mut::<MP_VOICE>();
                    (*aout).mflag = 0 as ::core::ffi::c_int as BOOL;
                    match (*aout).main.nna as ::core::ffi::c_int {
                        2 => {
                            (*aout).main.keyoff = ((*aout).main.keyoff
                                as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as UBYTE;
                            if (*aout).main.volflg as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int == 0
                                || (*aout).main.volflg as ::core::ffi::c_int
                                    & 4 as ::core::ffi::c_int != 0
                            {
                                (*aout).main.keyoff = (1 as ::core::ffi::c_int
                                    | 2 as ::core::ffi::c_int) as UBYTE;
                            }
                        }
                        3 => {
                            (*aout).main.keyoff = ((*aout).main.keyoff
                                as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as UBYTE;
                        }
                        1 | _ => {}
                    }
                }
            }
            if (*a).dct as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                let mut t: ::core::ffi::c_int = 0;
                t = 0 as ::core::ffi::c_int;
                while t
                    < if (md_sngchn as ::core::ffi::c_int)
                        < (*r#mod).numvoices as ::core::ffi::c_int
                    {
                        md_sngchn as ::core::ffi::c_int
                    } else {
                        (*r#mod).numvoices as ::core::ffi::c_int
                    }
                {
                    if Voice_Stopped_internal(t as SBYTE) == 0
                        && (*(*r#mod).voice.offset(t as isize)).masterchn
                            as ::core::ffi::c_int == channel as ::core::ffi::c_int
                        && (*a).main.sample as ::core::ffi::c_int
                            == (*(*r#mod).voice.offset(t as isize)).main.sample
                                as ::core::ffi::c_int
                    {
                        kill = 0 as ::core::ffi::c_int as BOOL;
                        match (*a).dct as ::core::ffi::c_int {
                            1 => {
                                if (*a).main.note as ::core::ffi::c_int
                                    == (*(*r#mod).voice.offset(t as isize)).main.note
                                        as ::core::ffi::c_int
                                {
                                    kill = 1 as ::core::ffi::c_int as BOOL;
                                }
                            }
                            2 => {
                                if (*a).main.handle as ::core::ffi::c_int
                                    == (*(*r#mod).voice.offset(t as isize)).main.handle
                                        as ::core::ffi::c_int
                                {
                                    kill = 1 as ::core::ffi::c_int as BOOL;
                                }
                            }
                            3 => {
                                kill = 1 as ::core::ffi::c_int as BOOL;
                            }
                            _ => {}
                        }
                        if kill != 0 {
                            match (*a).dca as ::core::ffi::c_int {
                                0 => {
                                    (*(*r#mod).voice.offset(t as isize)).main.fadevol = 0
                                        as UWORD;
                                }
                                1 => {
                                    (*(*r#mod).voice.offset(t as isize)).main.keyoff = ((*(*r#mod)
                                        .voice
                                        .offset(t as isize))
                                        .main
                                        .keyoff as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                                        as UBYTE;
                                    if (*(*r#mod).voice.offset(t as isize)).main.volflg
                                        as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                                        || (*(*r#mod).voice.offset(t as isize)).main.volflg
                                            as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0
                                    {
                                        (*(*r#mod).voice.offset(t as isize)).main.keyoff = (1
                                            as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as UBYTE;
                                    }
                                }
                                2 => {
                                    (*(*r#mod).voice.offset(t as isize)).main.keyoff = ((*(*r#mod)
                                        .voice
                                        .offset(t as isize))
                                        .main
                                        .keyoff as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
                                        as UBYTE;
                                }
                                _ => {}
                            }
                        }
                    }
                    t += 1;
                }
            }
        }
        channel += 1;
    }
}
unsafe extern "C" fn pt_SetupVoices(mut r#mod: *mut MODULE) {
    let mut channel: SWORD = 0;
    let mut a: *mut MP_CONTROL = ::core::ptr::null_mut::<MP_CONTROL>();
    let mut aout: *mut MP_VOICE = ::core::ptr::null_mut::<MP_VOICE>();
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int) < (*r#mod).numchn as ::core::ffi::c_int {
        a = (*r#mod).control.offset(channel as isize) as *mut MP_CONTROL;
        if (*a).main.notedelay == 0 {
            if (*a).main.kick as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                if (*r#mod).flags as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                {
                    if (*a).slave.is_null() {
                        let mut newchn: ::core::ffi::c_int = 0;
                        newchn = MP_FindEmptyChannel(r#mod);
                        if newchn != -1 as ::core::ffi::c_int {
                            (*a).slavechn = newchn as UBYTE;
                            (*a).slave = (*r#mod).voice.offset((*a).slavechn as isize);
                        }
                    }
                } else {
                    (*a).slavechn = channel as UBYTE;
                    (*a).slave = (*r#mod).voice.offset((*a).slavechn as isize);
                }
                aout = (*a).slave as *mut MP_VOICE;
                if !aout.is_null() {
                    if (*aout).mflag != 0 && !(*aout).master.is_null() {
                        (*(*aout).master).slave = ::core::ptr::null_mut::<MP_VOICE>();
                    }
                    (*aout).master = a;
                    (*a).slave = aout as *mut MP_VOICE;
                    (*aout).masterchn = channel;
                    (*aout).mflag = 1 as ::core::ffi::c_int as BOOL;
                }
            } else {
                aout = (*a).slave as *mut MP_VOICE;
            }
            if !aout.is_null() {
                (*aout).main = (*a).main;
            }
            (*a).main.kick = 0 as UBYTE;
        }
        channel += 1;
    }
}
unsafe extern "C" fn pt_EffectsPass2(mut r#mod: *mut MODULE) {
    let mut channel: SWORD = 0;
    let mut a: *mut MP_CONTROL = ::core::ptr::null_mut::<MP_CONTROL>();
    let mut c: UBYTE = 0;
    channel = 0 as SWORD;
    while (channel as ::core::ffi::c_int) < (*r#mod).numchn as ::core::ffi::c_int {
        a = (*r#mod).control.offset(channel as isize) as *mut MP_CONTROL;
        if !(*a).row.is_null() {
            UniSetRow((*a).row);
            loop {
                c = UniGetByte();
                if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                if c as ::core::ffi::c_int
                    == C2Rust_Unnamed::UNI_ITEFFECTS0.0 as ::core::ffi::c_int
                {
                    c = UniGetByte();
                    if c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::SS_S7EFFECTS.0 as ::core::ffi::c_int
                    {
                        DoNNAEffects(
                            r#mod,
                            a,
                            (c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                as UBYTE,
                        );
                    }
                } else {
                    UniSkipOpcode();
                }
            }
        }
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_HandleTick() {
    let mut channel: SWORD = 0;
    let mut max_volume: ::core::ffi::c_int = 0;
    if pf.is_null() || (*pf).forbid != 0
        || (*pf).sngpos as ::core::ffi::c_int >= (*pf).numpos as ::core::ffi::c_int
    {
        return;
    }
    (*pf).sngremainder = ((*pf).sngremainder as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
            * 5 as ::core::ffi::c_int) as UWORD;
    (*pf).sngtime = (*pf)
        .sngtime
        .wrapping_add(
            ((*pf).sngremainder as ::core::ffi::c_int / (*pf).bpm as ::core::ffi::c_int)
                as ULONG,
        );
    (*pf).sngremainder = ((*pf).sngremainder as ::core::ffi::c_int
        % (*pf).bpm as ::core::ffi::c_int) as UWORD;
    (*pf).vbtick = (*pf).vbtick.wrapping_add(1);
    if (*pf).vbtick as ::core::ffi::c_int >= (*pf).sngspd as ::core::ffi::c_int {
        if (*pf).pat_repcrazy != 0 {
            (*pf).pat_repcrazy = 0 as UBYTE;
        } else {
            (*pf).patpos = (*pf).patpos.wrapping_add(1);
        }
        (*pf).vbtick = 0 as UWORD;
        if (*pf).patdly != 0 {
            (*pf).patdly2 = (*pf).patdly;
            (*pf).patdly = 0 as UBYTE;
        }
        if (*pf).patdly2 != 0 {
            (*pf).patdly2 = (*pf).patdly2.wrapping_sub(1);
            if (*pf).patdly2 != 0 {
                if (*pf).patpos != 0 {
                    (*pf).patpos = (*pf).patpos.wrapping_sub(1);
                }
            }
        }
        if (*pf).patpos as ::core::ffi::c_int >= (*pf).numrow as ::core::ffi::c_int
            && (*pf).numrow as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*pf).posjmp == 0
        {
            (*pf).posjmp = 3 as SWORD;
        }
        if (*pf).posjmp != 0 {
            (*pf).patpos = (if (*pf).numrow as ::core::ffi::c_int != 0 {
                (*pf).patbrk as ::core::ffi::c_int % (*pf).numrow as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as UWORD;
            (*pf).pat_repcrazy = 0 as UBYTE;
            (*pf).sngpos = ((*pf).sngpos as ::core::ffi::c_int
                + ((*pf).posjmp as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                as SWORD;
            channel = 0 as SWORD;
            while (channel as ::core::ffi::c_int) < (*pf).numchn as ::core::ffi::c_int {
                (*(*pf).control.offset(channel as isize)).pat_reppos = -1 as SWORD;
                channel += 1;
            }
            (*pf).posjmp = 0 as SWORD;
            (*pf).patbrk = (*pf).posjmp as UWORD;
            if ((*pf).sngpos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                (*pf).sngpos = ((*pf).numpos as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as SWORD;
            }
            if (*pf).sngpos as ::core::ffi::c_int >= (*pf).numpos as ::core::ffi::c_int
                || *(*pf).positions.offset((*pf).sngpos as isize) as ::core::ffi::c_int
                    == -1 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
            {
                if (*pf).wrap == 0 {
                    return;
                }
                (*pf).sngpos = (*pf).reppos as SWORD;
                if (*pf).sngpos == 0 {
                    (*pf).volume = (if (*pf).initvolume as ::core::ffi::c_int
                        > 128 as ::core::ffi::c_int
                    {
                        128 as ::core::ffi::c_int
                    } else {
                        (*pf).initvolume as ::core::ffi::c_int
                    }) as SWORD;
                    if (*pf).initspeed as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        (*pf).sngspd = (if ((*pf).initspeed as ::core::ffi::c_int)
                            < 32 as ::core::ffi::c_int
                        {
                            (*pf).initspeed as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        }) as UWORD;
                    } else {
                        (*pf).sngspd = 6 as UWORD;
                    }
                    (*pf).bpm = (if ((*pf).inittempo as ::core::ffi::c_int)
                        < 32 as ::core::ffi::c_int
                    {
                        32 as ::core::ffi::c_int
                    } else {
                        (*pf).inittempo as ::core::ffi::c_int
                    }) as UWORD;
                }
            }
        }
        if (*pf).patdly2 == 0 {
            pt_Notes(pf);
        }
    }
    if ((*pf).sngpos as ::core::ffi::c_int
        == (*pf).numpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        || *(*pf)
            .positions
            .offset(
                ((*pf).sngpos as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            == -1 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int)
        && (*pf).fadeout != 0
    {
        max_volume = if (*pf).numrow as ::core::ffi::c_int != 0 {
            ((*pf).numrow as ::core::ffi::c_int - (*pf).patpos as ::core::ffi::c_int)
                * 128 as ::core::ffi::c_int / (*pf).numrow as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        max_volume = 128 as ::core::ffi::c_int;
    }
    pt_EffectsPass1(pf);
    if (*pf).flags as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0 {
        pt_NNA(pf);
    }
    pt_SetupVoices(pf);
    pt_EffectsPass2(pf);
    pt_UpdateVoices(pf, max_volume);
}
unsafe extern "C" fn Player_Init_internal(mut r#mod: *mut MODULE) {
    let mut t: ::core::ffi::c_int = 0;
    t = 0 as ::core::ffi::c_int;
    while t < (*r#mod).numchn as ::core::ffi::c_int {
        (*(*r#mod).control.offset(t as isize)).main.chanvol = (*r#mod)
            .chanvol[t as usize] as SBYTE;
        (*(*r#mod).control.offset(t as isize)).main.panning = (*r#mod)
            .panning[t as usize] as SWORD;
        t += 1;
    }
    (*r#mod).sngtime = 0 as ULONG;
    (*r#mod).sngremainder = 0 as UWORD;
    (*r#mod).pat_repcrazy = 0 as UBYTE;
    (*r#mod).sngpos = 0 as SWORD;
    if (*r#mod).initspeed as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        (*r#mod).sngspd = (if ((*r#mod).initspeed as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            (*r#mod).initspeed as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        }) as UWORD;
    } else {
        (*r#mod).sngspd = 6 as UWORD;
    }
    (*r#mod).volume = (if (*r#mod).initvolume as ::core::ffi::c_int
        > 128 as ::core::ffi::c_int
    {
        128 as ::core::ffi::c_int
    } else {
        (*r#mod).initvolume as ::core::ffi::c_int
    }) as SWORD;
    (*r#mod).vbtick = (*r#mod).sngspd;
    (*r#mod).patdly = 0 as UBYTE;
    (*r#mod).patdly2 = 0 as UBYTE;
    (*r#mod).bpm = (if ((*r#mod).inittempo as ::core::ffi::c_int)
        < 32 as ::core::ffi::c_int
    {
        32 as ::core::ffi::c_int
    } else {
        (*r#mod).inittempo as ::core::ffi::c_int
    }) as UWORD;
    (*r#mod).realchn = 0 as UBYTE;
    (*r#mod).patpos = 0 as UWORD;
    (*r#mod).posjmp = 2 as SWORD;
    (*r#mod).numrow = -1 as ::core::ffi::c_int as UWORD;
    (*r#mod).patbrk = 0 as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Init(mut r#mod: *mut MODULE) -> ::core::ffi::c_int {
    (*r#mod).extspd = 1 as ::core::ffi::c_int as BOOL;
    (*r#mod).panflag = 1 as ::core::ffi::c_int as BOOL;
    (*r#mod).wrap = 0 as ::core::ffi::c_int as BOOL;
    (*r#mod).r#loop = 1 as ::core::ffi::c_int as BOOL;
    (*r#mod).fadeout = 0 as ::core::ffi::c_int as BOOL;
    (*r#mod).relspd = 0 as SWORD;
    (*r#mod).control = MikMod_calloc(
        (*r#mod).numchn as size_t,
        ::core::mem::size_of::<MP_CONTROL>() as size_t,
    ) as *mut MP_CONTROL as *mut MP_CONTROL;
    if (*r#mod).control.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    (*r#mod).voice = MikMod_calloc(
        md_sngchn as size_t,
        ::core::mem::size_of::<MP_VOICE>() as size_t,
    ) as *mut MP_VOICE as *mut MP_VOICE;
    if (*r#mod).voice.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    (*r#mod).numvoices = md_sngchn;
    Player_Init_internal(r#mod);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Exit_internal(mut r#mod: *mut MODULE) {
    if r#mod.is_null() {
        return;
    }
    if r#mod == pf {
        Player_Stop_internal();
        pf = ::core::ptr::null_mut::<MODULE>();
    }
    MikMod_free((*r#mod).control as *mut ::core::ffi::c_void);
    MikMod_free((*r#mod).voice as *mut ::core::ffi::c_void);
    (*r#mod).control = ::core::ptr::null_mut::<MP_CONTROL>();
    (*r#mod).voice = ::core::ptr::null_mut::<MP_VOICE>();
}
#[no_mangle]
pub unsafe extern "C" fn Player_Exit(mut r#mod: *mut MODULE) {
    Player_Exit_internal(r#mod);
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetVolume(mut volume: SWORD) {
    if !pf.is_null() {
        (*pf).volume = (if (volume as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else if volume as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
            128 as ::core::ffi::c_int
        } else {
            volume as ::core::ffi::c_int
        }) as SWORD;
        (*pf).initvolume = (*pf).volume as UBYTE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetModule() -> *mut MODULE {
    let mut result: *mut MODULE = ::core::ptr::null_mut::<MODULE>();
    result = pf;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Start(mut r#mod: *mut MODULE) {
    let mut t: ::core::ffi::c_int = 0;
    if r#mod.is_null() {
        return;
    }
    if MikMod_Active() == 0 {
        MikMod_EnableOutput();
    }
    (*r#mod).forbid = 0 as ::core::ffi::c_int as BOOL;
    if pf != r#mod {
        if !pf.is_null() {
            (*pf).forbid = 1 as ::core::ffi::c_int as BOOL;
        }
        t = 0 as ::core::ffi::c_int;
        while t < md_sngchn as ::core::ffi::c_int {
            Voice_Stop_internal(t as SBYTE);
            t += 1;
        }
    }
    pf = r#mod;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Stop_internal() {
    if md_sfxchn == 0 {
        MikMod_DisableOutput_internal();
    }
    if !pf.is_null() {
        (*pf).forbid = 1 as ::core::ffi::c_int as BOOL;
    }
    pf = ::core::ptr::null_mut::<MODULE>();
}
#[no_mangle]
pub unsafe extern "C" fn Player_Stop() {
    Player_Stop_internal();
}
#[no_mangle]
pub unsafe extern "C" fn Player_Active() -> BOOL {
    let mut result: BOOL = 0 as BOOL;
    if !pf.is_null() {
        result = !((*pf).sngpos as ::core::ffi::c_int
            >= (*pf).numpos as ::core::ffi::c_int) as ::core::ffi::c_int as BOOL;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_NextPosition() {
    if !pf.is_null() {
        let mut t: ::core::ffi::c_int = 0;
        (*pf).forbid = 1 as ::core::ffi::c_int as BOOL;
        (*pf).posjmp = 3 as SWORD;
        (*pf).patbrk = 0 as UWORD;
        (*pf).vbtick = (*pf).sngspd;
        t = 0 as ::core::ffi::c_int;
        while t
            < if (md_sngchn as ::core::ffi::c_int)
                < (*pf).numvoices as ::core::ffi::c_int
            {
                md_sngchn as ::core::ffi::c_int
            } else {
                (*pf).numvoices as ::core::ffi::c_int
            }
        {
            Voice_Stop_internal(t as SBYTE);
            (*(*pf).voice.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).voice.offset(t as isize)).main.s = ::core::ptr::null_mut::<SAMPLE>();
            t += 1;
        }
        t = 0 as ::core::ffi::c_int;
        while t < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).control.offset(t as isize)).main.s = ::core::ptr::null_mut::<
                SAMPLE,
            >();
            t += 1;
        }
        (*pf).forbid = 0 as ::core::ffi::c_int as BOOL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_PrevPosition() {
    if !pf.is_null() {
        let mut t: ::core::ffi::c_int = 0;
        (*pf).forbid = 1 as ::core::ffi::c_int as BOOL;
        (*pf).posjmp = 1 as SWORD;
        (*pf).patbrk = 0 as UWORD;
        (*pf).vbtick = (*pf).sngspd;
        t = 0 as ::core::ffi::c_int;
        while t
            < if (md_sngchn as ::core::ffi::c_int)
                < (*pf).numvoices as ::core::ffi::c_int
            {
                md_sngchn as ::core::ffi::c_int
            } else {
                (*pf).numvoices as ::core::ffi::c_int
            }
        {
            Voice_Stop_internal(t as SBYTE);
            (*(*pf).voice.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).voice.offset(t as isize)).main.s = ::core::ptr::null_mut::<SAMPLE>();
            t += 1;
        }
        t = 0 as ::core::ffi::c_int;
        while t < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).control.offset(t as isize)).main.s = ::core::ptr::null_mut::<
                SAMPLE,
            >();
            t += 1;
        }
        (*pf).forbid = 0 as ::core::ffi::c_int as BOOL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetPosition(mut pos: UWORD) {
    if !pf.is_null() {
        let mut t: ::core::ffi::c_int = 0;
        (*pf).forbid = 1 as ::core::ffi::c_int as BOOL;
        if pos as ::core::ffi::c_int >= (*pf).numpos as ::core::ffi::c_int {
            pos = (*pf).numpos;
        }
        (*pf).posjmp = 2 as SWORD;
        (*pf).patbrk = 0 as UWORD;
        (*pf).sngpos = pos as SWORD;
        (*pf).vbtick = (*pf).sngspd;
        t = 0 as ::core::ffi::c_int;
        while t
            < if (md_sngchn as ::core::ffi::c_int)
                < (*pf).numvoices as ::core::ffi::c_int
            {
                md_sngchn as ::core::ffi::c_int
            } else {
                (*pf).numvoices as ::core::ffi::c_int
            }
        {
            Voice_Stop_internal(t as SBYTE);
            (*(*pf).voice.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).voice.offset(t as isize)).main.s = ::core::ptr::null_mut::<SAMPLE>();
            t += 1;
        }
        t = 0 as ::core::ffi::c_int;
        while t < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(t as isize)).main.i = ::core::ptr::null_mut::<
                INSTRUMENT,
            >();
            (*(*pf).control.offset(t as isize)).main.s = ::core::ptr::null_mut::<
                SAMPLE,
            >();
            t += 1;
        }
        (*pf).forbid = 0 as ::core::ffi::c_int as BOOL;
        if pos == 0 {
            Player_Init_internal(pf);
        }
    }
}
unsafe extern "C" fn Player_Unmute_internal(
    mut arg1: SLONG,
    mut ap: ::core::ffi::VaList,
) {
    let mut t: SLONG = 0;
    let mut arg2: SLONG = 0;
    let mut arg3: SLONG = 0 as SLONG;
    if !pf.is_null() {
        match arg1 {
            32001 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                while arg2 < (*pf).numchn as SLONG && arg2 <= arg3 {
                    (*(*pf).control.offset(arg2 as isize)).muted = 0 as UBYTE;
                    arg2 += 1;
                }
            }
            32000 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                t = 0 as SLONG;
                while t < (*pf).numchn as SLONG {
                    if !(t >= arg2 && t <= arg3) {
                        (*(*pf).control.offset(t as isize)).muted = 0 as UBYTE;
                    }
                    t += 1;
                }
            }
            _ => {
                if arg1 < (*pf).numchn as SLONG {
                    (*(*pf).control.offset(arg1 as isize)).muted = 0 as UBYTE;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_Unmute(mut arg1: SLONG, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaListImpl;
    args = c2rust_args.clone();
    Player_Unmute_internal(arg1, args.as_va_list());
}
unsafe extern "C" fn Player_Mute_internal(mut arg1: SLONG, mut ap: ::core::ffi::VaList) {
    let mut t: SLONG = 0;
    let mut arg2: SLONG = 0;
    let mut arg3: SLONG = 0 as SLONG;
    if !pf.is_null() {
        match arg1 {
            32001 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                while arg2 < (*pf).numchn as SLONG && arg2 <= arg3 {
                    (*(*pf).control.offset(arg2 as isize)).muted = 1 as UBYTE;
                    arg2 += 1;
                }
            }
            32000 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                t = 0 as SLONG;
                while t < (*pf).numchn as SLONG {
                    if !(t >= arg2 && t <= arg3) {
                        (*(*pf).control.offset(t as isize)).muted = 1 as UBYTE;
                    }
                    t += 1;
                }
            }
            _ => {
                if arg1 < (*pf).numchn as SLONG {
                    (*(*pf).control.offset(arg1 as isize)).muted = 1 as UBYTE;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_Mute(mut arg1: SLONG, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaListImpl;
    args = c2rust_args.clone();
    Player_Mute_internal(arg1, args.as_va_list());
}
unsafe extern "C" fn Player_ToggleMute_internal(
    mut arg1: SLONG,
    mut ap: ::core::ffi::VaList,
) {
    let mut arg2: SLONG = 0;
    let mut arg3: SLONG = 0 as SLONG;
    let mut t: ULONG = 0;
    if !pf.is_null() {
        match arg1 {
            32001 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                while arg2 < (*pf).numchn as SLONG && arg2 <= arg3 {
                    (*(*pf).control.offset(arg2 as isize)).muted = (1
                        as ::core::ffi::c_int
                        - (*(*pf).control.offset(arg2 as isize)).muted
                            as ::core::ffi::c_int) as UBYTE;
                    arg2 += 1;
                }
            }
            32000 => {
                arg2 = ap.arg::<SLONG>();
                if arg2 == 0
                    && {
                        arg3 = ap.arg::<SLONG>();
                        arg3 == 0
                    } || arg2 > arg3 || arg3 >= (*pf).numchn as SLONG
                {
                    return;
                }
                t = 0 as ULONG;
                while t < (*pf).numchn as ULONG {
                    if !(t >= arg2 as ULONG && t <= arg3 as ULONG) {
                        (*(*pf).control.offset(t as isize)).muted = (1
                            as ::core::ffi::c_int
                            - (*(*pf).control.offset(t as isize)).muted
                                as ::core::ffi::c_int) as UBYTE;
                    }
                    t = t.wrapping_add(1);
                }
            }
            _ => {
                if arg1 < (*pf).numchn as SLONG {
                    (*(*pf).control.offset(arg1 as isize)).muted = (1
                        as ::core::ffi::c_int
                        - (*(*pf).control.offset(arg1 as isize)).muted
                            as ::core::ffi::c_int) as UBYTE;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_ToggleMute(mut arg1: SLONG, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaListImpl;
    args = c2rust_args.clone();
    Player_ToggleMute_internal(arg1, args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn Player_Muted(mut chan: UBYTE) -> BOOL {
    let mut result: BOOL = 1 as BOOL;
    if !pf.is_null() {
        result = (if (chan as ::core::ffi::c_int) < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(chan as isize)).muted as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as BOOL;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetChannelVoice(mut chan: UBYTE) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !pf.is_null() {
        result = if (chan as ::core::ffi::c_int) < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(chan as isize)).slavechn as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetChannelPeriod(mut chan: UBYTE) -> UWORD {
    let mut result: UWORD = 0 as UWORD;
    if !pf.is_null() {
        result = (if (chan as ::core::ffi::c_int) < (*pf).numchn as ::core::ffi::c_int {
            (*(*pf).control.offset(chan as isize)).main.period as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as UWORD;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_Paused_internal() -> BOOL {
    return if !pf.is_null() { (*pf).forbid } else { 1 as BOOL };
}
#[no_mangle]
pub unsafe extern "C" fn Player_Paused() -> BOOL {
    let mut result: BOOL = 0;
    result = Player_Paused_internal();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Player_TogglePause() {
    if !pf.is_null() {
        (*pf).forbid = 1 as BOOL - (*pf).forbid;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetSpeed(mut speed: UWORD) {
    if !pf.is_null() {
        (*pf).sngspd = (if speed as ::core::ffi::c_int != 0 {
            if (speed as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                speed as ::core::ffi::c_int
            } else {
                32 as ::core::ffi::c_int
            }
        } else {
            1 as ::core::ffi::c_int
        }) as UWORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetTempo(mut tempo: UWORD) {
    if (tempo as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        tempo = 32 as UWORD;
    }
    if !pf.is_null() {
        if (*pf).flags as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int == 0
            && tempo as ::core::ffi::c_int > 255 as ::core::ffi::c_int
        {
            tempo = 255 as UWORD;
        }
        (*pf).bpm = tempo;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Player_QueryVoices(
    mut numvoices: UWORD,
    mut vinfo: *mut VOICEINFO,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if numvoices as ::core::ffi::c_int > md_sngchn as ::core::ffi::c_int {
        numvoices = md_sngchn as UWORD;
    }
    if !pf.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < md_sngchn as ::core::ffi::c_int {
            (*vinfo.offset(i as isize)).i = (*(*pf).voice.offset(i as isize)).main.i;
            (*vinfo.offset(i as isize)).s = (*(*pf).voice.offset(i as isize)).main.s;
            (*vinfo.offset(i as isize)).panning = (*(*pf).voice.offset(i as isize))
                .main
                .panning;
            (*vinfo.offset(i as isize)).volume = (*(*pf).voice.offset(i as isize))
                .main
                .chanvol;
            (*vinfo.offset(i as isize)).period = (*(*pf).voice.offset(i as isize))
                .main
                .period;
            (*vinfo.offset(i as isize)).kick = (*(*pf).voice.offset(i as isize))
                .main
                .kick_flag;
            (*(*pf).voice.offset(i as isize)).main.kick_flag = 0 as UBYTE;
            i += 1;
        }
    }
    return numvoices as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetOrder() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = if !pf.is_null() {
        (*pf).sngpos as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetRow() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = if !pf.is_null() {
        (*pf).patpos as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    return ret;
}
