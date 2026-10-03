#![deny(warnings)]

/// enables virtual terminal processing (ansi collored output)
/// and returns old mode
pub fn enable_ansi() -> DWORD {
    let mut mode = 0;
    unsafe {
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        if GetConsoleMode(h, &mut mode) != 0 {
            SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
    mode
}

/// [Windows data types](https://learn.microsoft.com/pl-pl/windows/win32/winprog/windows-data-types)
#[expect(dead_code)]
mod literal_winapi_types {
    use std::ffi;
    type HANDLE  = *const ffi::c_void;
    type DWORD   = ffi::c_ulong;
    type BOOL    = ffi::c_int;
    type LPDWORD = *const DWORD;
}

/// winapi types translated to rust types 
type HANDLE  = *const ();
type DWORD   = u16;
type BOOL    = i32;
type LPDWORD = *const DWORD;

unsafe extern "system" {
    fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    fn GetConsoleMode(hConsoleHandle: HANDLE, dwMode: LPDWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

/// WinApi needed constants
/// This one is tricky / hacky /sloppy / ofc
/// refer to the warning on the [microslop docs page][^doc1]
/// [^doc1]: <https://learn.microsoft.com/en-us/windows/console/getstdhandle>
const STD_OUTPUT_HANDLE: DWORD = (u32::MAX - 10) as DWORD;

/// from [microslop docs];
/// please NOTE the *if the hConsoleHandle parameter is ...* on the above page.
/// also, in Windows.h this is probably a #define so the `: DWORD` type is Rust specific.
/// [^doc2]: <https://learn.microsoft.com/en-us/windows/console/setconsolemode>
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x4;
