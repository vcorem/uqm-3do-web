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

// Rust translation of src/uqm/cleanup.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn UninitResourceSystem();
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    static mut GameStrings: STRING;
    fn UninitSpace();
    fn UninitQueue(pq: *mut QUEUE) -> BOOLEAN;
    static mut disp_q: QUEUE;
    fn FreeHyperData();
    fn UninitSound();
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn FreeLanderData();
    static mut Screen: FRAME;
    static mut ActivityFrame: FRAME;
    static mut StatusFrame: FRAME;
    static mut BorderFrame: FRAME;
    static mut HDBorderFrame: FRAME;
    static mut CustBevelFrame: FRAME;
    static mut DefBevelFrame: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut PlyrFont: FONT;
    static mut LabelFont: FONT;
    static mut SlabFont: FONT;
    static mut SquareFont: FONT;
    static mut PlayMenuFont: FONT;
    static mut race_q: [QUEUE; 0];
    fn UninitPlayerInput();
    fn FreeIPData();
    static mut MenuSounds: SOUND;
    fn UninitVideoPlayer();
    fn FreeSC2Data();
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
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
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
#[no_mangle]
pub unsafe extern "C" fn FreeKernel() {
    UninitPlayerInput();
    UninitResourceSystem();
    DestroyDrawable(ReleaseDrawable(Screen));
    Screen = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyContext(ScreenContext);
    ScreenContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    uqm_lua_checked!(UninitVideoPlayer(), ());
    UninitSound();
}
unsafe extern "C" fn UninitContexts() {
    UninitQueue(&raw mut disp_q);
    DestroyContext(OffScreenContext);
    DestroyContext(SpaceContext);
    DestroyContext(StatusContext);
}
unsafe extern "C" fn UninitKernel() {
    UninitSpace();
    uqm_lua_checked!(DestroySound(ReleaseStringTable(MenuSounds)), ());
    DestroyFont(MicroFont);
    DestroyStringTable(ReleaseStringTable(GameStrings));
    DestroyDrawable(ReleaseDrawable(BorderFrame));
    DestroyDrawable(ReleaseDrawable(HDBorderFrame));
    DestroyDrawable(ReleaseDrawable(CustBevelFrame));
    DestroyDrawable(ReleaseDrawable(DefBevelFrame));
    DestroyDrawable(ReleaseDrawable(StatusFrame));
    DestroyDrawable(ReleaseDrawable(ActivityFrame));
    DestroyFont(TinyFont);
    DestroyFont(TinyFontBold);
    DestroyFont(StarConFont);
    DestroyFont(PlyrFont);
    DestroyFont(LabelFont);
    DestroyFont(SlabFont);
    DestroyFont(SquareFont);
    DestroyFont(PlayMenuFont);
    UninitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
    UninitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
    ActivityFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn FreeGameData() {
    FreeSC2Data();
    uqm_lua_checked!(FreeLanderData(), ());
    uqm_lua_checked!(FreeIPData(), ());
    FreeHyperData();
}
#[no_mangle]
pub unsafe extern "C" fn UninitGameKernel() {
    if !ActivityFrame.is_null() {
        uqm_lua_checked!(FreeGameData(), ());
        uqm_lua_checked!(UninitKernel(), ());
        UninitContexts();
    }
}
