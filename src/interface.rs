use crate::error::りさると;
use encoding_rs;
use libc::strlen;
use std::{
    ffi::{CString, c_char, c_long},
    ptr::null_mut,
    slice,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loadu(h: *const u8, len: c_long) -> bool {
    if let Ok(path) = unsafe { ptr_to_string(h, len) } {
        crate::init::init(&path)
    } else {
        false
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load(h: *const u8, len: c_long) -> bool {
    unsafe { loadu(h, len) }
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
        unsafe { *len = 0 };
        null_mut() as *const c_char
    }
}
unsafe fn ptr_to_string(h: *const u8, len: c_long) -> りさると<String> {
    let ptr = h ;
    if ptr.is_null() {
        return Ok(String::new());
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len.try_into()?) }
        .to_vec()
        .clone();
    Ok(match String::from_utf8(bytes.clone()) {
        Ok(a) => a,
        Err(_) => {
            let (a, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
            a.to_owned().to_string()
        }
    })
}
unsafe fn string_to_ptr(input: String) -> りさると<(*const c_char, c_long)> {
    let c_string = CString::new(input)?;
    let ptr = c_string.into_raw();
    let len = unsafe { strlen(ptr) } as c_long;
    Ok((ptr, len))
}
