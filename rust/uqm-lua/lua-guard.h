/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Lua 5.2 FFI error boundary. See README.md. */
#ifndef UQM_LUA_GUARD_H
#define UQM_LUA_GUARD_H
#include "lua.h"
#include "lauxlib.h"
int uqm_lua_failed(void);
int uqm_lua_guard_lua_absindex(lua_State *L, int idx);
int uqm_lua_guard_lua_gettop(lua_State *L);
void uqm_lua_guard_lua_settop(lua_State *L, int idx);
void uqm_lua_guard_lua_pushvalue(lua_State *L, int idx);
void uqm_lua_guard_lua_replace(lua_State *L, int idx);
int uqm_lua_guard_lua_type(lua_State *L, int idx);
const char * uqm_lua_guard_lua_typename(lua_State *L, int tp);
lua_Number uqm_lua_guard_lua_tonumberx(lua_State *L, int idx, int * isnum);
lua_Integer uqm_lua_guard_lua_tointegerx(lua_State *L, int idx, int * isnum);
int uqm_lua_guard_lua_toboolean(lua_State *L, int idx);
const char * uqm_lua_guard_lua_tolstring(lua_State *L, int idx, size_t * len);
const void * uqm_lua_guard_lua_topointer(lua_State *L, int idx);
void uqm_lua_guard_lua_pushnil(lua_State *L);
void uqm_lua_guard_lua_pushinteger(lua_State *L, lua_Integer n);
const char * uqm_lua_guard_lua_pushstring(lua_State *L, const char * s);
void uqm_lua_guard_lua_pushboolean(lua_State *L, int b);
void uqm_lua_guard_lua_pushlightuserdata(lua_State *L, void * p);
void uqm_lua_guard_lua_getglobal(lua_State *L, const char * var);
void uqm_lua_guard_lua_gettable(lua_State *L, int idx);
void uqm_lua_guard_lua_getfield(lua_State *L, int idx, const char * k);
void uqm_lua_guard_lua_rawgeti(lua_State *L, int idx, int n);
void uqm_lua_guard_lua_createtable(lua_State *L, int narr, int nrec);
void uqm_lua_guard_lua_settable(lua_State *L, int idx);
void uqm_lua_guard_lua_setfield(lua_State *L, int idx, const char * k);
void uqm_lua_guard_lua_rawseti(lua_State *L, int idx, int n);
lua_Integer uqm_lua_guard_luaL_checkinteger(lua_State *L, int numArg);
const char * uqm_lua_guard_luaL_checklstring(lua_State *L, int numArg, size_t * len);
void uqm_lua_guard_luaL_checktype(lua_State *L, int numArg, int t);
int uqm_lua_guard_luaL_checkoption(lua_State *L, int arg, const char * def, const char *const * lst);
int uqm_lua_guard_luaL_loadstring(lua_State *L, const char * s);
void uqm_lua_guard_lua_pushcclosure(lua_State *L, lua_CFunction fn, int n);
void uqm_lua_guard_luaL_setfuncs(lua_State *L, const luaL_Reg * l, int nup);
void uqm_lua_guard_lua_callk(lua_State *L, int nargs, int nresults, int ctx, lua_CFunction k);
int uqm_lua_guard_lua_pcallk(lua_State *L, int nargs, int nresults, int errfunc, int ctx, lua_CFunction k);
int uqm_lua_guard_lua_load(lua_State *L, lua_Reader reader, void * dt, const char * chunkname, const char * mode);
int uqm_lua_guard_luaL_error(lua_State *L, const char *fmt, ...);
lua_State *uqm_lua_guard_luaL_newstate(void);
void uqm_lua_guard_lua_close(lua_State *L);
#endif
