use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    _call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    TRUE
}
