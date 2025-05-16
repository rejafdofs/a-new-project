use crate::error::りさると;
use std::{
    ffi::{c_char, c_long},
    slice,
};
use windows::{
    Win32::Foundation::{GlobalFree, HGLOBAL},
    core::Free,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loadu(h: *const u8, len: c_long) -> bool {
    if let Ok(request) = unsafe { ptr_to_string(h, len) } {
        todo!();
        true
    } else {
        false
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload() -> bool {
    true
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request(h: *const u8, len: *mut c_long) -> *const c_char {
    if let Ok(request) = unsafe { ptr_to_string(h, *len) } {
        todo!()
    } else {
        todo!()
    }
}
unsafe fn ptr_to_string(h: *const u8, len: c_long) -> りさると<String> {
    let ptr = h as *const u8;
    if ptr.is_null() {
        return Ok(String::new());
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len.try_into()?) }
        .to_vec()
        .clone();
    Ok(String::from_utf8(bytes)?)
}
unsafe fn string_to_ptr(input: String) -> りさると<(*const c_char, c_long)> {
    todo!()
}
