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

// Rust translation of src/uqm/sounds.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn ChannelPlaying(Channel: COUNT) -> BOOLEAN;
    fn GetPositionalObject(channel: COUNT) -> *mut ::core::ffi::c_void;
    fn SetPositionalObject(channel: COUNT, positional_object: *mut ::core::ffi::c_void);
    fn UpdateSoundPosition(channel: COUNT, pos: SoundPosition);
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn PlaySoundEffect(
        S: SOUND,
        Channel: COUNT,
        Pos: SoundPosition,
        PositionalObject: *mut ::core::ffi::c_void,
        Priority: BYTE,
    );
    static mut DisplayArray: [PRIMITIVE; 330];
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
pub type TIME_VALUE = UWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const NUM_PRIMS: Self = Self(7);
}
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
#[no_mangle]
pub static mut MenuSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut GameSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
static mut num_sounds: BYTE = 0 as BYTE;
static mut sound_buf: [SOUND; 8] = [::core::ptr::null_mut::<
    STRING_TABLE_ENTRY_DESC,
>(); 8];
static mut sound_posobj: [*mut ELEMENT; 8] = [::core::ptr::null_mut::<ELEMENT>(); 8];
#[no_mangle]
pub unsafe extern "C" fn PlaySound(
    mut S: SOUND,
    mut Pos: SoundPosition,
    mut PositionalObject: *mut ELEMENT,
    mut Priority: BYTE,
) {
    let mut chan: BYTE = 0;
    let mut c: BYTE = 0;
    static mut lru_channel: [BYTE; 4] = [0 as BYTE, 1 as BYTE, 2 as BYTE, 3 as BYTE];
    static mut channel: [SOUND; 4] = [
        ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    ];
    if S.is_null() {
        return;
    }
    chan = 0 as BYTE;
    while (chan as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        if S == channel[chan as usize] {
            break;
        }
        chan = chan.wrapping_add(1);
    }
    if chan as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
        chan = 0 as BYTE;
        while (chan as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            if ChannelPlaying(
                    (chan as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                )
                .0 == 0
            {
                break;
            }
            chan = chan.wrapping_add(1);
        }
        if chan as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            chan = lru_channel[0usize];
        }
    }
    channel[chan as usize] = S;
    c = 0 as BYTE;
    while (c as ::core::ffi::c_int) < 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        if lru_channel[c as usize] as ::core::ffi::c_int == chan as ::core::ffi::c_int {
            memmove(
                (&raw mut lru_channel as *mut BYTE).offset(c as isize)
                    as *mut ::core::ffi::c_void,
                (&raw mut lru_channel as *mut BYTE)
                    .offset((c as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_void,
                (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    - c as ::core::ffi::c_int) as size_t,
            );
            break;
        } else {
            c = c.wrapping_add(1);
        }
    }
    lru_channel[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = chan;
    PlaySoundEffect(
        S,
        (chan as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
        Pos,
        PositionalObject as *mut ::core::ffi::c_void,
        Priority,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PlayMenuSound(mut S: MENU_SOUND_EFFECT) {
    PlaySoundEffect(
        SetAbsStringTableIndex(MenuSounds, S.0 as COUNT),
        0 as COUNT,
        NotPositional(),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        2 as BYTE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ProcessSound(
    mut Sound: SOUND,
    mut PositionalObject: *mut ELEMENT,
) {
    if Sound
        == ::core::ptr::with_exposed_provenance_mut::<
            ::core::ffi::c_void,
        >(!(0 as ::core::ffi::c_int) as usize) as SOUND
    {
        memset(
            &raw mut sound_buf as *mut SOUND as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[SOUND; 8]>(),
        );
        memset(
            &raw mut sound_posobj as *mut *mut ELEMENT as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut ELEMENT; 8]>(),
        );
        num_sounds = 8 as BYTE;
    } else if (num_sounds as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        sound_buf[num_sounds as usize] = Sound;
        let c2rust_fresh0 = num_sounds;
        num_sounds = num_sounds.wrapping_add(1);
        sound_posobj[c2rust_fresh0 as usize] = PositionalObject;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CalcSoundPosition(
    mut ElementPtr: *mut ELEMENT,
) -> SoundPosition {
    let mut pos: SoundPosition = SoundPosition {
        positional: BOOLEAN::FALSE,
        x: 0,
        y: 0,
    };
    if ElementPtr.is_null() {
        pos.y = 0 as ::core::ffi::c_int;
        pos.x = pos.y;
        pos.positional = BOOLEAN::FALSE;
    } else {
        let mut objtype: GRAPHICS_PRIM = 0;
        objtype = DisplayArray[(*ElementPtr).PrimIndex as usize].Type;
        if objtype as ::core::ffi::c_int == gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
        {
            pos.x = DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Line.first.x
                as ::core::ffi::c_int;
            pos.y = DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Line.first.y
                as ::core::ffi::c_int;
        } else {
            pos.x = DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Point.x
                as ::core::ffi::c_int;
            pos.y = DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Point.y
                as ::core::ffi::c_int;
        }
        pos.x
            -= ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int;
        pos.y
            -= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int;
        pos.positional = BOOLEAN::TRUE;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn NotPositional() -> SoundPosition {
    return CalcSoundPosition(::core::ptr::null_mut::<ELEMENT>());
}
#[no_mangle]
pub unsafe extern "C" fn UpdateSoundPositions() {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while i as ::core::ffi::c_int
        <= 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        let mut posobj: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        if ChannelPlaying(i).0 != 0 {
            posobj = GetPositionalObject(i) as *mut ELEMENT;
            if !posobj.is_null() {
                let mut pos: SoundPosition = SoundPosition {
                    positional: BOOLEAN::FALSE,
                    x: 0,
                    y: 0,
                };
                pos = CalcSoundPosition(posobj);
                if pos.positional.0 != 0 {
                    UpdateSoundPosition(i, pos);
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FlushSounds() {
    if num_sounds as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut pSound: *mut SOUND = ::core::ptr::null_mut::<SOUND>();
        let mut pSoundPosObj: *mut *mut ELEMENT = ::core::ptr::null_mut::<
            *mut ELEMENT,
        >();
        pSound = &raw mut sound_buf as *mut SOUND;
        pSoundPosObj = &raw mut sound_posobj as *mut *mut ELEMENT;
        loop {
            let mut pos: SoundPosition = CalcSoundPosition(*pSoundPosObj);
            let c2rust_fresh1 = pSound;
            pSound = pSound.offset(1);
            let c2rust_fresh2 = pSoundPosObj;
            pSoundPosObj = pSoundPosObj.offset(1);
            PlaySound(*c2rust_fresh1, pos, *c2rust_fresh2, 2 as BYTE);
            num_sounds = num_sounds.wrapping_sub(1);
            if num_sounds == 0 {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveSoundsForObject(mut PosObj: *mut ELEMENT) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        <= 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        if GetPositionalObject(i as COUNT) == PosObj as *mut ::core::ffi::c_void {
            SetPositionalObject(
                i as COUNT,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < num_sounds as ::core::ffi::c_int {
        if sound_posobj[i as usize] == PosObj {
            sound_posobj[i as usize] = ::core::ptr::null_mut::<ELEMENT>();
        }
        i += 1;
    }
}
