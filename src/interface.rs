use crate::error::りさると;

use std::{
    ffi::{c_char, c_long},
    slice,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load(h: *const c_char, len: c_long) -> bool {
    if let Ok(request) = unsafe { ptr_to_string(h, len) }{
        todo!();
        true
    }else {
        false
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload() -> bool {
true
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request(h: *const c_char, len: *mut c_long) -> *const c_char {
    if let Ok(request) = unsafe { ptr_to_string(h, *len) }{
        todo!()
    }else {
        todo!()
    }
}
unsafe fn ptr_to_string(ptr: *const c_char, len: c_long) -> りさると<String> {
    if ptr.is_null() {
        return Ok(String::new());
    }
    // ポインタからスライスを作成するにゃん
    let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, len.try_into()?) };

    // スライスをUTF-8として解釈し、Stringに変換するにゃん
    Ok(String::from_utf8(bytes.to_vec())?)
}
