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

// Rust translation of src/uqm/commanim.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn DrawStamp(pStamp: *mut STAMP);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut CommData: LOCDATA;
    fn XFormColorMap_step() -> BOOLEAN;
    fn TFB_Random() -> DWORD;
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type COORD = SWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_VALIGN(pub ::core::ffi::c_uint);
impl TEXT_VALIGN {
    pub const VALIGN_TOP: Self = Self(0);
    pub const VALIGN_MIDDLE: Self = Self(1);
    pub const VALIGN_BOTTOM: Self = Self(2);
}
pub type COLORMAP = STRING;
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIMATION_DESC {
    pub StartIndex: COUNT,
    pub NumFrames: BYTE,
    pub AnimFlags: BYTE,
    pub BaseFrameRate: COUNT,
    pub RandomFrameRate: COUNT,
    pub BaseRestartRate: COUNT,
    pub RandomRestartRate: COUNT,
    pub BlockMask: DWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ANIM_DIR(pub ::core::ffi::c_int);
impl ANIM_DIR {
    pub const DOWN_DIR: Self = Self(-1);
    pub const NO_DIR: Self = Self(0);
    pub const UP_DIR: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ANIM_TYPE(pub ::core::ffi::c_uint);
impl ANIM_TYPE {
    pub const PICTURE_ANIM: Self = Self(0);
    pub const COLOR_ANIM: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SEQUENCE {
    pub ADPtr: *mut ANIMATION_DESC,
    pub Alarm: DWORD,
    pub Direction: ANIM_DIR,
    pub CurIndex: COUNT,
    pub NextIndex: COUNT,
    pub FramesLeft: COUNT,
    pub AnimType: ANIM_TYPE,
    pub Change: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOCDATA {
    pub init_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub post_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub uninit_encounter_func: Option<unsafe extern "C" fn() -> COUNT>,
    pub AlienFrameRes: RESOURCE,
    pub AlienFontRes: RESOURCE,
    pub AlienTextFColor: Color,
    pub AlienTextBColor: Color,
    pub AlienTextBaseline: POINT,
    pub AlienTextWidth: COUNT,
    pub AlienTextAlign: TEXT_ALIGN,
    pub AlienTextValign: TEXT_VALIGN,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
    pub AlienAltSongRes: RESOURCE,
    pub AlienSongFlags: LDAS_FLAGS,
    pub ConversationPhrasesRes: RESOURCE,
    pub NumAnimations: COUNT,
    pub AlienAmbientArray: [ANIMATION_DESC; 20],
    pub AlienTransitionDesc: ANIMATION_DESC,
    pub AlienTalkDesc: ANIMATION_DESC,
    pub AlienNumberSpeech: NUMBER_SPEECH,
    pub AlienFrame: FRAME,
    pub AlienFont: FONT,
    pub AlienColorMap: COLORMAP,
    pub AlienSong: MUSIC_REF,
    pub ConversationPhrases: STRING,
}
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type NUMBER_SPEECH = *const NUMBER_SPEECH_DESC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NUMBER_SPEECH_DESC {
    pub NumDigits: COUNT,
    pub Digits: [SPEECH_DIGIT; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGIT {
    pub Divider: ::core::ffi::c_int,
    pub Subtrahend: ::core::ffi::c_int,
    pub StrDigits: *mut COUNT,
    pub Names: *mut SPEECH_DIGITNAME,
    pub CommonNameIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGITNAME {
    pub Divider: ::core::ffi::c_int,
    pub MaxRemainder: ::core::ffi::c_int,
    pub StrIndex: COUNT,
}
pub type LDAS_FLAGS = DWORD;
pub type TimeCount = DWORD;
static mut LastTime: TimeCount = 0;
static mut Sequences: [SEQUENCE; 22] = [SEQUENCE {
    ADPtr: ::core::ptr::null_mut::<ANIMATION_DESC>(),
    Alarm: 0,
    Direction: ANIM_DIR::NO_DIR,
    CurIndex: 0,
    NextIndex: 0,
    FramesLeft: 0,
    AnimType: ANIM_TYPE::PICTURE_ANIM,
    Change: BOOLEAN::FALSE,
}; 22];
static mut ActiveMask: DWORD = 0;
static mut TalkDesc: ANIMATION_DESC = ANIMATION_DESC {
    StartIndex: 0,
    NumFrames: 0,
    AnimFlags: 0,
    BaseFrameRate: 0,
    RandomFrameRate: 0,
    BaseRestartRate: 0,
    RandomRestartRate: 0,
    BlockMask: 0,
};
static mut TransitDesc: ANIMATION_DESC = ANIMATION_DESC {
    StartIndex: 0,
    NumFrames: 0,
    AnimFlags: 0,
    BaseFrameRate: 0,
    RandomFrameRate: 0,
    BaseRestartRate: 0,
    RandomRestartRate: 0,
    BlockMask: 0,
};
static mut Talk: *mut SEQUENCE = ::core::ptr::null_mut::<SEQUENCE>();
static mut Transit: *mut SEQUENCE = ::core::ptr::null_mut::<SEQUENCE>();
static mut FirstAmbient: COUNT = 0;
static mut TotalSequences: COUNT = 0;
#[inline]
unsafe extern "C" fn randomFrameRate(mut pSeq: *mut SEQUENCE) -> DWORD {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    return ((*ADPtr).BaseFrameRate as DWORD)
        .wrapping_add(
            TFB_Random()
                .wrapping_rem(
                    ((*ADPtr).RandomFrameRate as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as DWORD,
                ),
        );
}
#[inline]
unsafe extern "C" fn randomRestartRate(mut pSeq: *mut SEQUENCE) -> DWORD {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    return ((*ADPtr).BaseRestartRate as DWORD)
        .wrapping_add(
            TFB_Random()
                .wrapping_rem(
                    ((*ADPtr).RandomRestartRate as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as DWORD,
                ),
        );
}
#[inline]
unsafe extern "C" fn randomFrameIndex(
    mut pSeq: *mut SEQUENCE,
    mut from: COUNT,
) -> COUNT {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    return (from as DWORD)
        .wrapping_add(
            TFB_Random()
                .wrapping_rem(
                    ((*ADPtr).NumFrames as ::core::ffi::c_int
                        - from as ::core::ffi::c_int) as DWORD,
                ),
        ) as COUNT;
}
unsafe extern "C" fn SetupAmbientSequences(mut pSeq: *mut SEQUENCE, mut Num: COUNT) {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < Num as ::core::ffi::c_int {
        let mut ADPtr: *mut ANIMATION_DESC = (&raw mut CommData.AlienAmbientArray
            as *mut ANIMATION_DESC)
            .offset(i as isize);
        memset(
            pSeq as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SEQUENCE>(),
        );
        (*pSeq).ADPtr = ADPtr;
        if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            (*pSeq).AnimType = ANIM_TYPE::COLOR_ANIM;
        } else {
            (*pSeq).AnimType = ANIM_TYPE::PICTURE_ANIM;
        }
        (*pSeq).Direction = ANIM_DIR::UP_DIR;
        (*pSeq).FramesLeft = (*ADPtr).NumFrames as COUNT;
        if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        {
            (*pSeq).NextIndex = TFB_Random().wrapping_rem((*ADPtr).NumFrames as DWORD)
                as COUNT;
        } else if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*pSeq).NextIndex = 1 as COUNT;
            (*pSeq).FramesLeft = (*pSeq).FramesLeft.wrapping_sub(1);
        } else if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            (*pSeq).CurIndex = ((*ADPtr).NumFrames as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT;
            (*pSeq).NextIndex = 0 as COUNT;
        }
        (*pSeq).Alarm = randomRestartRate(pSeq).wrapping_add(1 as DWORD);
        i = i.wrapping_add(1);
        pSeq = pSeq.offset(1);
    }
}
unsafe extern "C" fn SetupTalkSequence(
    mut pSeq: *mut SEQUENCE,
    mut ADPtr: *mut ANIMATION_DESC,
) {
    memset(
        pSeq as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SEQUENCE>(),
    );
    (*ADPtr).AnimFlags = ((*ADPtr).AnimFlags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
    (*pSeq).ADPtr = ADPtr;
    (*pSeq).AnimType = ANIM_TYPE::PICTURE_ANIM;
}
#[inline]
unsafe extern "C" fn animAtNeutralIndex(mut pSeq: *mut SEQUENCE) -> BOOLEAN {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    if (*ADPtr).AnimFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        return BOOLEAN(
            ((*pSeq).NextIndex as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        )
    } else {
        return BOOLEAN(
            ((*pSeq).CurIndex as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        )
    };
}
#[inline]
unsafe extern "C" fn conflictsWithTalkingAnim(mut pSeq: *mut SEQUENCE) -> BOOLEAN {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    return BOOLEAN(
        ((*ADPtr).AnimFlags as ::core::ffi::c_int
            & CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn ProcessColormapAnims(mut pSeq: *mut SEQUENCE, mut Num: COUNT) {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < Num as ::core::ffi::c_int {
        let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
        if !((*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
            || (*pSeq).AnimType.0 != ANIM_TYPE::COLOR_ANIM.0 || (*pSeq).Change.0 == 0)
        {
            XFormColorMap(
                GetColorMapAddress(
                    SetAbsStringTableIndex(
                        CommData.AlienColorMap,
                        ((*ADPtr).StartIndex as ::core::ffi::c_int
                            + (*pSeq).CurIndex as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                (*pSeq).Alarm.wrapping_sub(1 as DWORD) as SIZE,
            );
            (*pSeq).Change = BOOLEAN::FALSE;
        }
        i = i.wrapping_add(1);
        pSeq = pSeq.offset(1);
    }
}
unsafe extern "C" fn AdvanceAmbientSequence(mut pSeq: *mut SEQUENCE) -> BOOLEAN {
    let mut active: BOOLEAN = BOOLEAN::FALSE;
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    (*pSeq).FramesLeft = (*pSeq).FramesLeft.wrapping_sub(1);
    if (*pSeq).FramesLeft as ::core::ffi::c_int != 0
        || (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && (*pSeq).NextIndex as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        active = BOOLEAN::TRUE;
        (*pSeq).Alarm = randomFrameRate(pSeq).wrapping_add(1 as DWORD);
    } else {
        active = BOOLEAN::FALSE;
        (*pSeq).Alarm = randomRestartRate(pSeq).wrapping_add(1 as DWORD);
        if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        {
            (*pSeq).NextIndex = 0 as COUNT;
        }
    }
    (*pSeq).CurIndex = (*pSeq).NextIndex;
    (*pSeq).Change = BOOLEAN::TRUE;
    if (*pSeq).FramesLeft as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pSeq).FramesLeft = (*ADPtr).NumFrames as COUNT;
        if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*pSeq).FramesLeft = (*pSeq).FramesLeft.wrapping_sub(1);
            (*pSeq).Direction = ANIM_DIR(-(*pSeq).Direction.0);
        } else if (*ADPtr).AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            (*pSeq).NextIndex = -1 as ::core::ffi::c_int as COUNT;
        }
    }
    if (*ADPtr).AnimFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        (*pSeq).NextIndex = randomFrameIndex(pSeq, 0 as COUNT);
    } else {
        (*pSeq).NextIndex = ((*pSeq).NextIndex as ::core::ffi::c_int
            + (*pSeq).Direction.0) as COUNT;
    }
    return active;
}
unsafe extern "C" fn ResetSequence(mut pSeq: *mut SEQUENCE) {
    (*pSeq).Direction = ANIM_DIR::NO_DIR;
    (*pSeq).CurIndex = 0 as COUNT;
    (*pSeq).Change = BOOLEAN::TRUE;
}
unsafe extern "C" fn AdvanceTalkingSequence(
    mut pSeq: *mut SEQUENCE,
    mut ElapsedTicks: DWORD,
) {
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    if (*pSeq).Direction.0 == ANIM_DIR::NO_DIR.0 {
        (*pSeq).Direction = ANIM_DIR::UP_DIR;
        if CommData.AlienTalkDesc.StartIndex as ::core::ffi::c_int
            != (*ADPtr).StartIndex as ::core::ffi::c_int
        {
            *ADPtr = CommData.AlienTalkDesc;
        }
        ((*pSeq).CurIndex as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"pSeq->CurIndex == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/commanim.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    249 as ::core::ffi::c_int,
                    b"AdvanceTalkingSequence\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        (*pSeq).Alarm = 0 as DWORD;
        (*ADPtr).AnimFlags = ((*ADPtr).AnimFlags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as BYTE;
    }
    if (*pSeq).Alarm > ElapsedTicks {
        (*pSeq).Alarm = (*pSeq).Alarm.wrapping_sub(ElapsedTicks);
        return;
    }
    (*pSeq).Alarm = randomFrameRate(pSeq);
    (*pSeq).Change = BOOLEAN::TRUE;
    if (*pSeq).CurIndex as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pSeq).CurIndex = randomFrameIndex(pSeq, 1 as COUNT);
        (*pSeq).Alarm = (*pSeq).Alarm.wrapping_add(randomRestartRate(pSeq));
    } else {
        (*pSeq).CurIndex = 0 as COUNT;
    };
}
unsafe extern "C" fn AdvanceTransitSequence(
    mut pSeq: *mut SEQUENCE,
    mut ElapsedTicks: DWORD,
) -> BOOLEAN {
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
    if (*pSeq).Direction.0 == ANIM_DIR::NO_DIR.0 {
        (*pSeq).Alarm = 0 as DWORD;
        (*ADPtr).AnimFlags = ((*ADPtr).AnimFlags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as BYTE;
    }
    if (*pSeq).Alarm > ElapsedTicks {
        (*pSeq).Alarm = (*pSeq).Alarm.wrapping_sub(ElapsedTicks);
        return BOOLEAN::FALSE;
    }
    (*pSeq).Change = BOOLEAN::TRUE;
    if (*pSeq).Direction.0 == ANIM_DIR::NO_DIR.0 {
        (*pSeq).FramesLeft = (*ADPtr).NumFrames as COUNT;
        if CommData.AlienTransitionDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
        {
            (*pSeq).Direction = ANIM_DIR::DOWN_DIR;
            (*pSeq).CurIndex = ((*ADPtr).NumFrames as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT;
        } else if CommData.AlienTransitionDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        {
            (*pSeq).Direction = ANIM_DIR::UP_DIR;
            if CommData.AlienTransitionDesc.StartIndex as ::core::ffi::c_int
                != (*ADPtr).StartIndex as ::core::ffi::c_int
            {
                *ADPtr = CommData.AlienTransitionDesc;
            }
            (*pSeq).CurIndex = 0 as COUNT;
        }
    }
    (*pSeq).FramesLeft = (*pSeq).FramesLeft.wrapping_sub(1);
    if (*pSeq).FramesLeft as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*pSeq).Direction.0 == ANIM_DIR::UP_DIR.0 {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as BYTE;
        } else if (*pSeq).Direction.0 == ANIM_DIR::DOWN_DIR.0 {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as BYTE;
            (*ADPtr).AnimFlags = ((*ADPtr).AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
            done = BOOLEAN::TRUE;
        }
        (*pSeq).Direction = ANIM_DIR::NO_DIR;
    } else {
        (*pSeq).Alarm = randomFrameRate(pSeq);
        (*pSeq).CurIndex = ((*pSeq).CurIndex as ::core::ffi::c_int + (*pSeq).Direction.0)
            as COUNT;
    }
    return done;
}
#[no_mangle]
pub unsafe extern "C" fn InitCommAnimations() {
    ActiveMask = 0 as DWORD;
    TalkDesc = CommData.AlienTalkDesc;
    TransitDesc = CommData.AlienTransitionDesc;
    TotalSequences = 0 as COUNT;
    Transit = (&raw mut Sequences as *mut SEQUENCE)
        .offset(TotalSequences as ::core::ffi::c_int as isize);
    SetupTalkSequence(Transit, &raw mut TransitDesc);
    TotalSequences = TotalSequences.wrapping_add(1);
    Talk = (&raw mut Sequences as *mut SEQUENCE)
        .offset(TotalSequences as ::core::ffi::c_int as isize);
    SetupTalkSequence(Talk, &raw mut TalkDesc);
    TotalSequences = TotalSequences.wrapping_add(1);
    FirstAmbient = TotalSequences;
    SetupAmbientSequences(
        (&raw mut Sequences as *mut SEQUENCE)
            .offset(FirstAmbient as ::core::ffi::c_int as isize),
        CommData.NumAnimations,
    );
    TotalSequences = (TotalSequences as ::core::ffi::c_int
        + CommData.NumAnimations as ::core::ffi::c_int) as COUNT;
    LastTime = GetTimeCounter();
}
#[no_mangle]
pub unsafe extern "C" fn ProcessCommAnimations(
    mut FullRedraw: BOOLEAN,
    mut paused: BOOLEAN,
) -> BOOLEAN {
    if paused.0 != 0 {
        XFormColorMap_step();
        return BOOLEAN::FALSE;
    } else {
        let mut i: COUNT = 0;
        let mut pSeq: *mut SEQUENCE = ::core::ptr::null_mut::<SEQUENCE>();
        let mut Change: BOOLEAN = BOOLEAN::FALSE;
        let mut CanTalk: BOOLEAN = BOOLEAN::TRUE;
        let mut CurTime: TimeCount = 0;
        let mut ElapsedTicks: DWORD = 0;
        let mut NextActiveMask: DWORD = 0;
        CurTime = GetTimeCounter();
        ElapsedTicks = CurTime.wrapping_sub(LastTime) as DWORD;
        LastTime = CurTime;
        NextActiveMask = ActiveMask;
        pSeq = (&raw mut Sequences as *mut SEQUENCE)
            .offset(FirstAmbient as ::core::ffi::c_int as isize);
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < CommData.NumAnimations as ::core::ffi::c_int {
            let mut ADPtr: *mut ANIMATION_DESC = (*pSeq).ADPtr;
            let mut ActiveBit: DWORD = ((1 as ::core::ffi::c_long)
                << i as ::core::ffi::c_int) as DWORD;
            's_45: {
                if (*ADPtr).AnimFlags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                {
                    if (*pSeq).Direction.0 == ANIM_DIR::NO_DIR.0 {
                        if conflictsWithTalkingAnim(pSeq).0 == 0 {
                            (*pSeq).Direction = ANIM_DIR::UP_DIR;
                        }
                    } else if (*pSeq).Alarm > ElapsedTicks {
                        (*pSeq).Alarm = (*pSeq).Alarm.wrapping_sub(ElapsedTicks);
                    } else if ActiveMask & (*ADPtr).BlockMask != 0 {
                        (ActiveMask & ActiveBit == 0
                            && !(b"Check animations' mutual blocking masks\0".as_ptr()
                                as *const ::core::ffi::c_char)
                                .is_null()
                            || {
                                __assert_fail(
                                    b"!(ActiveMask & ActiveBit) && \"Check animations' mutual blocking masks\"\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"src/uqm/commanim.c\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    427 as ::core::ffi::c_int,
                                    b"ProcessCommAnimations\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                0 as ::core::ffi::c_int != 0
                            }) as ::core::ffi::c_int;
                        (animAtNeutralIndex(pSeq).0 != 0
                            || {
                                __assert_fail(
                                    b"animAtNeutralIndex (pSeq)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"src/uqm/commanim.c\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    428 as ::core::ffi::c_int,
                                    b"ProcessCommAnimations\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                0 as ::core::ffi::c_int != 0
                            }) as ::core::ffi::c_int;
                        (*pSeq).Alarm = randomRestartRate(pSeq).wrapping_add(1 as DWORD);
                        break 's_45;
                    } else if AdvanceAmbientSequence(pSeq).0 != 0 {
                        ActiveMask |= ActiveBit;
                        NextActiveMask |= ActiveBit;
                    } else {
                        NextActiveMask &= !ActiveBit;
                    }
                    if (*pSeq).AnimType.0 == ANIM_TYPE::PICTURE_ANIM.0
                        && (*pSeq).Direction.0 != ANIM_DIR::NO_DIR.0
                        && conflictsWithTalkingAnim(pSeq).0 != 0
                    {
                        if animAtNeutralIndex(pSeq).0 != 0 {
                            (*pSeq).Direction = ANIM_DIR::NO_DIR;
                            NextActiveMask &= !ActiveBit;
                        } else {
                            CanTalk = BOOLEAN::FALSE;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            pSeq = pSeq.offset(1);
        }
        ActiveMask = NextActiveMask;
        if CanTalk.0 != 0 && haveTalkingAnim().0 != 0 && runningTalkingAnim().0 != 0 {
            let mut done: BOOLEAN = BOOLEAN::FALSE;
            if signaledStopTalkingAnim().0 != 0 && haveTransitionAnim().0 != 0 {
                CommData.AlienTransitionDesc.AnimFlags = (CommData
                    .AlienTransitionDesc
                    .AnimFlags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
            }
            if CommData.AlienTransitionDesc.AnimFlags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
            {
                if CommData.AlienTransitionDesc.AnimFlags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                    && (*Transit).Direction.0 == ANIM_DIR::NO_DIR.0
                {
                    ResetSequence(Talk);
                }
                done = AdvanceTransitSequence(Transit, ElapsedTicks);
            } else if signaledStopTalkingAnim().0 == 0 {
                AdvanceTalkingSequence(Talk, ElapsedTicks);
            } else {
                ResetSequence(Talk);
                done = BOOLEAN::TRUE;
            }
            if signaledStopTalkingAnim().0 != 0 && done.0 != 0 {
                clearRunTalkingAnim();
                clearStopTalkingAnim();
            }
        } else if (*Talk).Direction.0 == ANIM_DIR::NO_DIR.0 {
            TalkDesc.AnimFlags = (TalkDesc.AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
        }
        BatchGraphics();
        let mut ColorChange: BOOLEAN = XFormColorMap_step();
        if ColorChange.0 != 0 {
            FullRedraw = BOOLEAN::TRUE;
        }
        ProcessColormapAnims(
            (&raw mut Sequences as *mut SEQUENCE)
                .offset(FirstAmbient as ::core::ffi::c_int as isize),
            CommData.NumAnimations,
        );
        Change = DrawAlienFrame(
            &raw mut Sequences as *mut SEQUENCE,
            TotalSequences,
            FullRedraw,
        );
        if FullRedraw.0 != 0 {
            Change = BOOLEAN::TRUE;
        }
        UnbatchGraphics();
        pSeq = (&raw mut Sequences as *mut SEQUENCE)
            .offset(FirstAmbient as ::core::ffi::c_int as isize);
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < CommData.NumAnimations as ::core::ffi::c_int {
            let mut ADPtr_0: *mut ANIMATION_DESC = (*pSeq).ADPtr;
            let mut ActiveBit_0: DWORD = ((1 as ::core::ffi::c_long)
                << i as ::core::ffi::c_int) as DWORD;
            if (*ADPtr_0).AnimFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
            {
                if (*ADPtr_0).AnimFlags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
                    && NextActiveMask & ActiveBit_0 == 0
                {
                    (*ADPtr_0).AnimFlags = ((*ADPtr_0).AnimFlags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
                }
            }
            i = i.wrapping_add(1);
            pSeq = pSeq.offset(1);
        }
        return Change;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawAlienFrame(
    mut Sequences_0: *mut SEQUENCE,
    mut Num: COUNT,
    mut fullRedraw: BOOLEAN,
) -> BOOLEAN {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Change: BOOLEAN = BOOLEAN::FALSE;
    BatchGraphics();
    s.origin.x = -0 as COORD;
    s.origin.y = 0 as COORD;
    if fullRedraw.0 != 0 {
        s.frame = CommData.AlienFrame;
        DrawStamp(&raw mut s);
        i = CommData.NumAnimations as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut ADPtr: *mut ANIMATION_DESC = (&raw mut CommData.AlienAmbientArray
                as *mut ANIMATION_DESC)
                .offset(i as isize);
            if (*ADPtr).AnimFlags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) == 0
            {
                (*ADPtr).AnimFlags = ((*ADPtr).AnimFlags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
                if (*ADPtr).AnimFlags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
                {
                    s.frame = SetAbsFrameIndex(CommData.AlienFrame, (*ADPtr).StartIndex);
                    DrawStamp(&raw mut s);
                }
            }
            i -= 1;
        }
    }
    if !Sequences_0.is_null() {
        i = Num as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut pSeq: *mut SEQUENCE = Sequences_0.offset(i as isize);
            let mut ADPtr_0: *mut ANIMATION_DESC = (*pSeq).ADPtr;
            if !((*ADPtr_0).AnimFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                || (*pSeq).AnimType.0 != ANIM_TYPE::PICTURE_ANIM.0)
            {
                if !(fullRedraw.0 == 0 && (*pSeq).Change.0 == 0) {
                    s.frame = SetAbsFrameIndex(
                        CommData.AlienFrame,
                        ((*ADPtr_0).StartIndex as ::core::ffi::c_int
                            + (*pSeq).CurIndex as ::core::ffi::c_int) as COUNT,
                    );
                    DrawStamp(&raw mut s);
                    (*pSeq).Change = BOOLEAN::FALSE;
                    Change = BOOLEAN::TRUE;
                }
            }
            i -= 1;
        }
    }
    UnbatchGraphics();
    return Change;
}
#[inline]
unsafe extern "C" fn haveTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.NumFrames as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn haveTransitionAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTransitionDesc.NumFrames as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn clearRunTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
        as BYTE;
}
#[inline]
unsafe extern "C" fn runningTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn clearStopTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
        as BYTE;
}
#[inline]
unsafe extern "C" fn signaledStopTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
