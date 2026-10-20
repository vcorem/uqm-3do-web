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

// Rust translation of src/uqm/cons_res.c.
extern "C" {
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
}
pub type size_t = usize;
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const FIRST_ROCKY_WORLD: Self = Self(0);
    pub const FIRST_SMALL_ROCKY_WORLD: Self = Self(0);
    pub const OOLITE_WORLD: Self = Self(0);
    pub const YTTRIC_WORLD: Self = Self(1);
    pub const QUASI_DEGENERATE_WORLD: Self = Self(2);
    pub const LANTHANIDE_WORLD: Self = Self(3);
    pub const TREASURE_WORLD: Self = Self(4);
    pub const UREA_WORLD: Self = Self(5);
    pub const METAL_WORLD: Self = Self(6);
    pub const RADIOACTIVE_WORLD: Self = Self(7);
    pub const OPALESCENT_WORLD: Self = Self(8);
    pub const CYANIC_WORLD: Self = Self(9);
    pub const ACID_WORLD: Self = Self(10);
    pub const ALKALI_WORLD: Self = Self(11);
    pub const HALIDE_WORLD: Self = Self(12);
    pub const GREEN_WORLD: Self = Self(13);
    pub const COPPER_WORLD: Self = Self(14);
    pub const CARBIDE_WORLD: Self = Self(15);
    pub const ULTRAMARINE_WORLD: Self = Self(16);
    pub const NOBLE_WORLD: Self = Self(17);
    pub const AZURE_WORLD: Self = Self(18);
    pub const CHONDRITE_WORLD: Self = Self(19);
    pub const PURPLE_WORLD: Self = Self(20);
    pub const SUPER_DENSE_WORLD: Self = Self(21);
    pub const PELLUCID_WORLD: Self = Self(22);
    pub const DUST_WORLD: Self = Self(23);
    pub const CRIMSON_WORLD: Self = Self(24);
    pub const CIMMERIAN_WORLD: Self = Self(25);
    pub const INFRARED_WORLD: Self = Self(26);
    pub const SELENIC_WORLD: Self = Self(27);
    pub const AURIC_WORLD: Self = Self(28);
    pub const LAST_SMALL_ROCKY_WORLD: Self = Self(28);
    pub const FIRST_LARGE_ROCKY_WORLD: Self = Self(29);
    pub const FLUORESCENT_WORLD: Self = Self(29);
    pub const ULTRAVIOLET_WORLD: Self = Self(30);
    pub const PLUTONIC_WORLD: Self = Self(31);
    pub const RAINBOW_WORLD: Self = Self(32);
    pub const SHATTERED_WORLD: Self = Self(33);
    pub const SAPPHIRE_WORLD: Self = Self(34);
    pub const ORGANIC_WORLD: Self = Self(35);
    pub const XENOLITHIC_WORLD: Self = Self(36);
    pub const REDUX_WORLD: Self = Self(37);
    pub const PRIMORDIAL_WORLD: Self = Self(38);
    pub const EMERALD_WORLD: Self = Self(39);
    pub const CHLORINE_WORLD: Self = Self(40);
    pub const MAGNETIC_WORLD: Self = Self(41);
    pub const WATER_WORLD: Self = Self(42);
    pub const TELLURIC_WORLD: Self = Self(43);
    pub const HYDROCARBON_WORLD: Self = Self(44);
    pub const IODINE_WORLD: Self = Self(45);
    pub const VINYLOGOUS_WORLD: Self = Self(46);
    pub const RUBY_WORLD: Self = Self(47);
    pub const MAGMA_WORLD: Self = Self(48);
    pub const MAROON_WORLD: Self = Self(49);
    pub const LAST_LARGE_ROCKY_WORLD: Self = Self(49);
    pub const FIRST_GAS_GIANT: Self = Self(50);
    pub const BLU_GAS_GIANT: Self = Self(50);
    pub const CYA_GAS_GIANT: Self = Self(51);
    pub const GRN_GAS_GIANT: Self = Self(52);
    pub const GRY_GAS_GIANT: Self = Self(53);
    pub const ORA_GAS_GIANT: Self = Self(54);
    pub const PUR_GAS_GIANT: Self = Self(55);
    pub const RED_GAS_GIANT: Self = Self(56);
    pub const VIO_GAS_GIANT: Self = Self(57);
    pub const YEL_GAS_GIANT: Self = Self(58);
    pub const LAST_GAS_GIANT: Self = Self(58);
    pub const NUMBER_OF_PLANET_TYPES: Self = Self(59);
    pub const WORLD_TYPE_SPECIAL: Self = Self(128);
    pub const PLANET_SHIELDED: Self = Self(128);
    pub const HIERARCHY_STARBASE: Self = Self(255);
    pub const SA_MATRA: Self = Self(254);
}
static mut planet_types: [*const ::core::ffi::c_char; 59] = [
    b"oolite\0".as_ptr() as *const ::core::ffi::c_char,
    b"yttric\0".as_ptr() as *const ::core::ffi::c_char,
    b"quasidegenerate\0".as_ptr() as *const ::core::ffi::c_char,
    b"lanthanide\0".as_ptr() as *const ::core::ffi::c_char,
    b"treasure\0".as_ptr() as *const ::core::ffi::c_char,
    b"urea\0".as_ptr() as *const ::core::ffi::c_char,
    b"metal\0".as_ptr() as *const ::core::ffi::c_char,
    b"radioactive\0".as_ptr() as *const ::core::ffi::c_char,
    b"opalescent\0".as_ptr() as *const ::core::ffi::c_char,
    b"cyanic\0".as_ptr() as *const ::core::ffi::c_char,
    b"acid\0".as_ptr() as *const ::core::ffi::c_char,
    b"alkali\0".as_ptr() as *const ::core::ffi::c_char,
    b"halide\0".as_ptr() as *const ::core::ffi::c_char,
    b"green\0".as_ptr() as *const ::core::ffi::c_char,
    b"copper\0".as_ptr() as *const ::core::ffi::c_char,
    b"carbide\0".as_ptr() as *const ::core::ffi::c_char,
    b"ultramarine\0".as_ptr() as *const ::core::ffi::c_char,
    b"noble\0".as_ptr() as *const ::core::ffi::c_char,
    b"azure\0".as_ptr() as *const ::core::ffi::c_char,
    b"chondrite\0".as_ptr() as *const ::core::ffi::c_char,
    b"purple\0".as_ptr() as *const ::core::ffi::c_char,
    b"superdense\0".as_ptr() as *const ::core::ffi::c_char,
    b"pellucid\0".as_ptr() as *const ::core::ffi::c_char,
    b"dust\0".as_ptr() as *const ::core::ffi::c_char,
    b"crimson\0".as_ptr() as *const ::core::ffi::c_char,
    b"cimmerian\0".as_ptr() as *const ::core::ffi::c_char,
    b"infrared\0".as_ptr() as *const ::core::ffi::c_char,
    b"selenic\0".as_ptr() as *const ::core::ffi::c_char,
    b"auric\0".as_ptr() as *const ::core::ffi::c_char,
    b"fluorescent\0".as_ptr() as *const ::core::ffi::c_char,
    b"ultraviolet\0".as_ptr() as *const ::core::ffi::c_char,
    b"plutonic\0".as_ptr() as *const ::core::ffi::c_char,
    b"rainbow\0".as_ptr() as *const ::core::ffi::c_char,
    b"shattered\0".as_ptr() as *const ::core::ffi::c_char,
    b"sapphire\0".as_ptr() as *const ::core::ffi::c_char,
    b"organic\0".as_ptr() as *const ::core::ffi::c_char,
    b"xenolithic\0".as_ptr() as *const ::core::ffi::c_char,
    b"redux\0".as_ptr() as *const ::core::ffi::c_char,
    b"primordial\0".as_ptr() as *const ::core::ffi::c_char,
    b"emerald\0".as_ptr() as *const ::core::ffi::c_char,
    b"chlorine\0".as_ptr() as *const ::core::ffi::c_char,
    b"magnetic\0".as_ptr() as *const ::core::ffi::c_char,
    b"water\0".as_ptr() as *const ::core::ffi::c_char,
    b"telluric\0".as_ptr() as *const ::core::ffi::c_char,
    b"hydrocarbon\0".as_ptr() as *const ::core::ffi::c_char,
    b"iodine\0".as_ptr() as *const ::core::ffi::c_char,
    b"vinylogous\0".as_ptr() as *const ::core::ffi::c_char,
    b"ruby\0".as_ptr() as *const ::core::ffi::c_char,
    b"magma\0".as_ptr() as *const ::core::ffi::c_char,
    b"maroon\0".as_ptr() as *const ::core::ffi::c_char,
    b"bluegas\0".as_ptr() as *const ::core::ffi::c_char,
    b"cyangas\0".as_ptr() as *const ::core::ffi::c_char,
    b"greengas\0".as_ptr() as *const ::core::ffi::c_char,
    b"greygas\0".as_ptr() as *const ::core::ffi::c_char,
    b"orangegas\0".as_ptr() as *const ::core::ffi::c_char,
    b"purplegas\0".as_ptr() as *const ::core::ffi::c_char,
    b"redgas\0".as_ptr() as *const ::core::ffi::c_char,
    b"violetgas\0".as_ptr() as *const ::core::ffi::c_char,
    b"yellowgas\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut planet_sizes: [*const ::core::ffi::c_char; 3] = [
    b"large\0".as_ptr() as *const ::core::ffi::c_char,
    b"medium\0".as_ptr() as *const ::core::ffi::c_char,
    b"small\0".as_ptr() as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut planet: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
static mut buffer: [::core::ffi::c_char; 80] = [0; 80];
#[no_mangle]
pub unsafe extern "C" fn load_gravity_well(mut selector: BYTE) {
    let mut i: COUNT = 0;
    if selector as ::core::ffi::c_int
        == C2Rust_Unnamed::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
    {
        planet[0usize] = CaptureDrawable(
            LoadGraphicInstance(b"planet.samatra.large\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        planet[2usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        planet[1usize] = planet[2usize];
    } else {
        let mut ptype: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if selector as ::core::ffi::c_int
            & C2Rust_Unnamed::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
        {
            ptype = b"slaveshield\0".as_ptr() as *const ::core::ffi::c_char;
        } else {
            ptype = planet_types[selector as usize];
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int)
            < 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                79 as size_t,
                b"planet.%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                ptype,
                planet_sizes[i as usize],
            );
            buffer[79usize] = '\0' as ::core::ffi::c_char;
            planet[i as usize] = CaptureDrawable(
                LoadGraphicInstance(
                    &raw mut buffer as *mut ::core::ffi::c_char as RESOURCE,
                ) as DRAWABLE,
            );
            i = i.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn free_gravity_well() {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        DestroyDrawable(ReleaseDrawable(planet[i as usize]));
        planet[i as usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_life_form(mut selector: BYTE) -> FRAME {
    snprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        79 as size_t,
        b"graphics.life.%d\0".as_ptr() as *const ::core::ffi::c_char,
        selector as ::core::ffi::c_int,
    );
    buffer[79usize] = '\0' as ::core::ffi::c_char;
    return CaptureDrawable(
        LoadGraphicInstance(&raw mut buffer as *mut ::core::ffi::c_char as RESOURCE)
            as DRAWABLE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn load_orbit_theme(mut selector: BYTE) -> MUSIC_REF {
    snprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        79 as size_t,
        b"music.orbit%d\0".as_ptr() as *const ::core::ffi::c_char,
        selector as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    );
    buffer[79usize] = '\0' as ::core::ffi::c_char;
    return LoadMusicInstance(&raw mut buffer as *mut ::core::ffi::c_char as RESOURCE);
}
