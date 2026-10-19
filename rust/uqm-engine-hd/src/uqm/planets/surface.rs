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
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type RandomContext;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    static mut SysGenRNG: *mut RandomContext;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
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
    pub PlanetToSunDist: DWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub PartiallyScavengedList: [[BYTE; 32]; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
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
            let mut temp_deposit_quality: SDWORD = 0;
            deposit_quality_fine = (RandomContext_Random(SysGenRNG) as UWORD
                as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                + (((*eptr).Density as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int + (*SysInfoPtr).StarSize as ::core::ffi::c_int)
                    * 50 as ::core::ffi::c_int) as COUNT;
            if (which_deposit as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                temp_deposit_quality = (deposit_quality_fine as ::core::ffi::c_int
                    - (*SysInfoPtr)
                        .PlanetInfo
                        .PartiallyScavengedList[PlanetScanTypes::MINERAL_SCAN.0
                        as ::core::ffi::c_int as usize][which_deposit as usize]
                        as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as SDWORD;
            } else {
                temp_deposit_quality = deposit_quality_fine as SDWORD;
            }
            if temp_deposit_quality < 0 as ::core::ffi::c_int {
                temp_deposit_quality = 0 as ::core::ffi::c_int as SDWORD;
            }
            if temp_deposit_quality < 150 as ::core::ffi::c_int {
                deposit_quality_gross = 0 as COUNT;
            } else if temp_deposit_quality < 225 as ::core::ffi::c_int {
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
unsafe extern "C" fn widthPick() -> COORD {
    let mut map_dimensions: [EXTENT; 2] = [
        extent {
            width: 210 as COORD,
            height: 67 as COORD,
        },
        extent {
            width: (243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            height: 75 as COORD,
        },
    ];
    return map_dimensions[optPlanetTexture.0 as usize].width;
}
unsafe extern "C" fn heightPick() -> COORD {
    let mut map_dimensions: [EXTENT; 2] = [
        extent {
            width: 210 as COORD,
            height: 67 as COORD,
        },
        extent {
            width: (243 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            height: 75 as COORD,
        },
    ];
    return map_dimensions[optPlanetTexture.0 as usize].height;
}
unsafe extern "C" fn scaleMapWidth(mut value: COORD) -> COORD {
    let mut percentage: ::core::ffi::c_float = 1 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    if if is3DO(optSuperPC).0 != 0 {
        243 as ::core::ffi::c_int
    } else {
        210 as ::core::ffi::c_int
    }
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            33 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != widthPick() as ::core::ffi::c_int
    {
        percentage = scaleThing(
            (if is3DO(optSuperPC).0 != 0 {
                243 as ::core::ffi::c_int
            } else {
                210 as ::core::ffi::c_int
            }
                - if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    33 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_float,
            widthPick() as ::core::ffi::c_float,
        );
    }
    return (((value as ::core::ffi::c_int as ::core::ffi::c_float * percentage) as COORD
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
unsafe extern "C" fn scaleMapHeight(mut value: COORD) -> COORD {
    let mut percentage: ::core::ffi::c_float = 1 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    if 67 as ::core::ffi::c_int != heightPick() as ::core::ffi::c_int {
        percentage = scaleThing(
            67 as ::core::ffi::c_int as ::core::ffi::c_float,
            heightPick() as ::core::ffi::c_float,
        );
    }
    return (((value as ::core::ffi::c_int as ::core::ffi::c_float * percentage) as COORD
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateRandomLocation(mut loc: *mut POINT) {
    let mut rand_val: UWORD = 0;
    rand_val = RandomContext_Random(SysGenRNG) as UWORD;
    (*loc).x = scaleMapWidth(
        (8 as ::core::ffi::c_int
            + rand_val as BYTE as ::core::ffi::c_int
                % (widthPick() as ::core::ffi::c_int
                    - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))) as COORD,
    );
    (*loc).y = scaleMapHeight(
        (8 as ::core::ffi::c_int
            + (rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                % (heightPick() as ::core::ffi::c_int
                    - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))) as COORD,
    );
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
#[no_mangle]
pub unsafe extern "C" fn CustomMineralDeposits(
    mut SysInfoPtr: *const SYSTEM_INFO,
    mut which_deposit: COUNT,
    mut info: *mut NODE_INFO,
    mut numNodes: COUNT,
    mut r#type: COUNT,
    mut quality: BYTE,
) -> COUNT {
    let mut j: BYTE = 0;
    let mut num_deposits: COUNT = 0;
    let mut OpenSeason: BOOLEAN = BOOLEAN::TRUE;
    let mut temp_info: NODE_INFO = NODE_INFO {
        loc_pt: POINT { x: 0, y: 0 },
        density: 0,
        r#type: 0,
    };
    let mut eptr: *const ELEMENT_ENTRY = ::core::ptr::null::<ELEMENT_ENTRY>();
    if info.is_null() {
        info = &raw mut temp_info;
    }
    RandomContext_SeedRandom(
        SysGenRNG,
        (*SysInfoPtr)
            .PlanetInfo
            .ScanSeed[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as usize],
    );
    eptr = (&raw const (*(*SysInfoPtr).PlanetInfo.PlanDataPtr).UsefulElements
        as *const ELEMENT_ENTRY)
        .offset(0isize);
    num_deposits = 0 as COUNT;
    j = 8 as BYTE;
    loop {
        let mut num_possible: BYTE = 0;
        let mut depositQuality: BYTE = 0 as BYTE;
        num_possible = (RandomContext_Random(SysGenRNG) as UWORD as BYTE
            as ::core::ffi::c_int
            % (((*eptr).Density as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as BYTE;
        if num_possible as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && OpenSeason.0 != 0
        {
            num_possible = numNodes as BYTE;
            (*info).r#type = r#type;
            depositQuality = quality;
            OpenSeason = BOOLEAN::FALSE;
        } else {
            depositQuality = ((*eptr).Density as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
            (*info).r#type = (*eptr).ElementType as COUNT;
        }
        loop {
            let c2rust_fresh1 = num_possible;
            num_possible = num_possible.wrapping_sub(1);
            if c2rust_fresh1 == 0 {
                break;
            }
            let mut deposit_quality_fine: COUNT = 0;
            let mut deposit_quality_gross: COUNT = 0;
            let mut temp_deposit_quality: SDWORD = 0;
            deposit_quality_fine = (RandomContext_Random(SysGenRNG) as UWORD
                as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                + (depositQuality as ::core::ffi::c_int
                    + (*SysInfoPtr).StarSize as ::core::ffi::c_int)
                    * 50 as ::core::ffi::c_int) as COUNT;
            if (which_deposit as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                temp_deposit_quality = (deposit_quality_fine as ::core::ffi::c_int
                    - (*SysInfoPtr)
                        .PlanetInfo
                        .PartiallyScavengedList[PlanetScanTypes::MINERAL_SCAN.0
                        as ::core::ffi::c_int as usize][which_deposit as usize]
                        as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as SDWORD;
            } else {
                temp_deposit_quality = deposit_quality_fine as SDWORD;
            }
            if temp_deposit_quality < 0 as ::core::ffi::c_int {
                temp_deposit_quality = 0 as ::core::ffi::c_int as SDWORD;
            }
            if temp_deposit_quality < 150 as ::core::ffi::c_int {
                deposit_quality_gross = 0 as COUNT;
            } else if temp_deposit_quality < 225 as ::core::ffi::c_int {
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
pub unsafe extern "C" fn CustomMineralDeposit(
    mut info: *mut NODE_INFO,
    mut r#type: COUNT,
    mut quality: BYTE,
    mut location: POINT,
) -> COUNT {
    let mut deposit_quality_fine: COUNT = 0;
    let mut deposit_quality_gross: COUNT = 0;
    if info.is_null() {
        return 0 as COUNT;
    }
    (*info).r#type = r#type;
    deposit_quality_fine = (quality as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
        as COUNT;
    if (deposit_quality_fine as ::core::ffi::c_int) < 150 as ::core::ffi::c_int {
        deposit_quality_gross = 0 as COUNT;
    } else if (deposit_quality_fine as ::core::ffi::c_int) < 225 as ::core::ffi::c_int {
        deposit_quality_gross = 1 as COUNT;
    } else {
        deposit_quality_gross = 2 as COUNT;
    }
    (*info).loc_pt = location;
    (*info).density = (((quality as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as UWORD as ::core::ffi::c_int
        | deposit_quality_gross as BYTE as ::core::ffi::c_int) as COUNT;
    return 1 as COUNT;
}
#[inline]
unsafe extern "C" fn scaleThing(
    mut original: ::core::ffi::c_float,
    mut thingToScale: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return original / thingToScale;
}
