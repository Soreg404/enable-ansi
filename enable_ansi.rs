unsafe extern "system" {
    fn GetStdHandle(_:u32) -> *const ();
    fn GetConsoleMode(_:*const (), _:*mut u32) -> i32;
    fn SetConsoleMode(_:*const (), _:u32) -> i32;
}
pub fn enable_ansi() {
    unsafe {
        let h = GetStdHandle(u32::MAX - 10);
        let mut mode = 0;
        if GetConsoleMode(h, &mut mode) != 0 {
            SetConsoleMode(h, mode | 0x0004);
        }
    }
}

