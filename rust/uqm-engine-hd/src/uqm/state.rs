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

// Rust translation of src/uqm/state.c.
extern "C" {
    pub type frame_desc;
    pub type font_desc;
    pub type string_table_entry;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
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
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE_FILE {
    pub symname: *const ::core::ffi::c_char,
    pub size_hint: DWORD,
    pub open_count: ::core::ffi::c_int,
    pub data: *mut BYTE,
    pub used: DWORD,
    pub size: DWORD,
    pub ptr: DWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
pub type COORD = SWORD;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
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
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
pub type SOLARSYS_STATE = solarsys_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
    pub ScanFrame: [FRAME; 3],
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScanColors: *mut *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
    pub BackFrame: FRAME,
    pub light_diff: *mut *mut DWORD,
    pub map_rotate: *mut *mut MAP3D_POINT,
    pub TopoMask: FRAME,
    pub sphereBytes: *mut BYTE,
    pub sphereMap: COLORMAP,
    pub scanType: COUNT,
    pub Shade: FRAME,
    pub ShadeColors: *mut Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type COLORMAP = STRING;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PLANET_DESC = planet_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
    pub orbit: PLANET_ORBIT,
    pub size: COUNT,
    pub angle: COUNT,
    pub rotFrameIndex: ::core::ffi::c_int,
    pub rotPointIndex: ::core::ffi::c_int,
    pub rotwidth: ::core::ffi::c_int,
    pub rotheight: ::core::ffi::c_int,
    pub orb_speed: ::core::ffi::c_double,
    pub rot_speed: ::core::ffi::c_double,
    pub PlanetByte: BYTE,
    pub MoonByte: BYTE,
    pub frame_offset: BYTE,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
static mut state_files: [GAME_STATE_FILE; 3] = [GAME_STATE_FILE {
    symname: ::core::ptr::null::<::core::ffi::c_char>(),
    size_hint: 0,
    open_count: 0,
    data: ::core::ptr::null_mut::<BYTE>(),
    used: 0,
    size: 0,
    ptr: 0,
}; 3];
#[no_mangle]
pub unsafe extern "C" fn OpenStateFile(
    mut stateFile: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> *mut GAME_STATE_FILE {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    if stateFile < 0 as ::core::ffi::c_int || stateFile >= 3 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GAME_STATE_FILE>();
    }
    fp = (&raw mut state_files as *mut GAME_STATE_FILE).offset(stateFile as isize);
    (*fp).open_count += 1;
    if (*fp).open_count > 1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"WARNING: State file %s open count is %d after open()\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*fp).symname,
            (*fp).open_count,
        );
    }
    if (*fp).data.is_null() {
        (*fp).data = HMalloc((*fp).size_hint as size_t) as *mut BYTE;
        if (*fp).data.is_null() {
            return ::core::ptr::null_mut::<GAME_STATE_FILE>();
        }
        (*fp).size = (*fp).size_hint;
    }
    if *mode.offset(0isize) as ::core::ffi::c_int == 'w' as ::core::ffi::c_int {
        (*fp).used = 0 as DWORD;
        memset(
            (*fp).data as *mut ::core::ffi::c_void,
            0xcc as ::core::ffi::c_int,
            (*fp).size as size_t,
        );
    } else if *mode.offset(0isize) as ::core::ffi::c_int != 'r' as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"WARNING: State file %s opened with unsupported mode '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*fp).symname,
            mode,
        );
    }
    (*fp).ptr = 0 as DWORD;
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn CloseStateFile(mut fp: *mut GAME_STATE_FILE) {
    (*fp).ptr = 0 as DWORD;
    (*fp).open_count -= 1;
    if (*fp).open_count < 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"WARNING: State file %s open count is %d after close()\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*fp).symname,
            (*fp).open_count,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn DeleteStateFile(mut stateFile: ::core::ffi::c_int) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    if stateFile < 0 as ::core::ffi::c_int || stateFile >= 3 as ::core::ffi::c_int {
        return;
    }
    fp = (&raw mut state_files as *mut GAME_STATE_FILE).offset(stateFile as isize);
    if (*fp).open_count != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"WARNING: State file %s open count is %d during delete()\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*fp).symname,
            (*fp).open_count,
        );
    }
    (*fp).used = 0 as DWORD;
    (*fp).ptr = 0 as DWORD;
    HFree((*fp).data as *mut ::core::ffi::c_void);
    (*fp).data = ::core::ptr::null_mut::<BYTE>();
}
#[no_mangle]
pub unsafe extern "C" fn LengthStateFile(mut fp: *mut GAME_STATE_FILE) -> DWORD {
    return (*fp).used;
}
#[no_mangle]
pub unsafe extern "C" fn ReadStateFile(
    mut lpBuf: *mut ::core::ffi::c_void,
    mut size: COUNT,
    mut count: COUNT,
    mut fp: *mut GAME_STATE_FILE,
) -> ::core::ffi::c_int {
    let mut bytes: DWORD = (size as ::core::ffi::c_int * count as ::core::ffi::c_int)
        as DWORD;
    if (*fp).ptr >= (*fp).size {
        return 0 as ::core::ffi::c_int
    } else if (*fp).ptr.wrapping_add(bytes) > (*fp).size {
        bytes = (*fp).size.wrapping_sub((*fp).ptr);
        bytes = bytes.wrapping_sub(bytes.wrapping_rem(size as DWORD));
    }
    if bytes > 0 as DWORD {
        memcpy(
            lpBuf,
            (*fp).data.offset((*fp).ptr as isize) as *const ::core::ffi::c_void,
            bytes as size_t,
        );
        (*fp).ptr = (*fp).ptr.wrapping_add(bytes);
    }
    return bytes.wrapping_div(size as DWORD) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WriteStateFile(
    mut lpBuf: *const ::core::ffi::c_void,
    mut size: COUNT,
    mut count: COUNT,
    mut fp: *mut GAME_STATE_FILE,
) -> ::core::ffi::c_int {
    let mut bytes: DWORD = (size as ::core::ffi::c_int * count as ::core::ffi::c_int)
        as DWORD;
    if (*fp).ptr.wrapping_add(bytes) > (*fp).size {
        let mut newsize: DWORD = (*fp).ptr.wrapping_add(bytes);
        if newsize < (*fp).size.wrapping_mul(3 as DWORD).wrapping_div(2 as DWORD) {
            newsize = (*fp).size.wrapping_mul(3 as DWORD).wrapping_div(2 as DWORD);
        }
        (*fp).data = HRealloc((*fp).data as *mut ::core::ffi::c_void, newsize as size_t)
            as *mut BYTE;
        if (*fp).data.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*fp).size = newsize;
        if newsize > (*fp).size_hint {
            (*fp).size_hint = newsize;
        }
    }
    if bytes > 0 as DWORD {
        memcpy(
            (*fp).data.offset((*fp).ptr as isize) as *mut ::core::ffi::c_void,
            lpBuf,
            bytes as size_t,
        );
        (*fp).ptr = (*fp).ptr.wrapping_add(bytes);
        if (*fp).ptr > (*fp).used {
            (*fp).used = (*fp).ptr;
        }
    }
    return bytes.wrapping_div(size as DWORD) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SeekStateFile(
    mut fp: *mut GAME_STATE_FILE,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if whence == 1 as ::core::ffi::c_int {
        offset = (offset as ::core::ffi::c_ulong)
            .wrapping_add((*fp).ptr as ::core::ffi::c_ulong) as ::core::ffi::c_long;
    } else if whence == 2 as ::core::ffi::c_int {
        offset = (offset as ::core::ffi::c_ulong)
            .wrapping_add((*fp).used as ::core::ffi::c_ulong) as ::core::ffi::c_long;
    }
    if offset < 0 as ::core::ffi::c_long {
        (*fp).ptr = 0 as DWORD;
        return 0 as ::core::ffi::c_int;
    }
    (*fp).ptr = offset as DWORD;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn InitPlanetInfo() {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut pSD: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        pSD = (&raw mut star_array as *mut STAR_DESC).offset(0isize);
        loop {
            swrite_32(fp, 0 as DWORD);
            pSD = pSD.offset(1);
            if !((*pSD).star_pt.x as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int
                && (*pSD).star_pt.y as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int)
            {
                break;
            }
        }
        CloseStateFile(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UninitPlanetInfo() {
    DeleteStateFile(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GetPlanetInfo() {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut k: COUNT = 0;
    let mut l: COUNT = 0;
    (*pSolarSysState)
        .SysInfo
        .PlanetInfo
        .ScanRetrieveMask[PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int
        as usize] = 0 as DWORD;
    (*pSolarSysState)
        .SysInfo
        .PlanetInfo
        .ScanRetrieveMask[PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
        as usize] = 0 as DWORD;
    (*pSolarSysState)
        .SysInfo
        .PlanetInfo
        .ScanRetrieveMask[PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
        as usize] = 0 as DWORD;
    l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
    while (l as ::core::ffi::c_int)
        < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
    {
        k = 0 as COUNT;
        while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            (*pSolarSysState)
                .SysInfo
                .PlanetInfo
                .PartiallyScavengedList[l as usize][k as usize] = 0 as BYTE;
            k = k.wrapping_add(1);
        }
        l = l.wrapping_add(1);
    }
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut star_index: COUNT = 0;
        let mut planet_index: COUNT = 0;
        let mut moon_index: COUNT = 0;
        let mut offset: DWORD = 0;
        star_index = CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
            as COUNT;
        planet_index = (*(*pSolarSysState).pBaseDesc)
            .pPrevDesc
            .offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
            as COUNT;
        if (*(*pSolarSysState).pOrbitalDesc).pPrevDesc
            == &raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC
        {
            moon_index = 0 as COUNT;
        } else {
            moon_index = ((*pSolarSysState)
                .pOrbitalDesc
                .offset_from(&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                + 1isize) as COUNT;
        }
        SeekStateFile(
            fp,
            (star_index as usize).wrapping_mul(::core::mem::size_of::<DWORD>())
                as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_32(fp, &raw mut offset);
        if offset != 0 {
            let mut i: COUNT = 0;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < planet_index as ::core::ffi::c_int {
                offset = (offset as ::core::ffi::c_ulong)
                    .wrapping_add(
                        (((*pSolarSysState).PlanetDesc[i as usize].NumPlanets
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<DWORD>()
                                    .wrapping_mul(
                                        PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                            as usize,
                                    )
                                    .wrapping_add(
                                        ::core::mem::size_of::<BYTE>()
                                            .wrapping_mul(
                                                PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                                    as usize,
                                            )
                                            .wrapping_mul(32usize),
                                    ),
                            ) as ::core::ffi::c_ulong,
                    ) as DWORD;
                i = i.wrapping_add(1);
            }
            offset = (offset as ::core::ffi::c_ulong)
                .wrapping_add(
                    (moon_index as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<DWORD>()
                                .wrapping_mul(
                                    PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                        as usize,
                                )
                                .wrapping_add(
                                    ::core::mem::size_of::<BYTE>()
                                        .wrapping_mul(
                                            PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                                as usize,
                                        )
                                        .wrapping_mul(32usize),
                                ),
                        ) as ::core::ffi::c_ulong,
                ) as DWORD;
            SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
            sread_a32(
                fp,
                &raw mut (*pSolarSysState).SysInfo.PlanetInfo.ScanRetrieveMask
                    as *mut DWORD,
                PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as COUNT,
            );
            let mut ar: *mut BYTE = (&raw mut *(&raw mut (*pSolarSysState)
                .SysInfo
                .PlanetInfo
                .PartiallyScavengedList as *mut [BYTE; 32])
                .offset(0isize) as *mut BYTE)
                .offset(0isize);
            l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
            while (l as ::core::ffi::c_int)
                < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
            {
                k = 0 as COUNT;
                while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    sread_8(fp, ar);
                    k = k.wrapping_add(1);
                    ar = ar.offset(1);
                }
                l = l.wrapping_add(1);
            }
        }
        CloseStateFile(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PutPlanetInfo() {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut i: COUNT = 0;
        let mut k: COUNT = 0;
        let mut l: COUNT = 0;
        let mut star_index: COUNT = 0;
        let mut planet_index: COUNT = 0;
        let mut moon_index: COUNT = 0;
        let mut offset: DWORD = 0;
        star_index = CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
            as COUNT;
        planet_index = (*(*pSolarSysState).pBaseDesc)
            .pPrevDesc
            .offset_from(&raw mut (*pSolarSysState).PlanetDesc as *mut PLANET_DESC)
            as COUNT;
        if (*(*pSolarSysState).pOrbitalDesc).pPrevDesc
            == &raw mut (*pSolarSysState).SunDesc as *mut PLANET_DESC
        {
            moon_index = 0 as COUNT;
        } else {
            moon_index = ((*pSolarSysState)
                .pOrbitalDesc
                .offset_from(&raw mut (*pSolarSysState).MoonDesc as *mut PLANET_DESC)
                + 1isize) as COUNT;
        }
        SeekStateFile(
            fp,
            (star_index as usize).wrapping_mul(::core::mem::size_of::<DWORD>())
                as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_32(fp, &raw mut offset);
        if offset == 0 as DWORD {
            let mut ScanRetrieveMask: [DWORD; 3] = [0 as DWORD, 0 as DWORD, 0 as DWORD];
            let mut PartiallyScavengedList: [[BYTE; 32]; 3] = [[0; 32]; 3];
            l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
            while (l as ::core::ffi::c_int)
                < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
            {
                k = 0 as COUNT;
                while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    PartiallyScavengedList[l as usize][k as usize] = 0 as BYTE;
                    k = k.wrapping_add(1);
                }
                l = l.wrapping_add(1);
            }
            offset = LengthStateFile(fp);
            SeekStateFile(
                fp,
                (star_index as usize).wrapping_mul(::core::mem::size_of::<DWORD>())
                    as ::core::ffi::c_long,
                0 as ::core::ffi::c_int,
            );
            swrite_32(fp, offset);
            SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < (*pSolarSysState).SunDesc[0usize].NumPlanets as ::core::ffi::c_int
            {
                let mut j: COUNT = 0;
                let mut ar: *mut BYTE = (&raw mut *(&raw mut PartiallyScavengedList
                    as *mut [BYTE; 32])
                    .offset(0isize) as *mut BYTE)
                    .offset(0isize);
                swrite_a32(
                    fp,
                    &raw mut ScanRetrieveMask as *mut DWORD,
                    PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as COUNT,
                );
                l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
                while (l as ::core::ffi::c_int)
                    < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                {
                    k = 0 as COUNT;
                    while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                        swrite_8(fp, *ar);
                        k = k.wrapping_add(1);
                        ar = ar.offset(1);
                    }
                    l = l.wrapping_add(1);
                }
                j = 0 as COUNT;
                while (j as ::core::ffi::c_int)
                    < (*pSolarSysState).PlanetDesc[i as usize].NumPlanets
                        as ::core::ffi::c_int
                {
                    let mut ar_0: *mut BYTE = (&raw mut *(&raw mut PartiallyScavengedList
                        as *mut [BYTE; 32])
                        .offset(0isize) as *mut BYTE)
                        .offset(0isize);
                    swrite_a32(
                        fp,
                        &raw mut ScanRetrieveMask as *mut DWORD,
                        PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as COUNT,
                    );
                    l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
                    while (l as ::core::ffi::c_int)
                        < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                    {
                        k = 0 as COUNT;
                        while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                            swrite_8(fp, *ar_0);
                            k = k.wrapping_add(1);
                            ar_0 = ar_0.offset(1);
                        }
                        l = l.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < planet_index as ::core::ffi::c_int {
            offset = (offset as ::core::ffi::c_ulong)
                .wrapping_add(
                    (((*pSolarSysState).PlanetDesc[i as usize].NumPlanets
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<DWORD>()
                                .wrapping_mul(
                                    PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                        as usize,
                                )
                                .wrapping_add(
                                    ::core::mem::size_of::<BYTE>()
                                        .wrapping_mul(
                                            PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                                as usize,
                                        )
                                        .wrapping_mul(32usize),
                                ),
                        ) as ::core::ffi::c_ulong,
                ) as DWORD;
            i = i.wrapping_add(1);
        }
        offset = (offset as ::core::ffi::c_ulong)
            .wrapping_add(
                (moon_index as usize)
                    .wrapping_mul(
                        ::core::mem::size_of::<DWORD>()
                            .wrapping_mul(
                                PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                    as usize,
                            )
                            .wrapping_add(
                                ::core::mem::size_of::<BYTE>()
                                    .wrapping_mul(
                                        PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
                                            as usize,
                                    )
                                    .wrapping_mul(32usize),
                            ),
                    ) as ::core::ffi::c_ulong,
            ) as DWORD;
        SeekStateFile(fp, offset as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
        swrite_a32(
            fp,
            &raw mut (*pSolarSysState).SysInfo.PlanetInfo.ScanRetrieveMask as *mut DWORD,
            PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int as COUNT,
        );
        let mut ar_1: *mut BYTE = (&raw mut *(&raw mut (*pSolarSysState)
            .SysInfo
            .PlanetInfo
            .PartiallyScavengedList as *mut [BYTE; 32])
            .offset(0isize) as *mut BYTE)
            .offset(0isize);
        l = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
        while (l as ::core::ffi::c_int)
            < PlanetScanTypes::NUM_SCAN_TYPES.0 as ::core::ffi::c_int
        {
            k = 0 as COUNT;
            while (k as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                swrite_8(fp, *ar_1);
                k = k.wrapping_add(1);
                ar_1 = ar_1.offset(1);
            }
            l = l.wrapping_add(1);
        }
        CloseStateFile(fp);
    }
}
#[inline]
unsafe extern "C" fn sread_8(mut fp: *mut GAME_STATE_FILE, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_32(mut fp: *mut GAME_STATE_FILE, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_a32(
    mut fp: *mut GAME_STATE_FILE,
    mut ar: *mut DWORD,
    mut count: COUNT,
) -> COUNT {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/state.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                124 as ::core::ffi::c_int,
                b"sread_a32\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if sread_32(fp, ar) as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            return 0 as COUNT;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as COUNT;
}
#[inline]
unsafe extern "C" fn swrite_8(mut fp: *mut GAME_STATE_FILE, mut v: BYTE) -> COUNT {
    return WriteStateFile(
        &raw mut v as *const ::core::ffi::c_void,
        1 as COUNT,
        1 as COUNT,
        fp,
    ) as COUNT;
}
#[inline]
unsafe extern "C" fn swrite_32(mut fp: *mut GAME_STATE_FILE, mut v: DWORD) -> COUNT {
    return WriteStateFile(
        &raw mut v as *const ::core::ffi::c_void,
        4 as COUNT,
        1 as COUNT,
        fp,
    ) as COUNT;
}
#[inline]
unsafe extern "C" fn swrite_a32(
    mut fp: *mut GAME_STATE_FILE,
    mut ar: *const DWORD,
    mut count: COUNT,
) -> COUNT {
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if swrite_32(fp, *ar) as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            return 0 as COUNT;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as COUNT;
}
#[export_name = "uqm_c2rust_init_0e4888f246ed0b4b"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    state_files = [
        GAME_STATE_FILE {
            symname: b"STARINFO\0".as_ptr() as *const ::core::ffi::c_char,
            size_hint: 502usize
                .wrapping_mul(::core::mem::size_of::<DWORD>())
                .wrapping_add(
                    3800usize
                        .wrapping_mul(
                            3usize.wrapping_mul(::core::mem::size_of::<DWORD>()),
                        ),
                ) as DWORD,
            open_count: 0 as ::core::ffi::c_int,
            data: ::core::ptr::null_mut::<BYTE>(),
            used: 0 as DWORD,
            size: 0 as DWORD,
            ptr: 0 as DWORD,
        },
        GAME_STATE_FILE {
            symname: b"RANDGRPINFO\0".as_ptr() as *const ::core::ffi::c_char,
            size_hint: (4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as DWORD,
            open_count: 0 as ::core::ffi::c_int,
            data: ::core::ptr::null_mut::<BYTE>(),
            used: 0 as DWORD,
            size: 0 as DWORD,
            ptr: 0 as DWORD,
        },
        GAME_STATE_FILE {
            symname: b"DEFGRPINFO\0".as_ptr() as *const ::core::ffi::c_char,
            size_hint: (10 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as DWORD,
            open_count: 0 as ::core::ffi::c_int,
            data: ::core::ptr::null_mut::<BYTE>(),
            used: 0 as DWORD,
            size: 0 as DWORD,
            ptr: 0 as DWORD,
        },
    ];
}
// Explicit engine initialization replaces platform constructor tables.
