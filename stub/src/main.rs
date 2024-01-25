use libc::{c_void, c_char};
use std::ffi::CString;

extern "C" {
    fn luaL_newstate() -> *mut c_void;
    fn luaL_openlibs(L: *mut c_void) -> *mut c_void;
    fn lua_close(L: *mut c_void) -> *mut c_void;
    fn luaL_loadfile (L: *mut c_void, filename: *const c_char) -> i32;
    fn lua_pcall(L: *mut c_void, nargs: i32, nresults: i32, errfunc: i32) -> i32;
}

fn main() {
    let script = r"E:\repository\Guardalupe\stub\test_script.lua";
    unsafe {
        let lua_state = luaL_newstate();
        luaL_openlibs(lua_state);

        let script_str = CString::new(script).expect("CString conversion of script name failed");
        let script_ptr = script_str.as_ptr();
        let mut result = luaL_loadfile(lua_state, script_ptr);
        if result != 0 {
            println!("luaL_loadfile failed: {result}");
        }
        result = lua_pcall(lua_state, 0, 0, 0);
        if result != 0 {
            println!("lua_pcall failed: {result}")
        }

        lua_close(lua_state);
    }
}
