use libc::c_void;

extern "C" {
    fn luaL_newstate() -> *mut c_void;
    fn luaL_openlibs(L: *mut c_void) -> *mut c_void;
    fn lua_close(L: *mut c_void) -> *mut c_void;
}

fn main() {
    unsafe {
        let l = luaL_newstate();
        luaL_openlibs(l);
        lua_close(l);
    }
}
