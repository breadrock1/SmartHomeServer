use smart_home_socket::room::Room;
use smart_home_socket::socket::Socket;
use std::ffi::{c_char, CString};

#[repr(C)]
pub struct CRoom {
    pub ptr: Room,
    pub name: Option<CString>,
    pub dev_name: Option<CString>,
}

#[repr(C)]
pub struct CSocketIter {
    ptr: *mut CRoom,
    name: Option<CString>,
    cursor: usize,
}

/// free handle
/// # Safety
///
#[no_mangle]
unsafe extern "C" fn room_new(name: *const c_char) -> *mut CRoom {
    let name = CString::from_raw(name.cast_mut());
    let room = Room::default();

    let ptr = CRoom {
        ptr: room,
        name: Some(name),
        dev_name: None,
    };

    let handle = Box::new(ptr);
    Box::into_raw(handle)
}

/// free handle
/// # Safety
///
#[no_mangle]
unsafe extern "C" fn room_free(ptr: *mut CRoom) {
    let _ = Box::from_raw(ptr);
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn socket_get(ptr: *mut CRoom, name: *const c_char) -> Socket {
    let c_room = &mut *ptr;

    let socket_name = const_char_to_string(name).unwrap();
    let socket_bytes = socket_name.to_bytes();
    let socket_name = String::from_utf8_lossy(socket_bytes).to_string();

    c_room.ptr.get_socket(socket_name).unwrap()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn socket_new(ptr: *mut CRoom, name: *const c_char) -> *const c_char {
    let c_room = &mut *ptr;

    let socket_name = const_char_to_string(name).unwrap();
    let socket_bytes = socket_name.to_bytes();
    let socket_name_str = String::from_utf8_lossy(socket_bytes).to_string();

    let msg = match c_room.ptr.create_socket(socket_name_str) {
        None => "Failed while creating socket".to_string(),
        Some(s) => format!("Created socket: {}", s),
    };

    c_room.dev_name = Some(socket_name.clone());
    let result_str = CString::new(msg).unwrap();
    result_str.into_raw().cast_const()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn socket_free(ptr: *mut CRoom, name: *const c_char) -> *const c_char {
    let handle = &mut *ptr;

    let socket_name = const_char_to_string(name).unwrap();
    let socket_bytes = socket_name.to_bytes();
    let socket_name_str = String::from_utf8_lossy(socket_bytes).to_string();

    let msg = match handle.ptr.remove_socket(socket_name_str) {
        Err(_) => "Failed while removing socket".to_string(),
        Ok(s) => format!("Removed socket: {}", s),
    };

    handle.dev_name = None;
    let result_str = CString::new(msg).unwrap();
    result_str.into_raw().cast_const()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn socket_switch(ptr: *mut CRoom, name: *const c_char) -> *const c_char {
    let handle = &mut *ptr;

    let socket_name = const_char_to_string(name).unwrap();
    let socket_bytes = socket_name.to_bytes();
    let socket_name_str = String::from_utf8_lossy(socket_bytes).to_string();

    let msg = match handle.ptr.switch_socket(socket_name_str) {
        Err(_) => "Failed while switching socket".to_string(),
        Ok(s) => format!("Switched socket: {}", s),
    };

    let result_str = CString::new(msg).unwrap();
    result_str.into_raw().cast_const()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn socket_status(ptr: *mut CRoom, name: *const c_char) -> *const c_char {
    let handle = &mut *ptr;

    let socket_name = const_char_to_string(name).unwrap();
    let socket_bytes = socket_name.to_bytes();
    let socket_name_str = String::from_utf8_lossy(socket_bytes).to_string();

    let msg = match handle.ptr.check_status(socket_name_str) {
        Err(_) => "Failed while getting status".to_string(),
        Ok(s) => format!("Status: {}", s),
    };

    let result_str = CString::new(msg).unwrap();
    result_str.into_raw().cast_const()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn sockets_count(ptr: *mut CRoom) -> usize {
    let c_room = &mut *ptr;
    c_room.ptr.sockets_iter().count()
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn sockets_iter(ptr: *mut CRoom) -> CSocketIter {
    let c_room = &mut *ptr;

    let mut s_iter = c_room.ptr.sockets_iter();
    let socket_name = match s_iter.next() {
        None => None,
        Some(s) => {
            let s_str = CString::new(s).unwrap();
            Some(s_str)
        }
    };

    CSocketIter {
        ptr,
        cursor: 0,
        name: socket_name,
    }
}

/// free handle
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn sockets_next(ptr: *mut CRoom, iter: *mut CSocketIter) -> CSocketIter {
    let c_room = &mut *ptr;
    let c_socket = &mut *iter;

    let cursor = c_socket.cursor;
    let mut s_iter = c_room.ptr.sockets_iter();
    let socket_name = match s_iter.nth(cursor) {
        None => None,
        Some(s) => {
            let s_str = CString::new(s).unwrap();
            Some(s_str)
        }
    };

    CSocketIter {
        ptr,
        name: socket_name,
        cursor: cursor + 1,
    }
}

/// free handle
/// # Safety
///
pub unsafe fn const_char_to_string(src: *const c_char) -> Option<CString> {
    Some(CString::from_raw(src.cast_mut()))
}

/// free handle
/// # Safety
///
pub unsafe fn string_to_const_char(src: CString) -> Option<*const c_char> {
    Some(src.into_raw().cast_const())
}
