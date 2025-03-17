use libc::{c_void, c_char};
use std::ffi::CString;

extern "C" {
    fn luaL_newstate() -> *mut c_void;
    fn luaL_openlibs(L: *mut c_void) -> *mut c_void;
    fn lua_close(L: *mut c_void) -> *mut c_void;
    fn luaL_loadfile (L: *mut c_void, filename: *const c_char) -> i32;
    fn lua_pcall(L: *mut c_void, nargs: i32, nresults: i32, errfunc: i32) -> i32;
    fn lua_dump(L: *mut c_void, lua_writer: unsafe extern "C" fn(*mut c_void, *const c_void, usize, *mut c_void) -> i32, data: *mut c_void) -> i32;
    fn luaL_loadbuffer(L: *mut c_void, buff: *const u8, sz: usize, name: *const c_char) -> i32;
}

fn main() {
    let _script_filename = r"E:\repository\Guardalupe\stub\test_script.bc";
    //let buffer = "print('Hello World')";
    let buffer: Vec<u8> = vec![0x1B, 0x4C, 0x4A, 0x02, 0x0A, 0x29, 0x02, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x36, 0x00, 0x00, 0x00, 0x27, 0x02, 0x01, 0x00, 0x42, 0x00, 0x02, 0x01, 0x4B, 0x00, 0x01, 0x00, 0x10, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x0A, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x00];

    unsafe {
        let lua_state = luaL_newstate();
        luaL_openlibs(lua_state);

        //let script_filename_str = CString::new(script_filename).expect("CString conversion of script name failed");
        //let script_filename_ptr = script_filename_str.as_ptr();
        // let mut result = luaL_loadfile(lua_state, script_ptr);

        let buffer_name_str = CString::new("test_buffer").expect("CScript conversion of buffer name failed.");
        let buffer_name_ptr = buffer_name_str.as_ptr();
        //let buffer_str = CString::new(buffer).expect("CString conversion of buffer failed");
        let buffer_ptr = buffer.as_ptr();
        let mut result = luaL_loadbuffer(lua_state, buffer_ptr, buffer.len(), buffer_name_ptr);

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
