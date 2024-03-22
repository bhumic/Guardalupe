#include <stdio.h>

extern "C" {
    #include <lua.h>
    #include <lauxlib.h>
    #include <lualib.h>
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: LuaScriptExecutor.exe <script file>");
        return 1;
    }

    const char* script_name = argv[1];

    lua_State* L = luaL_newstate();
    if (L == nullptr) {
        fprintf(stderr, "Failed to create a new Lua state");
        return 1;
    }
    luaL_openlibs(L);

    int status = luaL_loadfile(L, script_name);
    if (status != LUA_OK) {
        fprintf(stderr, "Failed to load script: %s(%d)\n", script_name, status);
        return 1;
    }

    status = lua_pcall(L, 0, 0, 0);
    if (status != LUA_OK) {
        fprintf(stderr, "Failed to execute Lua chunk");
        return 1;
    }

    lua_close(L);

    return 0;
}