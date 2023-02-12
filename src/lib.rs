#![no_std]

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::shared::windef::RECT;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    _call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn display_mouse_lock(x: f64, y: f64, width: f64, height: f64) {
    let mut rect = RECT {
        left: x as i32,
        top: y as i32,
        right: (x + width) as i32,
        bottom: (y + height) as i32,
    };

    unsafe {
        winapi::um::winuser::ClipCursor(&mut rect);
    }
}

#[no_mangle]
pub extern "C" fn display_mouse_unlock() {
    unsafe {
        winapi::um::winuser::ClipCursor(0 as *const RECT);
    }
}

/*fn lpcwstr(s: &str) -> *const u16 {
    s.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()
}*/
