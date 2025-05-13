use std::ffi::c_long;
use ::windows::Win32::Foundation::HGLOBAL;

#[unsafe(no_mangle)]
pub extern "C" fn load(h: HGLOBAL, len: c_long) -> bool {
    todo!()
}
pub extern "C" fn unload() -> bool {
    todo!()
}
pub extern "C" fn request(h: HGLOBAL, len: *const c_long) -> HGLOBAL {
    let _ = len;
    todo!()
}
