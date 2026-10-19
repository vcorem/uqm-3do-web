/*

  rotozoom.c - rotozoomer for 32bit or 8bit surfaces
  LGPL (c) A. Schiffler

  Note by sc2 developers:
  Taken from SDL_gfx library and modified, original code can be downloaded
  from http://www.ferzkopp.net/Software/SDL_gfx-2.0/

*/

// Rust translation of src/libs/graphics/sdl/rotozoom.c.
use alloc::vec::Vec;
extern "C" {
    pub type SDL_BlitMap;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SDL_CreateRGBSurface(
        flags: Uint32,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        depth: ::core::ffi::c_int,
        Rmask: Uint32,
        Gmask: Uint32,
        Bmask: Uint32,
        Amask: Uint32,
    ) -> *mut SDL_Surface;
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    fn SDL_LockSurface(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    fn SDL_UpperBlit(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> ::core::ffi::c_int;
    fn TFB_SetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: Uint8,
    ) -> ::core::ffi::c_int;
    fn TFB_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut Uint32,
    ) -> ::core::ffi::c_int;
    fn TFB_SetColorKey(
        surface: *mut SDL_Surface,
        key: Uint32,
        rleaccel: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_ulong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type Uint8 = uint8_t;
pub type Uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: ::core::ffi::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut SDL_PixelFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub pitch: ::core::ffi::c_int,
    pub pixels: *mut ::core::ffi::c_void,
    pub userdata: *mut ::core::ffi::c_void,
    pub locked: ::core::ffi::c_int,
    pub list_blitmap: *mut ::core::ffi::c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tColorRGBA {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tColorY {
    pub y: Uint8,
}
#[no_mangle]
pub unsafe extern "C" fn zoomSurfaceRGBA(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut smooth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c2rust_alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut sx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut sax: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut say: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut csax: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut csay: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut csx: ::core::ffi::c_int = 0;
    let mut csy: ::core::ffi::c_int = 0;
    let mut ex: ::core::ffi::c_int = 0;
    let mut ey: ::core::ffi::c_int = 0;
    let mut t1: ::core::ffi::c_int = 0;
    let mut t2: ::core::ffi::c_int = 0;
    let mut sstep: ::core::ffi::c_int = 0;
    let mut c00: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut c01: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut c10: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut c11: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut sp: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut csp: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut dp: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut sgap: ::core::ffi::c_int = 0;
    let mut dgap: ::core::ffi::c_int = 0;
    if smooth != 0 {
        sx = (65536.0f64
            * ((*src).w - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                as ::core::ffi::c_double
            / (*dst).w as ::core::ffi::c_float as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        sy = (65536.0f64
            * ((*src).h - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
                as ::core::ffi::c_double
            / (*dst).h as ::core::ffi::c_float as ::core::ffi::c_double)
            as ::core::ffi::c_int;
    } else {
        sx = (65536.0f64 * (*src).w as ::core::ffi::c_float as ::core::ffi::c_double
            / (*dst).w as ::core::ffi::c_float as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        sy = (65536.0f64 * (*src).h as ::core::ffi::c_float as ::core::ffi::c_double
            / (*dst).h as ::core::ffi::c_float as ::core::ffi::c_double)
            as ::core::ffi::c_int;
    }
    c2rust_alloca_allocations
        .push(
            ::alloc::vec::from_elem(
                0,
                (((*dst).w + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<Uint32>()) as usize,
            ),
        );
    sax = c2rust_alloca_allocations.last_mut().unwrap().as_mut_ptr()
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    if sax.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    c2rust_alloca_allocations
        .push(
            ::alloc::vec::from_elem(
                0,
                (((*dst).h + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<Uint32>()) as usize,
            ),
        );
    say = c2rust_alloca_allocations.last_mut().unwrap().as_mut_ptr()
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    if say.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    csx = 0 as ::core::ffi::c_int;
    csax = sax;
    x = 0 as ::core::ffi::c_int;
    while x <= (*dst).w {
        *csax = csx;
        csax = csax.offset(1);
        csx &= 0xffff as ::core::ffi::c_int;
        csx += sx;
        x += 1;
    }
    csy = 0 as ::core::ffi::c_int;
    csay = say;
    y = 0 as ::core::ffi::c_int;
    while y <= (*dst).h {
        *csay = csy;
        csay = csay.offset(1);
        csy &= 0xffff as ::core::ffi::c_int;
        csy += sy;
        y += 1;
    }
    csp = (*src).pixels as *mut tColorRGBA;
    sp = csp;
    dp = (*dst).pixels as *mut tColorRGBA;
    sgap = (*src).pitch - (*src).w * 4 as ::core::ffi::c_int;
    dgap = (*dst).pitch - (*dst).w * 4 as ::core::ffi::c_int;
    if smooth != 0 {
        csay = say;
        y = 0 as ::core::ffi::c_int;
        while y < (*dst).h {
            c00 = csp;
            c01 = csp;
            c01 = c01.offset(1);
            c10 = (csp as *mut Uint8).offset((*src).pitch as isize) as *mut tColorRGBA;
            c11 = c10;
            c11 = c11.offset(1);
            csax = sax;
            x = 0 as ::core::ffi::c_int;
            while x < (*dst).w {
                ex = *csax & 0xffff as ::core::ffi::c_int;
                ey = *csay & 0xffff as ::core::ffi::c_int;
                t1 = (((*c01).r as ::core::ffi::c_int - (*c00).r as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c00).r as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                t2 = (((*c11).r as ::core::ffi::c_int - (*c10).r as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c10).r as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                (*dp).r = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1) as Uint8;
                t1 = (((*c01).g as ::core::ffi::c_int - (*c00).g as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c00).g as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                t2 = (((*c11).g as ::core::ffi::c_int - (*c10).g as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c10).g as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                (*dp).g = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1) as Uint8;
                t1 = (((*c01).b as ::core::ffi::c_int - (*c00).b as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c00).b as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                t2 = (((*c11).b as ::core::ffi::c_int - (*c10).b as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c10).b as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                (*dp).b = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1) as Uint8;
                t1 = (((*c01).a as ::core::ffi::c_int - (*c00).a as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c00).a as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                t2 = (((*c11).a as ::core::ffi::c_int - (*c10).a as ::core::ffi::c_int)
                    * ex >> 16 as ::core::ffi::c_int) + (*c10).a as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                (*dp).a = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1) as Uint8;
                csax = csax.offset(1);
                sstep = *csax >> 16 as ::core::ffi::c_int;
                c00 = c00.offset(sstep as isize);
                c01 = c01.offset(sstep as isize);
                c10 = c10.offset(sstep as isize);
                c11 = c11.offset(sstep as isize);
                dp = dp.offset(1);
                x += 1;
            }
            csay = csay.offset(1);
            csp = (csp as *mut Uint8)
                .offset(((*csay >> 16 as ::core::ffi::c_int) * (*src).pitch) as isize)
                as *mut tColorRGBA;
            dp = (dp as *mut Uint8).offset(dgap as isize) as *mut tColorRGBA;
            y += 1;
        }
    } else {
        csay = say;
        y = 0 as ::core::ffi::c_int;
        while y < (*dst).h {
            sp = csp;
            csax = sax;
            x = 0 as ::core::ffi::c_int;
            while x < (*dst).w {
                *dp = *sp;
                csax = csax.offset(1);
                sp = sp.offset((*csax >> 16 as ::core::ffi::c_int) as isize);
                dp = dp.offset(1);
                x += 1;
            }
            csay = csay.offset(1);
            csp = (csp as *mut Uint8)
                .offset(((*csay >> 16 as ::core::ffi::c_int) * (*src).pitch) as isize)
                as *mut tColorRGBA;
            dp = (dp as *mut Uint8).offset(dgap as isize) as *mut tColorRGBA;
            y += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zoomSurfaceY(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
) -> ::core::ffi::c_int {
    let mut c2rust_alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut sx: Uint32 = 0;
    let mut sy: Uint32 = 0;
    let mut sax: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut say: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut csax: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut csay: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut csx: Uint32 = 0;
    let mut csy: Uint32 = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut sp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
    let mut dp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
    let mut csp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
    let mut dgap: ::core::ffi::c_int = 0;
    sx = (65536.0f64 * (*src).w as ::core::ffi::c_float as ::core::ffi::c_double
        / (*dst).w as ::core::ffi::c_float as ::core::ffi::c_double) as Uint32;
    sy = (65536.0f64 * (*src).h as ::core::ffi::c_float as ::core::ffi::c_double
        / (*dst).h as ::core::ffi::c_float as ::core::ffi::c_double) as Uint32;
    c2rust_alloca_allocations
        .push(
            ::alloc::vec::from_elem(
                0,
                ((*dst).w as usize).wrapping_mul(::core::mem::size_of::<Uint32>())
                    as usize,
            ),
        );
    sax = c2rust_alloca_allocations.last_mut().unwrap().as_mut_ptr()
        as *mut ::core::ffi::c_void as *mut Uint32;
    if sax.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    c2rust_alloca_allocations
        .push(
            ::alloc::vec::from_elem(
                0,
                ((*dst).h as usize).wrapping_mul(::core::mem::size_of::<Uint32>())
                    as usize,
            ),
        );
    say = c2rust_alloca_allocations.last_mut().unwrap().as_mut_ptr()
        as *mut ::core::ffi::c_void as *mut Uint32;
    if say.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    csx = 0 as Uint32;
    csax = sax;
    x = 0 as ::core::ffi::c_int;
    while x < (*dst).w {
        csx = csx.wrapping_add(sx);
        *csax = csx >> 16 as ::core::ffi::c_int;
        csx &= 0xffff as Uint32;
        csax = csax.offset(1);
        x += 1;
    }
    csy = 0 as Uint32;
    csay = say;
    y = 0 as ::core::ffi::c_int;
    while y < (*dst).h {
        csy = csy.wrapping_add(sy);
        *csay = csy >> 16 as ::core::ffi::c_int;
        csy &= 0xffff as Uint32;
        csay = csay.offset(1);
        y += 1;
    }
    csx = 0 as Uint32;
    csax = sax;
    x = 0 as ::core::ffi::c_int;
    while x < (*dst).w {
        csx = csx.wrapping_add(*csax);
        csax = csax.offset(1);
        x += 1;
    }
    csy = 0 as Uint32;
    csay = say;
    y = 0 as ::core::ffi::c_int;
    while y < (*dst).h {
        csy = csy.wrapping_add(*csay);
        csay = csay.offset(1);
        y += 1;
    }
    csp = (*src).pixels as *mut Uint8;
    sp = csp;
    dp = (*dst).pixels as *mut Uint8;
    dgap = (*dst).pitch - (*dst).w;
    csay = say;
    y = 0 as ::core::ffi::c_int;
    while y < (*dst).h {
        csax = sax;
        sp = csp;
        x = 0 as ::core::ffi::c_int;
        while x < (*dst).w {
            *dp = *sp;
            sp = sp.offset(*csax as isize);
            csax = csax.offset(1);
            dp = dp.offset(1);
            x += 1;
        }
        csp = csp.offset((*csay).wrapping_mul((*src).pitch as Uint32) as isize);
        csay = csay.offset(1);
        dp = dp.offset(dgap as isize);
        y += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn transformSurfaceRGBA(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut cx: ::core::ffi::c_int,
    mut cy: ::core::ffi::c_int,
    mut isin: ::core::ffi::c_int,
    mut icos: ::core::ffi::c_int,
    mut smooth: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut t1: ::core::ffi::c_int = 0;
    let mut t2: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut xd: ::core::ffi::c_int = 0;
    let mut yd: ::core::ffi::c_int = 0;
    let mut sdx: ::core::ffi::c_int = 0;
    let mut sdy: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut ex: ::core::ffi::c_int = 0;
    let mut ey: ::core::ffi::c_int = 0;
    let mut sw: ::core::ffi::c_int = 0;
    let mut sh: ::core::ffi::c_int = 0;
    let mut c00: tColorRGBA = tColorRGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    let mut c01: tColorRGBA = tColorRGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    let mut c10: tColorRGBA = tColorRGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    let mut c11: tColorRGBA = tColorRGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    let mut pc: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut sp: *mut tColorRGBA = ::core::ptr::null_mut::<tColorRGBA>();
    let mut gap: ::core::ffi::c_int = 0;
    xd = (*src).w - (*dst).w << 15 as ::core::ffi::c_int;
    yd = (*src).h - (*dst).h << 15 as ::core::ffi::c_int;
    ax = (cx << 16 as ::core::ffi::c_int) - icos * cx;
    ay = (cy << 16 as ::core::ffi::c_int) - isin * cx;
    sw = (*src).w - 1 as ::core::ffi::c_int;
    sh = (*src).h - 1 as ::core::ffi::c_int;
    pc = (*dst).pixels as *mut tColorRGBA;
    gap = (*dst).pitch - (*dst).w * 4 as ::core::ffi::c_int;
    if smooth != 0 {
        y = 0 as ::core::ffi::c_int;
        while y < (*dst).h {
            dy = cy - y;
            sdx = ax + isin * dy + xd;
            sdy = ay - icos * dy + yd;
            x = 0 as ::core::ffi::c_int;
            while x < (*dst).w {
                dx = sdx >> 16 as ::core::ffi::c_int;
                dy = sdy >> 16 as ::core::ffi::c_int;
                if dx >= -1 as ::core::ffi::c_int && dy >= -1 as ::core::ffi::c_int
                    && dx < (*src).w && dy < (*src).h
                {
                    if dx >= 0 as ::core::ffi::c_int && dy >= 0 as ::core::ffi::c_int
                        && dx < sw && dy < sh
                    {
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        sp = sp.offset(1 as ::core::ffi::c_int as isize);
                        c01 = *sp;
                        sp = (sp as *mut Uint8).offset((*src).pitch as isize)
                            as *mut tColorRGBA;
                        sp = sp.offset(-(1 as ::core::ffi::c_int as isize));
                        c10 = *sp;
                        sp = sp.offset(1 as ::core::ffi::c_int as isize);
                        c11 = *sp;
                    } else if dx == sw && dy == sh {
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        c11 = *sp;
                    } else if dx == -1 as ::core::ffi::c_int
                        && dy == -1 as ::core::ffi::c_int
                    {
                        sp = (*src).pixels as *mut tColorRGBA;
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        c11 = *sp;
                    } else if dx == -1 as ::core::ffi::c_int && dy == sh {
                        sp = (*src).pixels as *mut tColorRGBA;
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        c11 = *sp;
                    } else if dx == sw && dy == -1 as ::core::ffi::c_int {
                        sp = (*src).pixels as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        c11 = *sp;
                    } else if dx == -1 as ::core::ffi::c_int {
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        sp = (sp as *mut Uint8).offset((*src).pitch as isize)
                            as *mut tColorRGBA;
                        c11 = *sp;
                    } else if dy == -1 as ::core::ffi::c_int {
                        sp = (*src).pixels as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        c01 = *sp;
                        c10 = *sp;
                        sp = sp.offset(1 as ::core::ffi::c_int as isize);
                        c11 = *sp;
                    } else if dx == sw {
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        c01 = *sp;
                        sp = (sp as *mut Uint8).offset((*src).pitch as isize)
                            as *mut tColorRGBA;
                        c10 = *sp;
                        c11 = *sp;
                    } else if dy == sh {
                        sp = ((*src).pixels as *mut Uint8)
                            .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                        sp = sp.offset(dx as isize);
                        c00 = *sp;
                        sp = sp.offset(1 as ::core::ffi::c_int as isize);
                        c01 = *sp;
                        c10 = *sp;
                        c11 = *sp;
                    }
                    ex = sdx & 0xffff as ::core::ffi::c_int;
                    ey = sdy & 0xffff as ::core::ffi::c_int;
                    t1 = ((c01.r as ::core::ffi::c_int - c00.r as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c00.r as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    t2 = ((c11.r as ::core::ffi::c_int - c10.r as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c10.r as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    (*pc).r = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1)
                        as Uint8;
                    t1 = ((c01.g as ::core::ffi::c_int - c00.g as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c00.g as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    t2 = ((c11.g as ::core::ffi::c_int - c10.g as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c10.g as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    (*pc).g = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1)
                        as Uint8;
                    t1 = ((c01.b as ::core::ffi::c_int - c00.b as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c00.b as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    t2 = ((c11.b as ::core::ffi::c_int - c10.b as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c10.b as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    (*pc).b = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1)
                        as Uint8;
                    t1 = ((c01.a as ::core::ffi::c_int - c00.a as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c00.a as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    t2 = ((c11.a as ::core::ffi::c_int - c10.a as ::core::ffi::c_int)
                        * ex >> 16 as ::core::ffi::c_int) + c10.a as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    (*pc).a = (((t2 - t1) * ey >> 16 as ::core::ffi::c_int) + t1)
                        as Uint8;
                }
                sdx += icos;
                sdy += isin;
                pc = pc.offset(1);
                x += 1;
            }
            pc = (pc as *mut Uint8).offset(gap as isize) as *mut tColorRGBA;
            y += 1;
        }
    } else {
        y = 0 as ::core::ffi::c_int;
        while y < (*dst).h {
            dy = cy - y;
            sdx = ax + isin * dy + xd;
            sdy = ay - icos * dy + yd;
            x = 0 as ::core::ffi::c_int;
            while x < (*dst).w {
                dx = (sdx >> 16 as ::core::ffi::c_int) as ::core::ffi::c_short
                    as ::core::ffi::c_int;
                dy = (sdy >> 16 as ::core::ffi::c_int) as ::core::ffi::c_short
                    as ::core::ffi::c_int;
                if dx >= 0 as ::core::ffi::c_int && dy >= 0 as ::core::ffi::c_int
                    && dx < (*src).w && dy < (*src).h
                {
                    sp = ((*src).pixels as *mut Uint8)
                        .offset(((*src).pitch * dy) as isize) as *mut tColorRGBA;
                    sp = sp.offset(dx as isize);
                    *pc = *sp;
                }
                sdx += icos;
                sdy += isin;
                pc = pc.offset(1);
                x += 1;
            }
            pc = (pc as *mut Uint8).offset(gap as isize) as *mut tColorRGBA;
            y += 1;
        }
    };
}
unsafe extern "C" fn transformSurfaceY(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut cx: ::core::ffi::c_int,
    mut cy: ::core::ffi::c_int,
    mut isin: ::core::ffi::c_int,
    mut icos: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut xd: ::core::ffi::c_int = 0;
    let mut yd: ::core::ffi::c_int = 0;
    let mut sdx: ::core::ffi::c_int = 0;
    let mut sdy: ::core::ffi::c_int = 0;
    let mut ax: ::core::ffi::c_int = 0;
    let mut ay: ::core::ffi::c_int = 0;
    let mut sw: ::core::ffi::c_int = 0;
    let mut sh: ::core::ffi::c_int = 0;
    let mut pc: *mut tColorY = ::core::ptr::null_mut::<tColorY>();
    let mut sp: *mut tColorY = ::core::ptr::null_mut::<tColorY>();
    let mut gap: ::core::ffi::c_int = 0;
    let mut colorkey: Uint32 = 0 as Uint32;
    xd = (*src).w - (*dst).w << 15 as ::core::ffi::c_int;
    yd = (*src).h - (*dst).h << 15 as ::core::ffi::c_int;
    ax = (cx << 16 as ::core::ffi::c_int) - icos * cx;
    ay = (cy << 16 as ::core::ffi::c_int) - isin * cx;
    sw = (*src).w - 1 as ::core::ffi::c_int;
    sh = (*src).h - 1 as ::core::ffi::c_int;
    pc = (*dst).pixels as *mut tColorY;
    gap = (*dst).pitch - (*dst).w;
    TFB_GetColorKey(src, &raw mut colorkey);
    memset(
        pc as *mut ::core::ffi::c_void,
        (colorkey & 0xff as Uint32) as ::core::ffi::c_uchar as ::core::ffi::c_int,
        ((*dst).pitch * (*dst).h) as size_t,
    );
    y = 0 as ::core::ffi::c_int;
    while y < (*dst).h {
        dy = cy - y;
        sdx = ax + isin * dy + xd;
        sdy = ay - icos * dy + yd;
        x = 0 as ::core::ffi::c_int;
        while x < (*dst).w {
            dx = (sdx >> 16 as ::core::ffi::c_int) as ::core::ffi::c_short
                as ::core::ffi::c_int;
            dy = (sdy >> 16 as ::core::ffi::c_int) as ::core::ffi::c_short
                as ::core::ffi::c_int;
            if dx >= 0 as ::core::ffi::c_int && dy >= 0 as ::core::ffi::c_int
                && dx < (*src).w && dy < (*src).h
            {
                sp = (*src).pixels as *mut tColorY;
                sp = sp.offset(((*src).pitch * dy + dx) as isize);
                *pc = *sp;
            }
            sdx += icos;
            sdy += isin;
            pc = pc.offset(1);
            x += 1;
        }
        pc = pc.offset(gap as isize);
        y += 1;
    }
}
unsafe extern "C" fn rotozoomSurfaceSizeTrig(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut angle: ::core::ffi::c_double,
    mut zoom: ::core::ffi::c_double,
    mut dstwidth: *mut ::core::ffi::c_int,
    mut dstheight: *mut ::core::ffi::c_int,
    mut canglezoom: *mut ::core::ffi::c_double,
    mut sanglezoom: *mut ::core::ffi::c_double,
) {
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut cx: ::core::ffi::c_double = 0.;
    let mut cy: ::core::ffi::c_double = 0.;
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut radangle: ::core::ffi::c_double = 0.;
    let mut dstwidthhalf: ::core::ffi::c_int = 0;
    let mut dstheighthalf: ::core::ffi::c_int = 0;
    radangle = angle * (3.14159265358979323846f64 / 180.0f64);
    *sanglezoom = sin(radangle);
    *canglezoom = cos(radangle);
    *sanglezoom *= zoom;
    *canglezoom *= zoom;
    x = (width / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
    y = (height / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
    cx = *canglezoom * x;
    cy = *canglezoom * y;
    sx = *sanglezoom * x;
    sy = *sanglezoom * y;
    dstwidthhalf = (if ceil(fabs(cx) + fabs(sy))
        > 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        ceil(fabs(cx) + fabs(sy))
    } else {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    dstheighthalf = (if ceil(fabs(sx) + fabs(cy))
        > 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        ceil(fabs(sx) + fabs(cy))
    } else {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    *dstwidth = 2 as ::core::ffi::c_int * dstwidthhalf;
    *dstheight = 2 as ::core::ffi::c_int * dstheighthalf;
}
#[no_mangle]
pub unsafe extern "C" fn rotozoomSurfaceSize(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut angle: ::core::ffi::c_double,
    mut zoom: ::core::ffi::c_double,
    mut dstwidth: *mut ::core::ffi::c_int,
    mut dstheight: *mut ::core::ffi::c_int,
) {
    let mut dummy_sanglezoom: ::core::ffi::c_double = 0.;
    let mut dummy_canglezoom: ::core::ffi::c_double = 0.;
    rotozoomSurfaceSizeTrig(
        width,
        height,
        angle,
        zoom,
        dstwidth,
        dstheight,
        &raw mut dummy_sanglezoom,
        &raw mut dummy_canglezoom,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rotozoomSurface(
    mut src: *mut SDL_Surface,
    mut angle: ::core::ffi::c_double,
    mut zoom: ::core::ffi::c_double,
    mut smooth: ::core::ffi::c_int,
) -> *mut SDL_Surface {
    let mut rz_src: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut rz_dst: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut zoominv: ::core::ffi::c_double = 0.;
    let mut sanglezoom: ::core::ffi::c_double = 0.;
    let mut canglezoom: ::core::ffi::c_double = 0.;
    let mut sanglezoominv: ::core::ffi::c_double = 0.;
    let mut canglezoominv: ::core::ffi::c_double = 0.;
    let mut dstwidthhalf: ::core::ffi::c_int = 0;
    let mut dstwidth: ::core::ffi::c_int = 0;
    let mut dstheighthalf: ::core::ffi::c_int = 0;
    let mut dstheight: ::core::ffi::c_int = 0;
    let mut is32bit: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut src_converted: ::core::ffi::c_int = 0;
    if src.is_null() {
        return ::core::ptr::null_mut::<SDL_Surface>();
    }
    is32bit = ((*(*src).format).BitsPerPixel as ::core::ffi::c_int
        == 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if is32bit != 0
        || (*(*src).format).BitsPerPixel as ::core::ffi::c_int == 8 as ::core::ffi::c_int
    {
        rz_src = src;
        src_converted = 0 as ::core::ffi::c_int;
    } else {
        rz_src = SDL_CreateRGBSurface(
            0 as Uint32,
            (*src).w,
            (*src).h,
            32 as ::core::ffi::c_int,
            0xff as Uint32,
            0xff00 as Uint32,
            0xff0000 as ::core::ffi::c_int as Uint32,
            0xff000000 as Uint32,
        );
        SDL_UpperBlit(
            src,
            ::core::ptr::null::<SDL_Rect>(),
            rz_src,
            ::core::ptr::null_mut::<SDL_Rect>(),
        );
        src_converted = 1 as ::core::ffi::c_int;
        is32bit = 1 as ::core::ffi::c_int;
    }
    if zoom < 0.001f64 {
        zoom = 0.001f64;
    }
    zoominv = 65536.0f64 / (zoom * zoom);
    if fabs(angle) > 0.001f64 {
        rotozoomSurfaceSizeTrig(
            (*rz_src).w,
            (*rz_src).h,
            angle,
            zoom,
            &raw mut dstwidth,
            &raw mut dstheight,
            &raw mut canglezoom,
            &raw mut sanglezoom,
        );
        sanglezoominv = sanglezoom;
        canglezoominv = canglezoom;
        sanglezoominv *= zoominv;
        canglezoominv *= zoominv;
        dstwidthhalf = dstwidth / 2 as ::core::ffi::c_int;
        dstheighthalf = dstheight / 2 as ::core::ffi::c_int;
        rz_dst = ::core::ptr::null_mut::<SDL_Surface>();
        if is32bit != 0 {
            rz_dst = SDL_CreateRGBSurface(
                0 as Uint32,
                dstwidth,
                dstheight,
                32 as ::core::ffi::c_int,
                (*(*rz_src).format).Rmask,
                (*(*rz_src).format).Gmask,
                (*(*rz_src).format).Bmask,
                (*(*rz_src).format).Amask,
            );
        } else {
            rz_dst = SDL_CreateRGBSurface(
                0 as Uint32,
                dstwidth,
                dstheight,
                8 as ::core::ffi::c_int,
                0 as Uint32,
                0 as Uint32,
                0 as Uint32,
                0 as Uint32,
            );
        }
        SDL_LockSurface(rz_src);
        if is32bit != 0 {
            transformSurfaceRGBA(
                rz_src,
                rz_dst,
                dstwidthhalf,
                dstheighthalf,
                sanglezoominv as ::core::ffi::c_int,
                canglezoominv as ::core::ffi::c_int,
                smooth,
            );
            TFB_SetSurfaceAlphaMod(rz_dst, 255 as Uint8);
        } else {
            let mut srckey: Uint32 = 0 as Uint32;
            TFB_GetColorKey(rz_src, &raw mut srckey);
            i = 0 as ::core::ffi::c_int;
            while i < (*(*(*rz_src).format).palette).ncolors {
                *(*(*(*rz_dst).format).palette).colors.offset(i as isize) = *(*(*(*rz_src)
                    .format)
                    .palette)
                    .colors
                    .offset(i as isize);
                i += 1;
            }
            (*(*(*rz_dst).format).palette).ncolors = (*(*(*rz_src).format).palette)
                .ncolors;
            transformSurfaceY(
                rz_src,
                rz_dst,
                dstwidthhalf,
                dstheighthalf,
                sanglezoominv as ::core::ffi::c_int,
                canglezoominv as ::core::ffi::c_int,
            );
            TFB_SetColorKey(rz_dst, srckey, 1 as ::core::ffi::c_int);
        }
        SDL_UnlockSurface(rz_src);
    } else {
        zoomSurfaceSize(
            (*rz_src).w,
            (*rz_src).h,
            zoom,
            zoom,
            &raw mut dstwidth,
            &raw mut dstheight,
        );
        rz_dst = ::core::ptr::null_mut::<SDL_Surface>();
        if is32bit != 0 {
            rz_dst = SDL_CreateRGBSurface(
                0 as Uint32,
                dstwidth,
                dstheight,
                32 as ::core::ffi::c_int,
                (*(*rz_src).format).Rmask,
                (*(*rz_src).format).Gmask,
                (*(*rz_src).format).Bmask,
                (*(*rz_src).format).Amask,
            );
        } else {
            rz_dst = SDL_CreateRGBSurface(
                0 as Uint32,
                dstwidth,
                dstheight,
                8 as ::core::ffi::c_int,
                0 as Uint32,
                0 as Uint32,
                0 as Uint32,
                0 as Uint32,
            );
        }
        SDL_LockSurface(rz_src);
        if is32bit != 0 {
            zoomSurfaceRGBA(rz_src, rz_dst, smooth);
            TFB_SetSurfaceAlphaMod(rz_dst, 255 as Uint8);
        } else {
            let mut srckey_0: Uint32 = 0 as Uint32;
            TFB_GetColorKey(rz_src, &raw mut srckey_0);
            i = 0 as ::core::ffi::c_int;
            while i < (*(*(*rz_src).format).palette).ncolors {
                *(*(*(*rz_dst).format).palette).colors.offset(i as isize) = *(*(*(*rz_src)
                    .format)
                    .palette)
                    .colors
                    .offset(i as isize);
                i += 1;
            }
            (*(*(*rz_dst).format).palette).ncolors = (*(*(*rz_src).format).palette)
                .ncolors;
            zoomSurfaceY(rz_src, rz_dst);
            TFB_SetColorKey(rz_dst, srckey_0, 1 as ::core::ffi::c_int);
        }
        SDL_UnlockSurface(rz_src);
    }
    if src_converted != 0 {
        SDL_FreeSurface(rz_src);
    }
    return rz_dst;
}
#[no_mangle]
pub unsafe extern "C" fn rotateSurface(
    mut src: *mut SDL_Surface,
    mut dst: *mut SDL_Surface,
    mut angle: ::core::ffi::c_double,
    mut smooth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut zoominv: ::core::ffi::c_double = 0.;
    let mut sanglezoom: ::core::ffi::c_double = 0.;
    let mut canglezoom: ::core::ffi::c_double = 0.;
    let mut sanglezoominv: ::core::ffi::c_double = 0.;
    let mut canglezoominv: ::core::ffi::c_double = 0.;
    let mut dstwidthhalf: ::core::ffi::c_int = 0;
    let mut dstwidth: ::core::ffi::c_int = 0;
    let mut dstheighthalf: ::core::ffi::c_int = 0;
    let mut dstheight: ::core::ffi::c_int = 0;
    let mut is32bit: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if src.is_null() || dst.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*(*src).format).BitsPerPixel as ::core::ffi::c_int
        != (*(*dst).format).BitsPerPixel as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    is32bit = ((*(*src).format).BitsPerPixel as ::core::ffi::c_int
        == 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
    zoominv = 65536.0f64;
    if fabs(angle) <= 0.001f64 {
        SDL_UpperBlit(
            src,
            ::core::ptr::null::<SDL_Rect>(),
            dst,
            ::core::ptr::null_mut::<SDL_Rect>(),
        );
        return 0 as ::core::ffi::c_int;
    }
    rotozoomSurfaceSizeTrig(
        (*src).w,
        (*src).h,
        angle,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        &raw mut dstwidth,
        &raw mut dstheight,
        &raw mut canglezoom,
        &raw mut sanglezoom,
    );
    sanglezoominv = sanglezoom;
    canglezoominv = canglezoom;
    sanglezoominv *= zoominv;
    canglezoominv *= zoominv;
    dstwidthhalf = dstwidth / 2 as ::core::ffi::c_int;
    dstheighthalf = dstheight / 2 as ::core::ffi::c_int;
    if (*dst).w < dstwidth || (*dst).h < dstheight {
        return -1 as ::core::ffi::c_int;
    }
    SDL_LockSurface(src);
    SDL_LockSurface(dst);
    if is32bit != 0 {
        transformSurfaceRGBA(
            src,
            dst,
            dstwidthhalf,
            dstheighthalf,
            sanglezoominv as ::core::ffi::c_int,
            canglezoominv as ::core::ffi::c_int,
            smooth,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*(*(*src).format).palette).ncolors {
            *(*(*(*dst).format).palette).colors.offset(i as isize) = *(*(*(*src).format)
                .palette)
                .colors
                .offset(i as isize);
            i += 1;
        }
        (*(*(*dst).format).palette).ncolors = (*(*(*src).format).palette).ncolors;
        transformSurfaceY(
            src,
            dst,
            dstwidthhalf,
            dstheighthalf,
            sanglezoominv as ::core::ffi::c_int,
            canglezoominv as ::core::ffi::c_int,
        );
    }
    SDL_UnlockSurface(dst);
    SDL_UnlockSurface(src);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zoomSurfaceSize(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut zoomx: ::core::ffi::c_double,
    mut zoomy: ::core::ffi::c_double,
    mut dstwidth: *mut ::core::ffi::c_int,
    mut dstheight: *mut ::core::ffi::c_int,
) {
    if zoomx < 0.001f64 {
        zoomx = 0.001f64;
    }
    if zoomy < 0.001f64 {
        zoomy = 0.001f64;
    }
    *dstwidth = (width as ::core::ffi::c_double * zoomx) as ::core::ffi::c_int;
    *dstheight = (height as ::core::ffi::c_double * zoomy) as ::core::ffi::c_int;
    if *dstwidth < 1 as ::core::ffi::c_int {
        *dstwidth = 1 as ::core::ffi::c_int;
    }
    if *dstheight < 1 as ::core::ffi::c_int {
        *dstheight = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zoomSurface(
    mut src: *mut SDL_Surface,
    mut zoomx: ::core::ffi::c_double,
    mut zoomy: ::core::ffi::c_double,
    mut smooth: ::core::ffi::c_int,
) -> *mut SDL_Surface {
    let mut rz_src: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut rz_dst: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut dstwidth: ::core::ffi::c_int = 0;
    let mut dstheight: ::core::ffi::c_int = 0;
    let mut is32bit: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut src_converted: ::core::ffi::c_int = 0;
    if src.is_null() {
        return ::core::ptr::null_mut::<SDL_Surface>();
    }
    is32bit = ((*(*src).format).BitsPerPixel as ::core::ffi::c_int
        == 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if is32bit != 0
        || (*(*src).format).BitsPerPixel as ::core::ffi::c_int == 8 as ::core::ffi::c_int
    {
        rz_src = src;
        src_converted = 0 as ::core::ffi::c_int;
    } else {
        rz_src = SDL_CreateRGBSurface(
            0 as Uint32,
            (*src).w,
            (*src).h,
            32 as ::core::ffi::c_int,
            0xff as Uint32,
            0xff00 as Uint32,
            0xff0000 as ::core::ffi::c_int as Uint32,
            0xff000000 as Uint32,
        );
        SDL_UpperBlit(
            src,
            ::core::ptr::null::<SDL_Rect>(),
            rz_src,
            ::core::ptr::null_mut::<SDL_Rect>(),
        );
        src_converted = 1 as ::core::ffi::c_int;
        is32bit = 1 as ::core::ffi::c_int;
    }
    zoomSurfaceSize(
        (*rz_src).w,
        (*rz_src).h,
        zoomx,
        zoomy,
        &raw mut dstwidth,
        &raw mut dstheight,
    );
    rz_dst = ::core::ptr::null_mut::<SDL_Surface>();
    if is32bit != 0 {
        rz_dst = SDL_CreateRGBSurface(
            0 as Uint32,
            dstwidth,
            dstheight,
            32 as ::core::ffi::c_int,
            (*(*rz_src).format).Rmask,
            (*(*rz_src).format).Gmask,
            (*(*rz_src).format).Bmask,
            (*(*rz_src).format).Amask,
        );
    } else {
        rz_dst = SDL_CreateRGBSurface(
            0 as Uint32,
            dstwidth,
            dstheight,
            8 as ::core::ffi::c_int,
            0 as Uint32,
            0 as Uint32,
            0 as Uint32,
            0 as Uint32,
        );
    }
    SDL_LockSurface(rz_src);
    if is32bit != 0 {
        zoomSurfaceRGBA(rz_src, rz_dst, smooth);
        TFB_SetSurfaceAlphaMod(rz_dst, 255 as Uint8);
    } else {
        let mut srckey: Uint32 = 0 as Uint32;
        TFB_GetColorKey(rz_src, &raw mut srckey);
        i = 0 as ::core::ffi::c_int;
        while i < (*(*(*rz_src).format).palette).ncolors {
            *(*(*(*rz_dst).format).palette).colors.offset(i as isize) = *(*(*(*rz_src)
                .format)
                .palette)
                .colors
                .offset(i as isize);
            i += 1;
        }
        (*(*(*rz_dst).format).palette).ncolors = (*(*(*rz_src).format).palette).ncolors;
        zoomSurfaceY(rz_src, rz_dst);
        TFB_SetColorKey(rz_dst, srckey, 0 as ::core::ffi::c_int);
    }
    SDL_UnlockSurface(rz_src);
    if src_converted != 0 {
        SDL_FreeSurface(rz_src);
    }
    return rz_dst;
}
