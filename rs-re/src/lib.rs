use regex::{Captures, Regex};
use std::ffi::{c_char, c_int, c_void, CStr};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct find_match {
    pub found: c_int,
    pub start: usize,
    pub end: usize,
}

#[no_mangle]
pub extern "C" fn re_create(c_str: *const c_char) -> *mut c_void {
    unsafe {
        Box::into_raw(Box::new(
            Regex::new(
                CStr::from_ptr(c_str)
                    .to_str()
                    .expect("regex could not become a string slice"),
            )
            .expect("regex could not be compiled"),
        ))
        .cast()
    }
}

#[no_mangle]
pub extern "C" fn re_destroy(v_ptr: *mut c_void) {
    let re = unsafe { Box::from_raw(v_ptr.cast::<Regex>()) };

    println!("destroying regex: /{}/", re.as_str());
}

#[no_mangle]
pub extern "C" fn re_match(v_ptr: *mut c_void, content: *const c_char) -> c_int {
    let re: &Regex = unsafe { v_ptr.cast::<Regex>().as_ref().expect("regex was null") };

    let content: &str = unsafe {
        CStr::from_ptr(content)
            .to_str()
            .expect("content couldn not become a string slice")
    };

    if re.is_match(content) {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn re_find(v_ptr: *mut c_void, content: *const c_char) -> find_match {
    let re: &Regex = unsafe { v_ptr.cast::<Regex>().as_ref().expect("regex was null") };

    let content: &str = unsafe {
        CStr::from_ptr(content)
            .to_str()
            .expect("content could not become a string slice")
    };

    let Some(m) = re.find(content) else {
        return find_match {found: 0, start:0, end:0};
    };

    find_match {
        found: 1,
        start: m.start(),
        end: m.end(),
    }
}

#[no_mangle]
pub extern "C" fn re_captures(v_ptr: *mut c_void, content: *const c_char) -> *mut c_void {
    let re: &Regex = unsafe { v_ptr.cast::<Regex>().as_ref().expect("regex was null") };

    let content: &str = unsafe {
        CStr::from_ptr(content)
            .to_str()
            .expect("content couldn not become a string slice")
    };

    if let Some(caps) = re.captures(content) {
        Box::into_raw(Box::new(caps)).cast()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn re_captures_destroy(v_ptr: *mut c_void) {
    if v_ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(v_ptr.cast::<Captures>()) };

    println!("destroying captures");
}

#[no_mangle]
pub extern "C" fn re_capture_get(v_ptr: *mut c_void, number: usize) -> find_match {
    if let Some(ref m) = unsafe {
        v_ptr
            .cast::<Captures>()
            .as_ref()
            .expect("could not turn captures to ref")
    }
    .get(number)
    {
        find_match {
            found: 1,
            start: m.start(),
            end: m.end(),
        }
    } else {
        find_match {
            found: 0,
            start: 0,
            end: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn re_find_iter(v_ptr: *mut c_void, content: *const c_char) -> *mut c_void {
    std::unimplemented!()
}

#[no_mangle]
pub extern "C" fn re_destroy_find_iter(v_ptr: *mut c_void) {
    std::unimplemented!()
}

#[no_mangle]
pub extern "C" fn re_find_iter_next(v_ptr: *mut c_void) -> find_match {
    std::unimplemented!()
}

#[no_mangle]
pub extern "C" fn re_captures_iter(v_ptr: *mut c_void) {
    std::unimplemented!()
}

#[no_mangle]
pub extern "C" fn re_destroy_captures_iter(v_ptr: *mut c_void) {
    std::unimplemented!()
}

#[no_mangle]
pub extern "C" fn re_captures_iter_next(v_ptr: *mut c_void) -> *mut c_void {
    std::unimplemented!()
}
