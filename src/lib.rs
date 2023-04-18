#![no_std]
#![no_main]

#[cfg(windows)]
extern crate winapi;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::shared::windef::RECT;

// https://github.com/rust-lang/rust-analyzer/issues/4490#issuecomment-1074922003
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Define _fltused and _DllMainCRTStartup to avoid linking errors,
// Enabling no_std seems to expect _DllMainCRTStartup and _fltused instead of
// just DllMain, so yeah.
// Relevant: https://github.com/Trantect/win_driver_example/issues/4

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
pub unsafe extern "C" fn display_mouse_lock(x: f64, y: f64, width: f64, height: f64) {
    let mut rect = RECT {
        left: x as i32,
        top: y as i32,
        right: (x + width) as i32,
        bottom: (y + height) as i32,
    };

    winapi::um::winuser::ClipCursor(&mut rect);
}

#[no_mangle]
pub unsafe extern "C" fn display_mouse_unlock() {
    winapi::um::winuser::ClipCursor(0 as *const RECT);
}
