use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Console::{
    GetStdHandle,
    SetConsoleCursorInfo,
    SetConsoleCursorPosition,
    CONSOLE_CURSOR_INFO,
    COORD,
    STD_OUTPUT_HANDLE
};

pub fn goto_xy(x: usize, y: usize) {
    let hcon = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)};
    let hcon = hcon.unwrap();
    let dwpos = COORD {
        X: (x as i16),
        Y: (y as i16)
    };
    unsafe{SetConsoleCursorPosition(hcon, dwpos)};
}

pub fn hide_cursor() {
    let hcon = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)};
    let hcon = hcon.unwrap();
    let cci = CONSOLE_CURSOR_INFO { dwSize: 50, bVisible: BOOL(0) };
    unsafe{SetConsoleCursorInfo(hcon, &cci)};
}