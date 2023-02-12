#![no_std]
#[cfg(windows)]
extern crate winapi;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::shared::windef::RECT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[used]
#[no_mangle]
static _fltused: i32 = 0;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn _DllMainCRTStartup(
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
