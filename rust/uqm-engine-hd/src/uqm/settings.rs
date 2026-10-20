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

// Rust translation of src/uqm/settings.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn PLRPlaySong(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn PLRStop(MusicRef: MUSIC_REF);
    fn PLRSeek(MusicRef: MUSIC_REF, pos: DWORD);
    fn PLRPause(MusicRef: MUSIC_REF);
    fn PLRResume(MusicRef: MUSIC_REF);
    fn PlayChannel(
        channel: COUNT,
        snd: SOUND,
        pos: SoundPosition,
        positional_object: *mut ::core::ffi::c_void,
        priority: ::core::ffi::c_uchar,
    );
    fn SetMusicVolume(Volume: COUNT);
    fn SetChannelVolume(Channel: COUNT, Volume: COUNT, Priority: BYTE);
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn OkayToResume() -> BOOLEAN;
    fn GetMusicPosition() -> DWORD;
    static mut GlobData: GLOBDATA;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
pub type GRAPHICS_PRIM = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
    pub flags: BYTE,
}
pub type SOUND = STRING;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE {
    pub glob_flags: BYTE,
    pub CrewCost: BYTE,
    pub FuelCost: BYTE,
    pub ModuleCost: [BYTE; 20],
    pub ElementWorth: [BYTE; 8],
    pub DisplayArray: *mut PRIMITIVE,
    pub CurrentActivity: ACTIVITY,
    pub GameClock: CLOCK_STATE,
    pub autopilot: POINT,
    pub ip_location: POINT,
    pub ShipStamp: STAMP,
    pub ShipFacing: UWORD,
    pub ip_planet: BYTE,
    pub in_orbit: BYTE,
    pub velocity: VELOCITY_DESC,
    pub BattleGroupRef: DWORD,
    pub avail_race_q: QUEUE,
    pub npc_built_ship_q: QUEUE,
    pub ip_group_q: QUEUE,
    pub encounter_q: QUEUE,
    pub built_ship_q: QUEUE,
    pub stowed_ship_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIS_STATE {
    pub log_x: SDWORD,
    pub log_y: SDWORD,
    pub ResUnits: DWORD,
    pub FuelOnBoard: DWORD,
    pub CrewEnlisted: COUNT,
    pub TotalElementMass: COUNT,
    pub TotalBioMass: COUNT,
    pub ModuleSlots: [BYTE; 16],
    pub DriveSlots: [BYTE; 11],
    pub JetSlots: [BYTE; 8],
    pub NumLanders: BYTE,
    pub ElementAmounts: [COUNT; 8],
    pub ShipName: [UNICODE; 32],
    pub CommanderName: [UNICODE; 32],
    pub PlanetName: [UNICODE; 32],
    pub Difficulty: BYTE,
    pub Extended: BYTE,
    pub Nomad: BYTE,
    pub ShipSeed: BYTE,
    pub Seed: SDWORD,
    pub SaveVersion: BYTE,
}
static mut LastMusicRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
static mut LastContinuous: BOOLEAN = BOOLEAN::FALSE;
static mut LastPriority: BYTE = 0;
#[no_mangle]
pub unsafe extern "C" fn ToggleMusic() {
    GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
        as ::core::ffi::c_int ^ (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        as BYTE;
    if LastPriority as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
        if GlobData.Game_state.glob_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(PLRStop(LastMusicRef), ());
        } else if !LastMusicRef.is_null() {
            uqm_lua_checked!(PLRPlaySong(LastMusicRef, LastContinuous, LastPriority), ());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PlayMusic(
    mut MusicRef: MUSIC_REF,
    mut Continuous: BOOLEAN,
    mut Priority: BYTE,
) {
    LastMusicRef = MusicRef;
    LastContinuous = Continuous;
    LastPriority = Priority;
    if GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(PLRPlaySong(MusicRef, Continuous, Priority), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeekMusic(mut pos: DWORD) {
    uqm_lua_checked!(PLRSeek(LastMusicRef, pos), ());
}
#[no_mangle]
pub unsafe extern "C" fn StopMusic() {
    uqm_lua_checked!(PLRStop(LastMusicRef), ());
    LastMusicRef = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
}
#[no_mangle]
pub unsafe extern "C" fn ResumeMusic() {
    uqm_lua_checked!(PLRResume(LastMusicRef), ());
}
#[no_mangle]
pub unsafe extern "C" fn PauseMusic() {
    uqm_lua_checked!(PLRPause(LastMusicRef), ());
}
#[no_mangle]
pub unsafe extern "C" fn PlayMusicResume(mut MusicRef: MUSIC_REF, mut Volume: BYTE) {
    uqm_lua_checked!(SetMusicVolume(0 as COUNT), ());
    uqm_lua_checked!(PlayMusic(MusicRef, BOOLEAN::TRUE, 1 as BYTE), ());
    if OkayToResume().0 != 0 {
        uqm_lua_checked!(SeekMusic(GetMusicPosition()), ());
        uqm_lua_checked!(FadeMusic(Volume, (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE), ());
    } else {
        uqm_lua_checked!(SetMusicVolume(Volume as COUNT), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn ToggleSoundEffect() {
    GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
        as ::core::ffi::c_int ^ (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
        as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn PlaySoundEffect(
    mut S: SOUND,
    mut Channel: COUNT,
    mut Pos: SoundPosition,
    mut PositionalObject: *mut ::core::ffi::c_void,
    mut Priority: BYTE,
) {
    if GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(SetChannelVolume(
            Channel,
            (255 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
            Priority,
        ), ());
        uqm_lua_checked!(PlayChannel(Channel, S, Pos, PositionalObject, Priority), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn PlaySpeechEffect(
    mut S: SOUND,
    mut Pos: SoundPosition,
    mut PositionalObject: *mut ::core::ffi::c_void,
    mut Priority: BYTE,
) {
    if GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(PlayChannel(6 as COUNT, S, Pos, PositionalObject, Priority), ());
    }
}
