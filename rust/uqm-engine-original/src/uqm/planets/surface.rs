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

// Rust translation of src/uqm/planets/surface.c.
extern "C" {
    pub type RandomContext;
    pub type frame_desc;
    pub type font_desc;
    pub type string_table_entry;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut SysGenRNG: *mut RandomContext;
    static mut ScreenWidth: ::core::ffi::c_int;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
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
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: UWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanetFrame {
    pub Type: BYTE,
    pub BaseTectonics: BYTE,
    pub AtmoAndDensity: BYTE,
    pub UsefulElements: [ELEMENT_ENTRY; 8],
    pub CMapInstance: RESOURCE,
    pub XlatTabInstance: RESOURCE,
    pub num_faults: COUNT,
    pub fault_depth: SIZE,
    pub num_blemishes: COUNT,
    pub base_elevation: SIZE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[no_mangle]
pub static mut Elements: *const BYTE = ::core::ptr::null::<BYTE>();
#[no_mangle]
pub static mut PlanData: *const PlanetFrame = ::core::ptr::null::<PlanetFrame>();
unsafe extern "C" fn CalcMineralDeposits(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut which_deposit: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut j: BYTE = 0;
    let mut num_deposits: COUNT = 0;
    let mut eptr: *const ELEMENT_ENTRY = ::core::ptr::null::<ELEMENT_ENTRY>();
    eptr = (&raw const (*(*SysInfoPtr).PlanetInfo.PlanDataPtr).UsefulElements
        as *const ELEMENT_ENTRY)
        .offset(0isize);
    num_deposits = 0 as COUNT;
    j = 8 as BYTE;
    loop {
        let mut num_possible: BYTE = 0;
        num_possible = (RandomContext_Random(SysGenRNG) as UWORD as BYTE
            as ::core::ffi::c_int
            % (((*eptr).Density as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as BYTE;
        loop {
            let c2rust_fresh0 = num_possible;
            num_possible = num_possible.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let mut deposit_quality_fine: COUNT = 0;
            let mut deposit_quality_gross: COUNT = 0;
            deposit_quality_fine = (RandomContext_Random(SysGenRNG) as UWORD
                as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                + (((*eptr).Density as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int + (*SysInfoPtr).StarSize as ::core::ffi::c_int)
                    * 50 as ::core::ffi::c_int) as COUNT;
            if (deposit_quality_fine as ::core::ffi::c_int) < 150 as ::core::ffi::c_int {
                deposit_quality_gross = 0 as COUNT;
            } else if (deposit_quality_fine as ::core::ffi::c_int)
                < 225 as ::core::ffi::c_int
            {
                deposit_quality_gross = 1 as COUNT;
            } else {
                deposit_quality_gross = 2 as COUNT;
            }
            GenerateRandomLocation(&raw mut (*info).loc_pt);
            (*info).density = ((((deposit_quality_fine as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | deposit_quality_gross as BYTE as ::core::ffi::c_int) as COUNT;
            (*info).r#type = (*eptr).ElementType as COUNT;
            if num_deposits as ::core::ffi::c_int >= which_deposit as ::core::ffi::c_int
                || {
                    num_deposits = num_deposits.wrapping_add(1);
                    num_deposits as usize
                        == ::core::mem::size_of::<DWORD>().wrapping_mul(8usize)
                }
            {
                return num_deposits;
            }
        }
        eptr = eptr.offset(1);
        j = j.wrapping_sub(1);
        if j == 0 {
            break;
        }
    }
    return num_deposits;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateMineralDeposits(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut whichDeposit: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut temp_info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    if info.is_null() {
        info = &raw mut temp_info;
    }
    RandomContext_SeedRandom(
        SysGenRNG,
        (*SysInfoPtr)
            .PlanetInfo
            .ScanSeed[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as usize],
    );
    return CalcMineralDeposits(SysInfoPtr, whichDeposit, info);
}
unsafe extern "C" fn CalcLifeForms(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut which_life: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut num_life_forms: COUNT = 0;
    num_life_forms = 0 as COUNT;
    if ((*(*SysInfoPtr).PlanetInfo.PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed::GAS_GIANT.0 as ::core::ffi::c_int
    {
        let mut life_var: SIZE = 0;
        life_var = (RandomContext_Random(SysGenRNG) & 1023 as DWORD) as SIZE;
        if (life_var as ::core::ffi::c_int)
            < (*SysInfoPtr).PlanetInfo.LifeChance as ::core::ffi::c_int
            || ((*SysInfoPtr).PlanetInfo.LifeChance as ::core::ffi::c_int)
                < 10 as ::core::ffi::c_int
                && (life_var as ::core::ffi::c_int) < 10 as ::core::ffi::c_int
        {
            let mut num_types: BYTE = 0;
            num_types = (1 as ::core::ffi::c_int
                + RandomContext_Random(SysGenRNG) as UWORD as BYTE as ::core::ffi::c_int
                    % 3 as ::core::ffi::c_int) as BYTE;
            loop {
                let mut index: BYTE = 0;
                let mut num_creatures: BYTE = 0;
                let mut rand_val: UWORD = 0;
                rand_val = RandomContext_Random(SysGenRNG) as UWORD;
                index = (rand_val as BYTE as ::core::ffi::c_int
                    % 23 as ::core::ffi::c_int) as BYTE;
                num_creatures = (1 as ::core::ffi::c_int
                    + (rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as BYTE;
                loop {
                    GenerateRandomLocation(&raw mut (*info).loc_pt);
                    (*info).r#type = index as COUNT;
                    (*info).density = 0 as COUNT;
                    if num_life_forms as ::core::ffi::c_int
                        >= which_life as ::core::ffi::c_int
                        || {
                            num_life_forms = num_life_forms.wrapping_add(1);
                            num_life_forms as usize
                                == ::core::mem::size_of::<DWORD>().wrapping_mul(8usize)
                        }
                    {
                        return num_life_forms;
                    }
                    num_creatures = num_creatures.wrapping_sub(1);
                    if num_creatures == 0 {
                        break;
                    }
                }
                num_types = num_types.wrapping_sub(1);
                if num_types == 0 {
                    break;
                }
            }
        }
    }
    return num_life_forms;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateLifeForms(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut whichLife: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut temp_info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    if info.is_null() {
        info = &raw mut temp_info;
    }
    RandomContext_SeedRandom(
        SysGenRNG,
        (*SysInfoPtr)
            .PlanetInfo
            .ScanSeed[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as usize],
    );
    return CalcLifeForms(SysInfoPtr, whichLife, info);
}
#[no_mangle]
pub unsafe extern "C" fn GeneratePresetLife(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut lifeTypes: *const SBYTE,
    mut whichLife: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut i: COUNT = 0;
    let mut temp_info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    if info.is_null() {
        info = &raw mut temp_info;
    }
    RandomContext_SeedRandom(
        SysGenRNG,
        (*SysInfoPtr)
            .PlanetInfo
            .ScanSeed[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as usize],
    );
    i = 0 as COUNT;
    while (*lifeTypes.offset(i as isize) as ::core::ffi::c_int)
        >= 0 as ::core::ffi::c_int
    {
        GenerateRandomLocation(&raw mut (*info).loc_pt);
        (*info).r#type = *lifeTypes.offset(i as isize) as COUNT;
        (*info).density = 0 as COUNT;
        if i as ::core::ffi::c_int >= whichLife as ::core::ffi::c_int {
            break;
        }
        i = i.wrapping_add(1);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateRandomLocation(mut loc: *mut POINT) {
    let mut rand_val: UWORD = 0;
    rand_val = RandomContext_Random(SysGenRNG) as UWORD;
    (*loc).x = (8 as ::core::ffi::c_int
        + rand_val as BYTE as ::core::ffi::c_int
            % (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))) as COORD;
    (*loc).y = (8 as ::core::ffi::c_int
        + (rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            % (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateRandomNodes(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut scan: COUNT,
    mut numNodes: COUNT,
    mut r#type: COUNT,
    mut whichNode: COUNT,
    mut info: *mut NODE_INFO,
) -> COUNT {
    let mut i: COUNT = 0;
    let mut temp_info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    if info.is_null() {
        info = &raw mut temp_info;
    }
    RandomContext_SeedRandom(
        SysGenRNG,
        (*SysInfoPtr).PlanetInfo.ScanSeed[scan as usize],
    );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < numNodes as ::core::ffi::c_int {
        GenerateRandomLocation(&raw mut (*info).loc_pt);
        (*info).r#type = r#type;
        (*info).density = 0 as COUNT;
        if i as ::core::ffi::c_int >= whichNode as ::core::ffi::c_int {
            break;
        }
        i = i.wrapping_add(1);
    }
    return i;
}
