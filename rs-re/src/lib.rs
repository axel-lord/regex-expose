use std::ffi::{c_char, c_void, CStr};

#[no_mangle]
pub extern "C" fn re_create(c_str: *const c_char) -> *mut c_void {
    unsafe {
        Box::into_raw(Box::new(
            regex::Regex::new(CStr::from_ptr(c_str).to_str().unwrap()).unwrap(),
        ))
        .cast()
    }
}

#[no_mangle]
pub extern "C" fn re_destroy(v_ptr: *mut c_void) {
    let re = unsafe { Box::from_raw(v_ptr.cast::<regex::Regex>()) };

    println!("{}", re.as_str());
}
