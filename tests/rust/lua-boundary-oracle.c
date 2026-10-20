/* Independent execution against the pinned, unchanged Lua runtime. */
#include "lua-guard.h"
#include "lualib.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int rust_lua_exercise(lua_State *);
void rust_eh_personality(void) { abort(); }
static unsigned enters, leaves, completed, checks, cases;
static long allocation_budget = -1;
static void require(int ok, const char *what) {
    checks++;
    if (!ok) { fprintf(stderr, "Lua boundary mismatch: %s\n", what); abort(); }
}
void test_rust_enter(void) { enters++; }
void test_rust_leave(void) { leaves++; }
void test_operation_completed(void) { completed++; }
void test_arm_oom(void) { allocation_budget = 0; }
void test_disarm_oom(void) { allocation_budget = -1; }
const char *test_big_string(void) {
    static char big[65537];
    memset(big, 'p', sizeof big - 1); big[sizeof big - 1] = 0; return big;
}
static void *allocator(void *ud, void *ptr, size_t old, size_t size) {
    (void)ud;
    if (size == 0) { free(ptr); return NULL; }
    /* Lua allocator contract: shrinking an allocation must never fail. */
    if (ptr && size <= old) return ptr;
    if (allocation_budget == 0) return NULL;
    if (allocation_budget > 0) allocation_budget--;
    return realloc(ptr, size);
}
static int native_nested(lua_State *L) {
    lua_Integer n = luaL_checkinteger(L, 1);
    lua_pushinteger(L, n + 1); test_operation_completed(); return 1;
}
static int native_upvalue(lua_State *L) {
    lua_pushinteger(L,luaL_checkinteger(L,1)+lua_tointeger(L,lua_upvalueindex(1)));
    return 1;
}
static const char *native_reader(lua_State *L, void *data, size_t *size) {
    int *read = data;
    if (*read) { *size = 0; return NULL; }
    *read = 1;
    lua_Integer v = luaL_checkinteger(L, 1);
    if (!v) { *size = 8; return "return 7"; }
    *size = 7; return "return ";
}
static int native_exercise(lua_State *L) {
    int mode = (int)lua_tointegerx(L, lua_upvalueindex(1), NULL), results = 0;
    switch (mode) {
    case 0: lua_pushinteger(L, luaL_checkinteger(L, 1) + 5); results=1; break;
    case 1: { size_t size; const char *p=luaL_checklstring(L,1,&size);
        lua_pushstring(L,p); lua_pushinteger(L,(lua_Integer)size); results=2; break; }
    case 2: { int valid; luaL_checktype(L,1,LUA_TTABLE); lua_getfield(L,1,"value");
        lua_Number n=lua_tonumberx(L,-1,&valid); lua_pushinteger(L,(lua_Integer)n);
        lua_pushboolean(L,valid); results=2; break; }
    case 3: { const char *options[]={"red","blue",NULL};
        lua_pushinteger(L,luaL_checkoption(L,1,"blue",options)); results=1; break; }
    case 4: return luaL_error(L,"custom %s %d","failure",17);
    case 5: { lua_createtable(L,2,2); int table=lua_absindex(L,-1);
        lua_pushinteger(L,42); lua_setfield(L,table,"key");
        lua_pushstring(L,"seven"); lua_rawseti(L,table,7);
        lua_pushstring(L,"key"); lua_gettable(L,table); lua_rawgeti(L,table,7);
        lua_getglobal(L,"type"); lua_pushvalue(L,table); lua_callk(L,1,1,0,NULL);
        results=3; break; }
    case 6: lua_pushvalue(L,1); lua_callk(L,0,1,0,NULL); results=1; break;
    case 7: { lua_pushvalue(L,1); int status=lua_pcallk(L,0,1,0,0,NULL);
        lua_pushinteger(L,status); results=2; break; }
    case 8: { size_t size; const char *p=lua_tolstring(L,1,&size);
        lua_pushstring(L,p); lua_pushinteger(L,(lua_Integer)size); results=2; break; }
    case 9: { lua_pushlightuserdata(L,L); const void *p=lua_topointer(L,-1);
        lua_pushboolean(L,p==L); int n=lua_toboolean(L,-1); lua_pushnil(L);
        lua_replace(L,-2); lua_settop(L,-2); lua_pushinteger(L,n);
        lua_pushstring(L,lua_typename(L,lua_type(L,-1))); results=2; break; }
    case 10: { const luaL_Reg regs[]={{"inc",native_nested},{NULL,NULL}};
        lua_createtable(L,0,1); luaL_setfuncs(L,regs,0); lua_getfield(L,-1,"inc");
        lua_pushvalue(L,1); lua_callk(L,1,1,0,NULL); results=1; break; }
    case 11: { int read=0; int status=lua_load(L,native_reader,&read,"reader",NULL);
        lua_pushinteger(L,status); results=2; break; }
    case 12: test_arm_oom(); lua_createtable(L,100000,0); test_disarm_oom(); results=1; break;
    case 13: test_arm_oom(); lua_pushstring(L,test_big_string()); test_disarm_oom(); results=1; break;
    case 14: test_arm_oom(); lua_tolstring(L,1,NULL); test_disarm_oom(); results=1; break;
    case 15: lua_pushinteger(L,luaL_loadstring(L,"return (")); results=2; break;
    case 16: lua_pushstring(L,"key"); lua_pushinteger(L,9); lua_settable(L,1); break;
    case 17: lua_pushinteger(L,lua_gettop(L)); results=1; break;
    case 18: { const luaL_Reg regs[]={{"sum",native_upvalue},{"sum_again",native_upvalue},{NULL,NULL}};
        lua_createtable(L,0,2);lua_pushinteger(L,43);luaL_setfuncs(L,regs,1);
        lua_getfield(L,-1,"sum_again");lua_pushvalue(L,1);lua_call(L,1,1);results=1;break; }
    case 19: test_arm_oom();lua_pushcclosure(L,native_nested,0);test_disarm_oom();results=1;break;
    case 20: test_arm_oom();return luaL_error(L,"error message with new allocation %d",918273);
    case 21: test_arm_oom();luaL_checkinteger(L,1);test_disarm_oom();break;
    default: abort();
    }
    test_operation_completed(); return results;
}
struct snapshot { int status, top, kind[12]; char values[12][512]; unsigned completed; };
static lua_State *state(int rust, int mode) {
    test_disarm_oom();
    lua_State *L=lua_newstate(allocator,NULL); require(L!=NULL,"new state");
    /* Exercise the whitelist through the production registration wrapper. */
    lua_CFunction libs[]={luaopen_base,luaopen_table,luaopen_string,luaopen_bit32,luaopen_math};
    for (unsigned i=0;i<sizeof libs/sizeof libs[0];i++) {
        if (rust) uqm_lua_guard_lua_pushcclosure(L,libs[i],0); else lua_pushcclosure(L,libs[i],0);
        lua_call(L,0,0);
    }
    lua_pushinteger(L,mode);
    if (rust) uqm_lua_guard_lua_pushcclosure(L,rust_lua_exercise,1);
    else lua_pushcclosure(L,native_exercise,1);
    lua_setglobal(L,"exercise");
    return L;
}
static void snap(lua_State *L, int status, struct snapshot *s) {
    memset(s,0,sizeof *s); s->status=status;s->top=lua_gettop(L);s->completed=completed;
    require(s->top<=12,"snapshot stack capacity");
    for(int i=1;i<=s->top;i++) {
        int type=lua_type(L,i); s->kind[i-1]=type;
        if(type==LUA_TBOOLEAN) snprintf(s->values[i-1],512,"%d",lua_toboolean(L,i));
        else if(type==LUA_TNUMBER) snprintf(s->values[i-1],512,"%.17g",lua_tonumber(L,i));
        else if(type==LUA_TSTRING) {
            size_t n;const char *p=lua_tolstring(L,i,&n);require(n<512,"snapshot string capacity");
            memcpy(s->values[i-1],p,n); /* Byte strings include embedded NUL. */
        }
    }
}
static void check_returned(unsigned before_enter, unsigned before_leave) {
    require(enters-before_enter==leaves-before_leave,"every Rust Drop ran before Lua caught error");
    require(uqm_lua_failed()==0,"no escaped TLS frame or pending failure");
}
static void recovery(lua_State *L, int rust) {
    lua_settop(L,0);lua_pushinteger(L,0);
    if(rust) uqm_lua_guard_lua_pushcclosure(L,rust_lua_exercise,1);
    else lua_pushcclosure(L,native_exercise,1);
    lua_pushinteger(L,37);
    unsigned e=enters,l=leaves;
    require(lua_pcall(L,1,1,0)==LUA_OK,"state usable after caught error");
    require(lua_tointeger(L,-1)==42,"recovery result"); check_returned(e,l);
    lua_settop(L,0);
}
static void execute(int rust, int mode, const char *source, struct snapshot *s) {
    lua_State *L=state(rust,mode);require(luaL_loadstring(L,source)==LUA_OK,"test chunk parses");
    unsigned e=enters,l=leaves;completed=0;
    int status=lua_pcall(L,0,LUA_MULTRET,0);test_disarm_oom();
    check_returned(e,l);snap(L,status,s);recovery(L,rust);
    if(rust) uqm_lua_guard_lua_close(L); else lua_close(L);
    require(uqm_lua_failed()==0,"state close leaves frame stack empty");
}
static void compare(int mode,const char *source) {
    struct snapshot a,b;execute(0,mode,source,&a);execute(1,mode,source,&b);
    if(memcmp(&a,&b,sizeof a)) {
        fprintf(stderr,"mode=%d source=%s\nC:status=%d top=%d value=%s completed=%u\nRust:status=%d top=%d value=%s completed=%u\n",mode,source,a.status,a.top,a.values[0],a.completed,b.status,b.top,b.values[0],b.completed);
        require(0,"C and Rust callback output/status/side effects");
    }
    require(1,"C and Rust agree");cases++;
}
static void sweep_allocations(int mode,const char *source) {
    unsigned failures=0,successes=0;
    for(long budget=0;budget<80;budget++) {
        lua_State *L=state(1,mode);require(luaL_loadstring(L,source)==LUA_OK,"sweep chunk parses");
        unsigned e=enters,l=leaves;completed=0;allocation_budget=budget;
        int status=lua_pcall(L,0,LUA_MULTRET,0);test_disarm_oom();check_returned(e,l);
        require(status==LUA_OK || status==LUA_ERRMEM,"allocation sweep returns success or memory error");
        if(status==LUA_ERRMEM) { failures++;require(lua_type(L,-1)==LUA_TSTRING,"preallocated OOM error survives"); }
        else successes++;
        recovery(L,1);uqm_lua_guard_lua_close(L);cases++;
    }
    require(failures>0,"allocation sweep hit an error");require(successes>0,"allocation sweep reached success");
}
int main(void) {
    lua_State *fresh=uqm_lua_guard_luaL_newstate();require(fresh!=NULL,"guarded newstate");uqm_lua_guard_lua_close(fresh);
    const char *integer_args[]={"0","-17","2147483642","'42'","true","nil","{}","function() end",""};
    char script[512];
    for(unsigned i=0;i<sizeof integer_args/sizeof integer_args[0];i++) {
        snprintf(script,sizeof script,"return exercise(%s)",integer_args[i]);compare(0,script);compare(10,script);
    }
    compare(1,"return exercise('hello')");compare(1,"return exercise('a\\0b')");compare(1,"return exercise(31415)");compare(1,"return exercise(false)");
    compare(2,"return exercise({value=18})");compare(2,"return exercise({value='19'})");compare(2,"return exercise(false)");
    compare(2,"return exercise(setmetatable({}, {__index=function() error('index exploded') end}))");
    compare(3,"return exercise('red')");compare(3,"return exercise(nil)");compare(3,"return exercise('green')");
    compare(4,"return exercise()");compare(5,"return exercise()");
    compare(0,"return xpcall(function() exercise(false) end, function(e) return 'handled: '..e end)");
    compare(4,"return xpcall(exercise, function(e) return 'handled: '..e end)");
    compare(4,"local n=0;return xpcall(exercise,function(e) n=n+1;if n==1 then error('handler failed') end;return 'recovered handler' end)");
    compare(4,"local n=0; local ok,e=xpcall(exercise, function(e) n=n+1; return e end);return ok,e,n");
    compare(6,"local ok,e=pcall(exercise,function() error({code=71}) end);return ok,type(e),e.code");
    compare(6,"return xpcall(function() exercise(function() error('nested') end) end, function(e) return 'handled: '..e end)");
    compare(7,"return xpcall(function() return exercise(function() error('caught internally') end) end, function(e) return 'handled: '..e end)");
    compare(0,"return xpcall(function() exercise(false) end, function(e) return exercise(23) end)");
    compare(6,"return exercise(function() return 77 end)");compare(6,"return exercise(function() error('nested error') end)");
    compare(6,"return exercise(function() return exercise(function() error('nested nested') end) end)");
    compare(7,"return exercise(function() return 77 end)");compare(7,"return exercise(function() error('caught error') end)");
    compare(7,"return exercise(function() return exercise(function() error('caught twice') end) end)");
    compare(8,"return exercise(23456.125)");compare(8,"return exercise(false)");compare(9,"return exercise()");
    compare(11,"return exercise(0)");compare(11,"return exercise(1)");compare(11,"return exercise('invalid reader argument')");
    compare(12,"return exercise()");compare(13,"return exercise()");compare(14,"return exercise(987654.125)");
    compare(15,"return exercise()");compare(16,"return exercise({})");
    compare(16,"return exercise(setmetatable({}, {__newindex=function() error('write exploded') end}))");
    compare(17,"return exercise(1,2,3,4,5)");
    compare(18,"return exercise(77)");compare(18,"return exercise(false)");
    compare(20,"return exercise()");compare(21,"return exercise(false)");
    for(int i=-128;i<=128;i++) {snprintf(script,sizeof script,"return exercise(%d)",i*4093);compare(0,script);}
    for(int rust=0;rust<2;rust++) {
        lua_State *L=state(rust,4);
        require(luaL_dostring(L,"t=setmetatable({}, {__gc=exercise})")==LUA_OK,"register finalizer");
        unsigned e=enters,l=leaves;
        if(rust) uqm_lua_guard_lua_close(L);else lua_close(L);
        check_returned(e,l);if(rust)require(enters>e,"Rust finalizer returned before suppressed error");cases++;
    }
    { struct snapshot s;execute(1,19,"return exercise()",&s);require(s.status==LUA_ERRMEM,"callback registration OOM caught");cases++; }
    /* lua_pcall inside Rust must be free to catch a nested Rust callback error. */
    compare(7,"return exercise(function() return exercise(false) end)");
    sweep_allocations(5,"return exercise()");sweep_allocations(10,"return exercise(123)");sweep_allocations(11,"return exercise(0)");sweep_allocations(18,"return exercise(99)");
    require(enters==leaves,"all Rust frames returned");
    printf("Lua boundary passed: %u checks, %u cases, %u Rust frames returned; bad arguments, metamethod errors, parser/reader failures, nested callbacks and allocation failures.\n",checks,cases,enters);
    return 0;
}
