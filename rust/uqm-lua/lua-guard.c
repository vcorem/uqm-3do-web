/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Lua 5.2 FFI error boundary. Never put a Rust call beneath a C setjmp
 * unless that Rust call has its own local C guards for every throwing API.
 * The pinned Lua runtime is external C; all game logic remains in Rust. */
#include "lua-guard.h"
#include "lualib.h"
#include "ldo.h"
#include "lstate.h"
#include "lobject.h"
#include <stdarg.h>
#include <stdlib.h>

struct rust_frame {
    struct rust_frame *previous;
    lua_State *state;
    int status;
};
static _Thread_local struct rust_frame *current_frame;
int uqm_lua_failed(void) { return current_frame && current_frame->status; }

static void guarded(lua_State *L, Pfunc operation, void *arguments) {
    if (uqm_lua_failed()) return;
    /* This is an FFI boundary, not a Lua-level pcall: preserve an inherited
     * xpcall handler. It runs inside this local C protection; any Rust
     * callback it invokes receives its own nested trampoline frame. */
    int status = luaD_pcall(L, operation, arguments, savestack(L, L->top), L->errfunc);
    if (status == LUA_OK) return;
    /* There is no Lua caller to propagate into outside a callback. Preserve
     * Lua's fail-fast unprotected-error behavior, without jumping over Rust. */
    if (!current_frame || current_frame->state != L) abort();
    current_frame->status = status;
}

static int rust_callback(lua_State *L) {
    CClosure *closure = clCvalue(L->ci->func);
    lua_CFunction *function = (lua_CFunction *)(
        rawuvalue(&closure->upvalue[closure->nupvalues - 1]) + 1);
    struct rust_frame frame = {current_frame, L, LUA_OK};
    current_frame = &frame;
    int result = (*function)(L);
    current_frame = frame.previous;
    /* Every Rust frame has returned before Lua's non-local error transfer. */
    if (frame.status != LUA_OK) luaD_throw(L, frame.status);
    return result;
}

static int native_opener(lua_CFunction fn) {
    return fn == luaopen_base || fn == luaopen_table || fn == luaopen_string ||
           fn == luaopen_bit32 || fn == luaopen_math;
}
static void guarded_closure(lua_State *L, lua_CFunction fn, int n) {
    if (native_opener(fn)) { lua_pushcclosure(L, fn, n); return; }
    if (!fn || n < 0 || n >= 255) abort();
    luaL_checkstack(L, 1, "Rust callback upvalue");
    lua_CFunction *slot = (lua_CFunction *)lua_newuserdata(L, sizeof(*slot));
    *slot = fn;
    lua_pushcclosure(L, rust_callback, n + 1);
}
static void guarded_setfuncs(lua_State *L, const luaL_Reg *l, int nup) {
    luaL_checkstack(L, nup + 1, "Rust callback upvalues");
    for (; l->name != NULL; l++) {
        int i;
        for (i = 0; i < nup; i++) lua_pushvalue(L, -nup);
        guarded_closure(L, l->func, nup);
        lua_setfield(L, -(nup + 2), l->name);
    }
    lua_pop(L, nup);
}
struct reader_data { lua_Reader reader; void *data; };
static const char *rust_reader(lua_State *L, void *ud, size_t *size) {
    struct reader_data *data = ud;
    struct rust_frame frame = {current_frame, L, LUA_OK};
    current_frame = &frame;
    const char *result = data->reader(L, data->data, size);
    current_frame = frame.previous;
    if (frame.status != LUA_OK) luaD_throw(L, frame.status);
    return result;
}

struct args_lua_absindex { int idx; int result; };
static void op_lua_absindex(lua_State *L, void *ud) {
    struct args_lua_absindex *a = ud;
    a->result = lua_absindex(L, a->idx);
}
int uqm_lua_guard_lua_absindex(lua_State *L, int idx) {
    struct args_lua_absindex a = {0};
    a.idx = idx;
    guarded(L, op_lua_absindex, &a);
    return a.result;
}

struct args_lua_gettop {  int result; };
static void op_lua_gettop(lua_State *L, void *ud) {
    struct args_lua_gettop *a = ud;
    a->result = lua_gettop(L);
}
int uqm_lua_guard_lua_gettop(lua_State *L) {
    struct args_lua_gettop a = {0};
    guarded(L, op_lua_gettop, &a);
    return a.result;
}

struct args_lua_settop { int idx; int unused; };
static void op_lua_settop(lua_State *L, void *ud) {
    struct args_lua_settop *a = ud;
    lua_settop(L, a->idx);
}
void uqm_lua_guard_lua_settop(lua_State *L, int idx) {
    struct args_lua_settop a = {0};
    a.idx = idx;
    guarded(L, op_lua_settop, &a);
}

struct args_lua_pushvalue { int idx; int unused; };
static void op_lua_pushvalue(lua_State *L, void *ud) {
    struct args_lua_pushvalue *a = ud;
    lua_pushvalue(L, a->idx);
}
void uqm_lua_guard_lua_pushvalue(lua_State *L, int idx) {
    struct args_lua_pushvalue a = {0};
    a.idx = idx;
    guarded(L, op_lua_pushvalue, &a);
}

struct args_lua_replace { int idx; int unused; };
static void op_lua_replace(lua_State *L, void *ud) {
    struct args_lua_replace *a = ud;
    lua_replace(L, a->idx);
}
void uqm_lua_guard_lua_replace(lua_State *L, int idx) {
    struct args_lua_replace a = {0};
    a.idx = idx;
    guarded(L, op_lua_replace, &a);
}

struct args_lua_type { int idx; int result; };
static void op_lua_type(lua_State *L, void *ud) {
    struct args_lua_type *a = ud;
    a->result = lua_type(L, a->idx);
}
int uqm_lua_guard_lua_type(lua_State *L, int idx) {
    struct args_lua_type a = {0};
    a.idx = idx;
    guarded(L, op_lua_type, &a);
    return a.result;
}

struct args_lua_typename { int tp; const char * result; };
static void op_lua_typename(lua_State *L, void *ud) {
    struct args_lua_typename *a = ud;
    a->result = lua_typename(L, a->tp);
}
const char * uqm_lua_guard_lua_typename(lua_State *L, int tp) {
    struct args_lua_typename a = {0};
    a.tp = tp;
    guarded(L, op_lua_typename, &a);
    return a.result;
}

struct args_lua_tonumberx { int idx; int * isnum; lua_Number result; };
static void op_lua_tonumberx(lua_State *L, void *ud) {
    struct args_lua_tonumberx *a = ud;
    a->result = lua_tonumberx(L, a->idx, a->isnum);
}
lua_Number uqm_lua_guard_lua_tonumberx(lua_State *L, int idx, int * isnum) {
    struct args_lua_tonumberx a = {0};
    a.idx = idx;
    a.isnum = isnum;
    guarded(L, op_lua_tonumberx, &a);
    return a.result;
}

struct args_lua_tointegerx { int idx; int * isnum; lua_Integer result; };
static void op_lua_tointegerx(lua_State *L, void *ud) {
    struct args_lua_tointegerx *a = ud;
    a->result = lua_tointegerx(L, a->idx, a->isnum);
}
lua_Integer uqm_lua_guard_lua_tointegerx(lua_State *L, int idx, int * isnum) {
    struct args_lua_tointegerx a = {0};
    a.idx = idx;
    a.isnum = isnum;
    guarded(L, op_lua_tointegerx, &a);
    return a.result;
}

struct args_lua_toboolean { int idx; int result; };
static void op_lua_toboolean(lua_State *L, void *ud) {
    struct args_lua_toboolean *a = ud;
    a->result = lua_toboolean(L, a->idx);
}
int uqm_lua_guard_lua_toboolean(lua_State *L, int idx) {
    struct args_lua_toboolean a = {0};
    a.idx = idx;
    guarded(L, op_lua_toboolean, &a);
    return a.result;
}

struct args_lua_tolstring { int idx; size_t * len; const char * result; };
static void op_lua_tolstring(lua_State *L, void *ud) {
    struct args_lua_tolstring *a = ud;
    a->result = lua_tolstring(L, a->idx, a->len);
}
const char * uqm_lua_guard_lua_tolstring(lua_State *L, int idx, size_t * len) {
    struct args_lua_tolstring a = {0};
    a.idx = idx;
    a.len = len;
    guarded(L, op_lua_tolstring, &a);
    return a.result;
}

struct args_lua_topointer { int idx; const void * result; };
static void op_lua_topointer(lua_State *L, void *ud) {
    struct args_lua_topointer *a = ud;
    a->result = lua_topointer(L, a->idx);
}
const void * uqm_lua_guard_lua_topointer(lua_State *L, int idx) {
    struct args_lua_topointer a = {0};
    a.idx = idx;
    guarded(L, op_lua_topointer, &a);
    return a.result;
}

struct args_lua_pushnil {  int unused; };
static void op_lua_pushnil(lua_State *L, void *ud) {
    struct args_lua_pushnil *a = ud;
    lua_pushnil(L);
    (void)a;
}
void uqm_lua_guard_lua_pushnil(lua_State *L) {
    struct args_lua_pushnil a = {0};
    guarded(L, op_lua_pushnil, &a);
}

struct args_lua_pushinteger { lua_Integer n; int unused; };
static void op_lua_pushinteger(lua_State *L, void *ud) {
    struct args_lua_pushinteger *a = ud;
    lua_pushinteger(L, a->n);
}
void uqm_lua_guard_lua_pushinteger(lua_State *L, lua_Integer n) {
    struct args_lua_pushinteger a = {0};
    a.n = n;
    guarded(L, op_lua_pushinteger, &a);
}

struct args_lua_pushstring { const char * s; const char * result; };
static void op_lua_pushstring(lua_State *L, void *ud) {
    struct args_lua_pushstring *a = ud;
    a->result = lua_pushstring(L, a->s);
}
const char * uqm_lua_guard_lua_pushstring(lua_State *L, const char * s) {
    struct args_lua_pushstring a = {0};
    a.s = s;
    guarded(L, op_lua_pushstring, &a);
    return a.result;
}

struct args_lua_pushboolean { int b; int unused; };
static void op_lua_pushboolean(lua_State *L, void *ud) {
    struct args_lua_pushboolean *a = ud;
    lua_pushboolean(L, a->b);
}
void uqm_lua_guard_lua_pushboolean(lua_State *L, int b) {
    struct args_lua_pushboolean a = {0};
    a.b = b;
    guarded(L, op_lua_pushboolean, &a);
}

struct args_lua_pushlightuserdata { void * p; int unused; };
static void op_lua_pushlightuserdata(lua_State *L, void *ud) {
    struct args_lua_pushlightuserdata *a = ud;
    lua_pushlightuserdata(L, a->p);
}
void uqm_lua_guard_lua_pushlightuserdata(lua_State *L, void * p) {
    struct args_lua_pushlightuserdata a = {0};
    a.p = p;
    guarded(L, op_lua_pushlightuserdata, &a);
}

struct args_lua_getglobal { const char * var; int unused; };
static void op_lua_getglobal(lua_State *L, void *ud) {
    struct args_lua_getglobal *a = ud;
    lua_getglobal(L, a->var);
}
void uqm_lua_guard_lua_getglobal(lua_State *L, const char * var) {
    struct args_lua_getglobal a = {0};
    a.var = var;
    guarded(L, op_lua_getglobal, &a);
}

struct args_lua_gettable { int idx; int unused; };
static void op_lua_gettable(lua_State *L, void *ud) {
    struct args_lua_gettable *a = ud;
    lua_gettable(L, a->idx);
}
void uqm_lua_guard_lua_gettable(lua_State *L, int idx) {
    struct args_lua_gettable a = {0};
    a.idx = idx;
    guarded(L, op_lua_gettable, &a);
}

struct args_lua_getfield { int idx; const char * k; int unused; };
static void op_lua_getfield(lua_State *L, void *ud) {
    struct args_lua_getfield *a = ud;
    lua_getfield(L, a->idx, a->k);
}
void uqm_lua_guard_lua_getfield(lua_State *L, int idx, const char * k) {
    struct args_lua_getfield a = {0};
    a.idx = idx;
    a.k = k;
    guarded(L, op_lua_getfield, &a);
}

struct args_lua_rawgeti { int idx; int n; int unused; };
static void op_lua_rawgeti(lua_State *L, void *ud) {
    struct args_lua_rawgeti *a = ud;
    lua_rawgeti(L, a->idx, a->n);
}
void uqm_lua_guard_lua_rawgeti(lua_State *L, int idx, int n) {
    struct args_lua_rawgeti a = {0};
    a.idx = idx;
    a.n = n;
    guarded(L, op_lua_rawgeti, &a);
}

struct args_lua_createtable { int narr; int nrec; int unused; };
static void op_lua_createtable(lua_State *L, void *ud) {
    struct args_lua_createtable *a = ud;
    lua_createtable(L, a->narr, a->nrec);
}
void uqm_lua_guard_lua_createtable(lua_State *L, int narr, int nrec) {
    struct args_lua_createtable a = {0};
    a.narr = narr;
    a.nrec = nrec;
    guarded(L, op_lua_createtable, &a);
}

struct args_lua_settable { int idx; int unused; };
static void op_lua_settable(lua_State *L, void *ud) {
    struct args_lua_settable *a = ud;
    lua_settable(L, a->idx);
}
void uqm_lua_guard_lua_settable(lua_State *L, int idx) {
    struct args_lua_settable a = {0};
    a.idx = idx;
    guarded(L, op_lua_settable, &a);
}

struct args_lua_setfield { int idx; const char * k; int unused; };
static void op_lua_setfield(lua_State *L, void *ud) {
    struct args_lua_setfield *a = ud;
    lua_setfield(L, a->idx, a->k);
}
void uqm_lua_guard_lua_setfield(lua_State *L, int idx, const char * k) {
    struct args_lua_setfield a = {0};
    a.idx = idx;
    a.k = k;
    guarded(L, op_lua_setfield, &a);
}

struct args_lua_rawseti { int idx; int n; int unused; };
static void op_lua_rawseti(lua_State *L, void *ud) {
    struct args_lua_rawseti *a = ud;
    lua_rawseti(L, a->idx, a->n);
}
void uqm_lua_guard_lua_rawseti(lua_State *L, int idx, int n) {
    struct args_lua_rawseti a = {0};
    a.idx = idx;
    a.n = n;
    guarded(L, op_lua_rawseti, &a);
}

struct args_luaL_checkinteger { int numArg; lua_Integer result; };
static void op_luaL_checkinteger(lua_State *L, void *ud) {
    struct args_luaL_checkinteger *a = ud;
    a->result = luaL_checkinteger(L, a->numArg);
}
lua_Integer uqm_lua_guard_luaL_checkinteger(lua_State *L, int numArg) {
    struct args_luaL_checkinteger a = {0};
    a.numArg = numArg;
    guarded(L, op_luaL_checkinteger, &a);
    return a.result;
}

struct args_luaL_checklstring { int numArg; size_t * len; const char * result; };
static void op_luaL_checklstring(lua_State *L, void *ud) {
    struct args_luaL_checklstring *a = ud;
    a->result = luaL_checklstring(L, a->numArg, a->len);
}
const char * uqm_lua_guard_luaL_checklstring(lua_State *L, int numArg, size_t * len) {
    struct args_luaL_checklstring a = {0};
    a.numArg = numArg;
    a.len = len;
    guarded(L, op_luaL_checklstring, &a);
    return a.result;
}

struct args_luaL_checktype { int numArg; int t; int unused; };
static void op_luaL_checktype(lua_State *L, void *ud) {
    struct args_luaL_checktype *a = ud;
    luaL_checktype(L, a->numArg, a->t);
}
void uqm_lua_guard_luaL_checktype(lua_State *L, int numArg, int t) {
    struct args_luaL_checktype a = {0};
    a.numArg = numArg;
    a.t = t;
    guarded(L, op_luaL_checktype, &a);
}

struct args_luaL_checkoption { int arg; const char * def; const char *const * lst; int result; };
static void op_luaL_checkoption(lua_State *L, void *ud) {
    struct args_luaL_checkoption *a = ud;
    a->result = luaL_checkoption(L, a->arg, a->def, a->lst);
}
int uqm_lua_guard_luaL_checkoption(lua_State *L, int arg, const char * def, const char *const * lst) {
    struct args_luaL_checkoption a = {0};
    a.arg = arg;
    a.def = def;
    a.lst = lst;
    guarded(L, op_luaL_checkoption, &a);
    return a.result;
}

struct args_luaL_loadstring { const char * s; int result; };
static void op_luaL_loadstring(lua_State *L, void *ud) {
    struct args_luaL_loadstring *a = ud;
    a->result = luaL_loadstring(L, a->s);
}
int uqm_lua_guard_luaL_loadstring(lua_State *L, const char * s) {
    struct args_luaL_loadstring a = {0};
    a.s = s;
    guarded(L, op_luaL_loadstring, &a);
    return a.result;
}

struct args_lua_pushcclosure { lua_CFunction fn; int n; int unused; };
static void op_lua_pushcclosure(lua_State *L, void *ud) {
    struct args_lua_pushcclosure *a = ud;
    guarded_closure(L, a->fn, a->n);
}
void uqm_lua_guard_lua_pushcclosure(lua_State *L, lua_CFunction fn, int n) {
    struct args_lua_pushcclosure a = {0};
    a.fn = fn;
    a.n = n;
    guarded(L, op_lua_pushcclosure, &a);
}

struct args_luaL_setfuncs { const luaL_Reg * l; int nup; int unused; };
static void op_luaL_setfuncs(lua_State *L, void *ud) {
    struct args_luaL_setfuncs *a = ud;
    guarded_setfuncs(L, a->l, a->nup);
}
void uqm_lua_guard_luaL_setfuncs(lua_State *L, const luaL_Reg * l, int nup) {
    struct args_luaL_setfuncs a = {0};
    a.l = l;
    a.nup = nup;
    guarded(L, op_luaL_setfuncs, &a);
}

struct args_lua_callk { int nargs; int nresults; int ctx; lua_CFunction k; int unused; };
static void op_lua_callk(lua_State *L, void *ud) {
    struct args_lua_callk *a = ud;
    if (a->k != NULL) abort(); /* Game callbacks never yield. */
    lua_callk(L, a->nargs, a->nresults, a->ctx, a->k);
}
void uqm_lua_guard_lua_callk(lua_State *L, int nargs, int nresults, int ctx, lua_CFunction k) {
    struct args_lua_callk a = {0};
    a.nargs = nargs;
    a.nresults = nresults;
    a.ctx = ctx;
    a.k = k;
    guarded(L, op_lua_callk, &a);
}

struct args_lua_pcallk { int nargs; int nresults; int errfunc; int ctx; lua_CFunction k; int result; };
static void op_lua_pcallk(lua_State *L, void *ud) {
    struct args_lua_pcallk *a = ud;
    if (a->k != NULL) abort(); /* Game callbacks never yield. */
    a->result = lua_pcallk(L, a->nargs, a->nresults, a->errfunc, a->ctx, a->k);
}
int uqm_lua_guard_lua_pcallk(lua_State *L, int nargs, int nresults, int errfunc, int ctx, lua_CFunction k) {
    struct args_lua_pcallk a = {0};
    a.nargs = nargs;
    a.nresults = nresults;
    a.errfunc = errfunc;
    a.ctx = ctx;
    a.k = k;
    guarded(L, op_lua_pcallk, &a);
    return a.result;
}

struct args_lua_load { lua_Reader reader; void * dt; const char * chunkname; const char * mode; int result; };
static void op_lua_load(lua_State *L, void *ud) {
    struct args_lua_load *a = ud;
    struct reader_data reader = {a->reader, a->dt};
    a->result = lua_load(L, rust_reader, &reader, a->chunkname, a->mode);
}
int uqm_lua_guard_lua_load(lua_State *L, lua_Reader reader, void * dt, const char * chunkname, const char * mode) {
    struct args_lua_load a = {0};
    a.reader = reader;
    a.dt = dt;
    a.chunkname = chunkname;
    a.mode = mode;
    guarded(L, op_lua_load, &a);
    return a.result;
}

struct error_args { const char *format; va_list arguments; };
static void op_error(lua_State *L, void *ud) {
    struct error_args *a = ud;
    luaL_where(L, 1);
    lua_pushvfstring(L, a->format, a->arguments);
    lua_concat(L, 2);
    lua_error(L);
}
int uqm_lua_guard_luaL_error(lua_State *L, const char *fmt, ...) {
    struct error_args a;
    a.format = fmt;
    va_start(a.arguments, fmt);
    guarded(L, op_error, &a);
    va_end(a.arguments);
    return 0;
}
lua_State *uqm_lua_guard_luaL_newstate(void) {
    if (uqm_lua_failed()) return NULL;
    /* lua_newstate protects its own initialization and returns NULL on OOM. */
    lua_State *L = luaL_newstate();
    if (!L) abort(); /* Existing engine assumes state creation succeeds. */
    return L;
}
void uqm_lua_guard_lua_close(lua_State *L) {
    if (uqm_lua_failed()) return;
    /* Cannot enclose a freed state in luaD_pcall. Lua 5.2 close_state runs
     * finalizers under native protection with propagateerrors=0. */
    lua_close(L);
}
