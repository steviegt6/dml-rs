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
pub extern "C" fn display_mouse_lock(win_x: f64, win_y: f64, win_width: f64, win_height: f64) {
    let mut rect = RECT {
        left: win_x as i32,
        top: win_y as i32,
        right: (win_x + win_width) as i32,
        bottom: (win_y + win_height) as i32,
    };

    // so sad unsafe
    unsafe {
        winapi::um::winuser::ClipCursor(&mut rect);
    }
}

#[no_mangle]
pub extern "C" fn display_mouse_unlock() {
    // so sad unsafe
    unsafe {
        winapi::um::winuser::ClipCursor(std::ptr::null_mut());
    }
}

/*fn lpcwstr(s: &str) -> *const u16 {
    s.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()
}*/
