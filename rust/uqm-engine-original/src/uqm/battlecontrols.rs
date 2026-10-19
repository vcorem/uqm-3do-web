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

// Rust translation of src/uqm/battlecontrols.c.
extern "C" {
    pub type getmelee_struct;
    pub type frame_desc;
    pub type string_table_entry;
    pub type tfb_soundsample;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn computer_intelligence(
        context: *mut ComputerInputContext,
        StarShipPtr: *mut STARSHIP,
    ) -> BATTLE_INPUT_STATE;
    fn battleEndReadyHuman(context: *mut HumanInputContext) -> bool;
    fn battleEndReadyComputer(context: *mut ComputerInputContext) -> bool;
    fn battleEndReadyNetwork(context: *mut NetworkInputContext) -> bool;
    fn networkBattleInput(
        context: *mut NetworkInputContext,
        StarShipPtr: *mut STARSHIP,
    ) -> BATTLE_INPUT_STATE;
    fn selectShipHuman(
        context: *mut HumanInputContext,
        gms: *mut GETMELEE_STATE,
    ) -> BOOLEAN;
    fn selectShipComputer(
        context: *mut ComputerInputContext,
        gms: *mut GETMELEE_STATE,
    ) -> BOOLEAN;
    fn selectShipNetwork(
        context: *mut NetworkInputContext,
        gms: *mut GETMELEE_STATE,
    ) -> BOOLEAN;
    fn frameInputHuman(
        context: *mut HumanInputContext,
        StarShipPtr: *mut STARSHIP,
    ) -> BATTLE_INPUT_STATE;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputHandlers {
    pub frameInput: BattleFrameInputFunction,
    pub selectShip: SelectShipFunction,
    pub battleEndReady: BattleEndReadyFunction,
    pub deleteContext: DeleteInputContextFunction,
}
pub type DeleteInputContextFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type BattleEndReadyFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> bool,
>;
pub type SelectShipFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut GETMELEE_STATE) -> BOOLEAN,
>;
pub type GETMELEE_STATE = getmelee_struct;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type BattleFrameInputFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut STARSHIP) -> BATTLE_INPUT_STATE,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STARSHIP {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub RaceDescPtr: *mut RACE_DESC,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub ship_cost: BYTE,
    pub index: COUNT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub weapon_counter: BYTE,
    pub special_counter: BYTE,
    pub energy_counter: BYTE,
    pub ship_input_state: BYTE,
    pub cur_status_flags: STATUS_FLAGS,
    pub old_status_flags: STATUS_FLAGS,
    pub hShip: HELEMENT,
    pub ShipFacing: COUNT,
    pub playerNr: SIZE,
    pub control: BYTE,
}
pub type BYTE = uint8;
pub type uint8 = ::core::ffi::c_uchar;
pub type SIZE = SWORD;
pub type SWORD = sint16;
pub type sint16 = ::core::ffi::c_short;
pub type HELEMENT = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type STATUS_FLAGS = UWORD;
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 12],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
pub type ELEMENT = element;
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
pub type STATE = state;
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
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type TIME_VALUE = UWORD;
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
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
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
pub type ELEMENT_FLAGS = UWORD;
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type SOUND = STRING;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type RESOURCE = *const ::core::ffi::c_char;
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SPECIES_ID(pub ::core::ffi::c_uint);
impl SPECIES_ID {
    pub const NO_ID: Self = Self(0);
    pub const ARILOU_ID: Self = Self(1);
    pub const CHMMR_ID: Self = Self(2);
    pub const EARTHLING_ID: Self = Self(3);
    pub const ORZ_ID: Self = Self(4);
    pub const PKUNK_ID: Self = Self(5);
    pub const SHOFIXTI_ID: Self = Self(6);
    pub const SPATHI_ID: Self = Self(7);
    pub const SUPOX_ID: Self = Self(8);
    pub const THRADDASH_ID: Self = Self(9);
    pub const UTWIG_ID: Self = Self(10);
    pub const VUX_ID: Self = Self(11);
    pub const YEHAT_ID: Self = Self(12);
    pub const MELNORME_ID: Self = Self(13);
    pub const DRUUGE_ID: Self = Self(14);
    pub const ILWRATH_ID: Self = Self(15);
    pub const MYCON_ID: Self = Self(16);
    pub const SLYLANDRO_ID: Self = Self(17);
    pub const UMGAH_ID: Self = Self(18);
    pub const UR_QUAN_ID: Self = Self(19);
    pub const ZOQFOTPIK_ID: Self = Self(20);
    pub const SYREEN_ID: Self = Self(21);
    pub const KOHR_AH_ID: Self = Self(22);
    pub const ANDROSYNTH_ID: Self = Self(23);
    pub const CHENJESU_ID: Self = Self(24);
    pub const MMRNMHRM_ID: Self = Self(25);
    pub const LAST_MELEE_ID: Self = Self(25);
    pub const SIS_SHIP_ID: Self = Self(26);
    pub const SA_MATRA_ID: Self = Self(27);
    pub const UR_QUAN_PROBE_ID: Self = Self(28);
    pub const NUM_SPECIES_ID: Self = Self(29);
}
pub type BATTLE_INPUT_STATE = UBYTE;
pub type UBYTE = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComputerInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HumanInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetworkInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
pub type size_t = usize;
#[no_mangle]
pub static mut PlayerInput: [*mut InputContext; 2] = [::core::ptr::null_mut::<
    InputContext,
>(); 2];
#[no_mangle]
pub static mut ComputerInputHandlers: BattleInputHandlers = unsafe {
    BattleInputHandlers {
        frameInput: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ComputerInputContext,
                    *mut STARSHIP,
                ) -> BATTLE_INPUT_STATE,
            >,
            BattleFrameInputFunction,
        >(
            Some(
                computer_intelligence
                    as unsafe extern "C" fn(
                        *mut ComputerInputContext,
                        *mut STARSHIP,
                    ) -> BATTLE_INPUT_STATE,
            ),
        ),
        selectShip: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ComputerInputContext,
                    *mut GETMELEE_STATE,
                ) -> BOOLEAN,
            >,
            SelectShipFunction,
        >(
            Some(
                selectShipComputer
                    as unsafe extern "C" fn(
                        *mut ComputerInputContext,
                        *mut GETMELEE_STATE,
                    ) -> BOOLEAN,
            ),
        ),
        battleEndReady: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ComputerInputContext) -> bool>,
            BattleEndReadyFunction,
        >(
            Some(
                battleEndReadyComputer
                    as unsafe extern "C" fn(*mut ComputerInputContext) -> bool,
            ),
        ),
        deleteContext: Some(
            InputContext_delete as unsafe extern "C" fn(*mut InputContext) -> (),
        ),
    }
};
#[no_mangle]
pub static mut HumanInputHandlers: BattleInputHandlers = unsafe {
    BattleInputHandlers {
        frameInput: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut HumanInputContext,
                    *mut STARSHIP,
                ) -> BATTLE_INPUT_STATE,
            >,
            BattleFrameInputFunction,
        >(
            Some(
                frameInputHuman
                    as unsafe extern "C" fn(
                        *mut HumanInputContext,
                        *mut STARSHIP,
                    ) -> BATTLE_INPUT_STATE,
            ),
        ),
        selectShip: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut HumanInputContext,
                    *mut GETMELEE_STATE,
                ) -> BOOLEAN,
            >,
            SelectShipFunction,
        >(
            Some(
                selectShipHuman
                    as unsafe extern "C" fn(
                        *mut HumanInputContext,
                        *mut GETMELEE_STATE,
                    ) -> BOOLEAN,
            ),
        ),
        battleEndReady: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut HumanInputContext) -> bool>,
            BattleEndReadyFunction,
        >(
            Some(
                battleEndReadyHuman
                    as unsafe extern "C" fn(*mut HumanInputContext) -> bool,
            ),
        ),
        deleteContext: Some(
            InputContext_delete as unsafe extern "C" fn(*mut InputContext) -> (),
        ),
    }
};
#[no_mangle]
pub static mut NetworkInputHandlers: BattleInputHandlers = unsafe {
    BattleInputHandlers {
        frameInput: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut NetworkInputContext,
                    *mut STARSHIP,
                ) -> BATTLE_INPUT_STATE,
            >,
            BattleFrameInputFunction,
        >(
            Some(
                networkBattleInput
                    as unsafe extern "C" fn(
                        *mut NetworkInputContext,
                        *mut STARSHIP,
                    ) -> BATTLE_INPUT_STATE,
            ),
        ),
        selectShip: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut NetworkInputContext,
                    *mut GETMELEE_STATE,
                ) -> BOOLEAN,
            >,
            SelectShipFunction,
        >(
            Some(
                selectShipNetwork
                    as unsafe extern "C" fn(
                        *mut NetworkInputContext,
                        *mut GETMELEE_STATE,
                    ) -> BOOLEAN,
            ),
        ),
        battleEndReady: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut NetworkInputContext) -> bool>,
            BattleEndReadyFunction,
        >(
            Some(
                battleEndReadyNetwork
                    as unsafe extern "C" fn(*mut NetworkInputContext) -> bool,
            ),
        ),
        deleteContext: Some(
            InputContext_delete as unsafe extern "C" fn(*mut InputContext) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn InputContext_init(
    mut context: *mut InputContext,
    mut handlers: *mut BattleInputHandlers,
    mut playerNr: COUNT,
) {
    (*context).handlers = handlers;
    (*context).playerNr = playerNr;
}
#[no_mangle]
pub unsafe extern "C" fn InputContext_delete(mut context: *mut InputContext) {
    HFree(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ComputerInputContext_new(
    mut playerNr: COUNT,
) -> *mut ComputerInputContext {
    let mut result: *mut ComputerInputContext = HMalloc(
        ::core::mem::size_of::<ComputerInputContext>(),
    ) as *mut ComputerInputContext;
    InputContext_init(
        result as *mut InputContext,
        &raw mut ComputerInputHandlers,
        playerNr,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn HumanInputContext_new(
    mut playerNr: COUNT,
) -> *mut HumanInputContext {
    let mut result: *mut HumanInputContext = HMalloc(
        ::core::mem::size_of::<HumanInputContext>(),
    ) as *mut HumanInputContext;
    InputContext_init(
        result as *mut InputContext,
        &raw mut HumanInputHandlers,
        playerNr,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn NetworkInputContext_new(
    mut playerNr: COUNT,
) -> *mut NetworkInputContext {
    let mut result: *mut NetworkInputContext = HMalloc(
        ::core::mem::size_of::<NetworkInputContext>(),
    ) as *mut NetworkInputContext;
    InputContext_init(
        result as *mut InputContext,
        &raw mut NetworkInputHandlers,
        playerNr,
    );
    return result;
}
