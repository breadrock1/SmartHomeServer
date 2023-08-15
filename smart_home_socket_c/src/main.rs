mod lib;
use lib::*;
use libloading::{Library, Symbol};
use std::error::Error;
use std::ffi::{c_char, CString};

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let lib_path = "target\\debug\\smart_home_socket_c.dll";
        let lib = Library::new(lib_path)?;

        let room_new_func: Symbol<unsafe extern "C" fn(*const c_char) -> *mut CRoom> =
            lib.get(b"room_new")?;
        let socket_new_func: Symbol<
            unsafe extern "C" fn(ptr: *mut CRoom, name: *const c_char) -> *const c_char,
        > = lib.get(b"socket_new")?;

        let room_name = CString::new("Kitchen").unwrap();
        let room_name_c = string_to_const_char(room_name).unwrap();
        let c_room = room_new_func(room_name_c);

        let socket_name = CString::new("Socket 1").unwrap();
        let socket_name_c = string_to_const_char(socket_name).unwrap();
        let c_socket = socket_new_func(c_room, socket_name_c);

        let cmp_string = CString::new("Created socket: Socket 1");
        let c_socket_name = const_char_to_string(c_socket);
        assert_eq!(cmp_string.unwrap(), c_socket_name.unwrap());

        Ok(())
    }
}
