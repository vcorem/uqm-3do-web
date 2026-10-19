// SPDX-License-Identifier: GPL-2.0-or-later
// Rust engine integration support.
#![no_std]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]

#[macro_use]
extern crate c2rust_bitfields;

#[cfg(not(target_os = "emscripten"))]
compile_error!("Build this engine for wasm32-unknown-emscripten using scripts/rust-engine/build.sh.");
extern crate uqm_core;
extern crate alloc;
mod uqm_allocator;
mod uqm_engine_init;

#[macro_use]
#[path = "../uqm-lua/guard.rs"]
mod uqm_lua_guard;

pub mod src {
#[path = "uqm.rs"]
pub mod uqm_1;
pub mod libs {
pub mod callback {
pub mod alarm;
pub mod callback;
pub mod r#async;
} // mod callback
pub mod decomp {
pub mod lzdecode;
pub mod lzencode;
pub mod update;
} // mod decomp
pub mod file {
pub mod dirs;
pub mod files;
} // mod file
pub mod graphics {
// bbox: supplied by the independently tested uqm-core crate.
// boxint: supplied by the independently tested uqm-core crate.
// clipline: supplied by the independently tested uqm-core crate.
pub mod cmap;
pub mod context;
pub mod dcqueue;
pub mod drawable;
pub mod filegfx;
pub mod font;
pub mod frame;
pub mod gfx_common;
pub mod gfxload;
pub mod intersec;
pub mod loaddisp;
pub mod pixmap;
pub mod resgfx;
pub mod sdl {
#[path = "2xscalers.rs"]
pub mod module_2xscalers;
#[path = "2xscalers_3dnow.rs"]
pub mod module_2xscalers_3dnow;
#[path = "2xscalers_mmx.rs"]
pub mod module_2xscalers_mmx;
#[path = "2xscalers_sse.rs"]
pub mod module_2xscalers_sse;
pub mod biadv2x;
pub mod bilinear2x;
pub mod canvas;
pub mod clipboard;
pub mod hq2x;
pub mod nearest2x;
pub mod palette;
// Shared Rust PNG orchestration with guarded C library calls.
pub mod png2sdl;
pub mod primitives;
pub mod rotozoom;
pub mod scalers;
pub mod sdl2_common;
pub mod sdl2_pure;
pub mod sdl_common;
pub mod sdluio;
pub mod triscan2x;
} // mod sdl
pub mod tfb_draw;
pub mod tfb_prim;
pub mod widgets;
} // mod graphics
pub mod heap {
pub mod heap;
} // mod heap
pub mod input {
pub mod input_common;
pub mod sdl {
pub mod input;
pub mod keynames;
pub mod vcontrol;
} // mod sdl
} // mod input
pub mod list {
pub mod list;
} // mod list
pub mod log {
pub mod msgbox_stub;
pub mod uqmlog;
} // mod log
pub mod luauqm {
pub mod luauqm;
pub mod scriptres;
} // mod luauqm
pub mod math {
// random: supplied by the independently tested uqm-core crate.
// random2: supplied by the independently tested uqm-core crate.
// sqrt: supplied by the independently tested uqm-core crate.
} // mod math
pub mod memory {
pub mod w_memlib;
} // mod memory
pub mod mikmod {
pub mod drv_nos;
pub mod load_it;
pub mod load_mod;
pub mod load_s3m;
pub mod load_stm;
pub mod load_xm;
pub mod mdreg;
pub mod mdriver;
pub mod mdulaw;
pub mod mloader;
pub mod mlreg;
pub mod mlutil;
pub mod mmalloc;
pub mod mmerror;
pub mod mmio;
pub mod mplayer;
pub mod munitrk;
pub mod mwav;
pub mod npertab;
pub mod sloader;
pub mod virtch;
pub mod virtch2;
pub mod virtch_common;
} // mod mikmod
pub mod resource {
pub mod direct;
pub mod filecntl;
pub mod getres;
pub mod loadres;
pub mod propfile;
pub mod resinit;
pub mod stringbank;
} // mod resource
pub mod sound {
pub mod audiocore;
pub mod decoders {
pub mod aiffaud;
pub mod decoder;
pub mod dukaud;
pub mod modaud;
pub mod oggaud;
pub mod wav;
} // mod decoders
pub mod fileinst;
pub mod mixer {
pub mod mixer;
pub mod nosound {
pub mod audiodrv_nosound;
} // mod nosound
pub mod sdl {
pub mod audiodrv_sdl;
} // mod sdl
} // mod mixer
pub mod music;
pub mod openal {
pub mod audiodrv_openal;
} // mod openal
pub mod resinst;
pub mod sfx;
pub mod sound;
pub mod stream;
pub mod trackplayer;
} // mod sound
pub mod strings {
pub mod getstr;
pub mod sfileins;
pub mod sresins;
pub mod stringhashtable;
pub mod strings;
pub mod unicode;
} // mod strings
pub mod task {
pub mod tasklib;
} // mod task
pub mod threads {
pub mod pthread {
pub mod posixthreads;
} // mod pthread
pub mod thrcommon;
} // mod threads
pub mod time {
pub mod sdl {
pub mod sdltime;
} // mod sdl
pub mod timecommon;
} // mod time
pub mod uio {
pub mod charhashtable;
pub mod debug;
pub mod defaultfs;
pub mod fileblock;
pub mod fstypes;
pub mod gphys;
pub mod io;
pub mod ioaux;
pub mod mount;
pub mod mounttree;
pub mod paths;
pub mod physical;
pub mod r#match;
pub mod stdio {
pub mod stdio;
} // mod stdio
pub mod uiostream;
pub mod uioutils;
pub mod utils;
pub mod zip {
pub mod zip;
} // mod zip
} // mod uio
pub mod video {
pub mod dukvid;
pub mod legacyplayer;
pub mod vfileins;
pub mod video;
pub mod videodec;
pub mod vidplayer;
pub mod vresins;
} // mod video
} // mod libs
pub mod options;
pub mod port;
pub mod uqm {
#[path = "comm.rs"]
pub mod comm_1;
pub mod battle;
pub mod battlecontrols;
pub mod border;
pub mod build;
pub mod cleanup;
pub mod clock;
pub mod cnctdlg;
pub mod collide;
pub mod comm {
pub mod arilou {
pub mod arilouc;
} // mod arilou
pub mod blackur {
pub mod blackurc;
} // mod blackur
pub mod chmmr {
pub mod chmmrc;
} // mod chmmr
pub mod comandr {
pub mod comandr;
} // mod comandr
pub mod druuge {
pub mod druugec;
} // mod druuge
pub mod ilwrath {
pub mod ilwrathc;
} // mod ilwrath
pub mod melnorm {
pub mod melnorm;
} // mod melnorm
pub mod mycon {
pub mod myconc;
} // mod mycon
pub mod orz {
pub mod orzc;
} // mod orz
pub mod pkunk {
pub mod pkunkc;
} // mod pkunk
pub mod rebel {
pub mod rebel;
} // mod rebel
pub mod shofixt {
pub mod shofixt;
} // mod shofixt
pub mod slyhome {
pub mod slyhome;
} // mod slyhome
pub mod slyland {
pub mod slyland;
} // mod slyland
pub mod spahome {
pub mod spahome;
} // mod spahome
pub mod spathi {
pub mod spathic;
} // mod spathi
pub mod starbas {
pub mod starbas;
} // mod starbas
pub mod supox {
pub mod supoxc;
} // mod supox
pub mod syreen {
pub mod syreenc;
} // mod syreen
pub mod talkpet {
pub mod talkpet;
} // mod talkpet
pub mod thradd {
pub mod thraddc;
} // mod thradd
pub mod umgah {
pub mod umgahc;
} // mod umgah
pub mod urquan {
pub mod urquanc;
} // mod urquan
pub mod utwig {
pub mod utwigc;
} // mod utwig
pub mod vux {
pub mod vuxc;
} // mod vux
pub mod yehat {
pub mod yehatc;
} // mod yehat
pub mod zoqfot {
pub mod zoqfotc;
} // mod zoqfot
} // mod comm
pub mod commanim;
pub mod commglue;
pub mod confirm;
pub mod cons_res;
pub mod credits;
pub mod cyborg;
pub mod demo;
// displist: supplied by the independently tested uqm-core crate.
pub mod dummy;
pub mod encount;
pub mod flash;
pub mod fmv;
pub mod galaxy;
pub mod gameev;
pub mod gameinp;
pub mod gameopt;
pub mod gendef;
pub mod getchar;
pub mod globdata;
pub mod gravity;
pub mod grpinfo;
pub mod hyper;
pub mod init;
pub mod intel;
pub mod intro;
pub mod ipdisp;
pub mod load;
pub mod load_legacy;
pub mod loadship;
pub mod lua {
pub mod luacomm;
pub mod luadebug;
pub mod luaevent;
pub mod luafuncs {
pub mod commfuncs;
pub mod customfuncs;
pub mod eventfuncs;
pub mod logfuncs;
pub mod statefuncs;
} // mod luafuncs
pub mod luainit;
pub mod luastate;
} // mod lua
pub mod master;
pub mod menu;
pub mod misc;
pub mod oscill;
pub mod outfit;
pub mod pickship;
pub mod plandata;
pub mod planets {
pub mod calc;
pub mod cargo;
pub mod devices;
pub mod generate {
pub mod genand;
pub mod genburv;
pub mod genchmmr;
pub mod gencol;
pub mod gendefault;
pub mod gendru;
pub mod genilw;
pub mod genmel;
pub mod genmyc;
pub mod genorz;
pub mod genpet;
pub mod genpku;
pub mod genrain;
pub mod gensam;
pub mod genshof;
pub mod gensly;
pub mod gensol;
pub mod genspa;
pub mod gensup;
pub mod gensyr;
pub mod genthrad;
pub mod gentrap;
pub mod genutw;
pub mod genvault;
pub mod genvux;
pub mod genwreck;
pub mod genyeh;
pub mod genzfpscout;
pub mod genzoq;
} // mod generate
pub mod gentopo;
pub mod lander;
pub mod orbits;
pub mod oval;
pub mod pl_stuff;
pub mod planets;
pub mod plangen;
pub mod pstarmap;
pub mod report;
pub mod roster;
pub mod scan;
pub mod solarsys;
pub mod surface;
} // mod planets
pub mod process;
pub mod restart;
pub mod save;
pub mod settings;
pub mod setup;
pub mod setupmenu;
pub mod ship;
pub mod ships {
pub mod androsyn {
pub mod androsyn;
} // mod androsyn
pub mod arilou {
pub mod arilou;
} // mod arilou
pub mod blackurq {
pub mod blackurq;
} // mod blackurq
pub mod chenjesu {
pub mod chenjesu;
} // mod chenjesu
pub mod chmmr {
pub mod chmmr;
} // mod chmmr
pub mod druuge {
pub mod druuge;
} // mod druuge
pub mod human {
pub mod human;
} // mod human
pub mod ilwrath {
pub mod ilwrath;
} // mod ilwrath
pub mod lastbat {
pub mod lastbat;
} // mod lastbat
pub mod melnorme {
pub mod melnorme;
} // mod melnorme
pub mod mmrnmhrm {
pub mod mmrnmhrm;
} // mod mmrnmhrm
pub mod mycon {
pub mod mycon;
} // mod mycon
pub mod orz {
pub mod orz;
} // mod orz
pub mod pkunk {
pub mod pkunk;
} // mod pkunk
pub mod probe {
pub mod probe;
} // mod probe
pub mod shofixti {
pub mod shofixti;
} // mod shofixti
pub mod sis_ship {
pub mod sis_ship;
} // mod sis_ship
pub mod slylandr {
pub mod slylandr;
} // mod slylandr
pub mod spathi {
pub mod spathi;
} // mod spathi
pub mod supox {
pub mod supox;
} // mod supox
pub mod syreen {
pub mod syreen;
} // mod syreen
pub mod thradd {
pub mod thradd;
} // mod thradd
pub mod umgah {
pub mod umgah;
} // mod umgah
pub mod urquan {
pub mod urquan;
} // mod urquan
pub mod utwig {
pub mod utwig;
} // mod utwig
pub mod vux {
pub mod vux;
} // mod vux
pub mod yehat {
pub mod yehat;
} // mod yehat
pub mod zoqfot {
pub mod zoqfot;
} // mod zoqfot
} // mod ships
pub mod shipstat;
pub mod shipyard;
pub mod sis;
pub mod sounds;
pub mod starbase;
pub mod starcon;
pub mod starmap;
pub mod state;
pub mod status;
pub mod supermelee {
pub mod buildpick;
pub mod loadmele;
pub mod melee;
pub mod meleesetup;
pub mod netplay {
pub mod checkbuf;
pub mod checksum;
// crc: supplied by the independently tested uqm-core crate.
pub mod netconnection;
pub mod netinput;
pub mod netmelee;
pub mod netmisc;
pub mod netoptions;
pub mod netrcv;
pub mod netsend;
pub mod netstate;
pub mod notify;
pub mod notifyall;
pub mod packet;
pub mod packethandlers;
pub mod packetq;
pub mod packetsenders;
pub mod proto {
pub mod npconfirm;
pub mod ready;
pub mod reset;
} // mod proto
} // mod netplay
pub mod pickmele;
} // mod supermelee
pub mod tactrans;
// trans: supplied by the independently tested uqm-core crate.
pub mod uqmdebug;
pub mod util;
// velocity: supplied by the independently tested uqm-core crate.
pub mod weapon;
} // mod uqm
} // mod src


mod hd_rng_config;
