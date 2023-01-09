extern crate core;

use std::arch::asm;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color { BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE, UNKNOWN, DEFAULT }

const TERMINAL_FD: u64 = 0;
pub const CONTROL_SEQ: &str = "\u{1b}[";
pub const MOVE_UP: char = 'A';
pub const MOVE_DOWN: char = 'B';
pub const MOVE_RIGHT: char = 'C';
pub const MOVE_LEFT: char = 'D';

const IOCTRL: u64 = 16;
const IOCTRL_TCGETS: u64 = 0x5401;
const IOCTRL_TCSETS: u64 = 0x5402;

#[derive(Debug, Default)]
#[repr(C)]
struct TermiosState {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

unsafe fn syscall(num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let res;
    asm!(
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    lateout("rax") res,
    );
    res
}

fn sys_tcgets(termios: *const TermiosState) -> i64 {
    unsafe { syscall(IOCTRL, TERMINAL_FD, IOCTRL_TCGETS, termios as u64) }
}

fn sys_tcsets(termios: *const TermiosState) -> i64 {
    unsafe { syscall(IOCTRL, TERMINAL_FD, IOCTRL_TCSETS, termios as u64) }
}

pub fn do_in_game_terminal(game_fn: fn()) {
    let mut termios: TermiosState = Default::default();

    if sys_tcgets(&termios as *const _) == 0 {
        let original_flags = termios.c_lflag;
        termios.c_lflag &= !0xA; // single key input, no echo of user input
        sys_tcsets(&termios as *const _);
        hide_cursor();

        game_fn();

        // reset both options
        show_cursor();
        termios.c_lflag = original_flags;
        sys_tcgets(&termios as *const _);
    } else {
        panic!("Failed to set-up proper environment within terminal, be sure to use a real (unix-like) terminal!");
    }
}


pub fn create_game_screen(screen_height: u8) {
    // write X empty lines, move cursor back to start and save its position
    for _ in 0..screen_height {
        overwrite();
        println!();
    }
    move_cursor_up(screen_height);
    save_cursor();
}

pub fn move_cursor_up(lines: u8) {
    print!("{}{}{}", CONTROL_SEQ, lines, MOVE_UP);
}

#[allow(dead_code)]
pub fn move_cursor_down(lines: u8) {
    print!("{}{}{}", CONTROL_SEQ, lines, MOVE_DOWN);
}

#[allow(dead_code)]
pub fn move_cursor_right(columns: u8) {
    print!("{}{}{}", CONTROL_SEQ, columns, MOVE_RIGHT);
}

#[allow(dead_code)]
pub fn move_cursor_left(columns: u8) {
    print!("{}{}{}", CONTROL_SEQ, columns, MOVE_LEFT);
}

pub fn show_cursor() {
    print!("{}?25h", CONTROL_SEQ);
}

pub fn hide_cursor() {
    print!("{}?25l", CONTROL_SEQ);
}

pub fn save_cursor() {
    print!("{}s", CONTROL_SEQ);
}

pub fn restore_cursor() {
    print!("{}u", CONTROL_SEQ);
}

pub fn overwrite() {
    print!("{}2K\r", CONTROL_SEQ);
}

pub fn line_in_color(msg: &str, color: Color) {
    overwrite();
    println!("{}", color!(msg, color));
}

#[macro_export]
macro_rules! color {
    ($msg:expr, $color:expr) => {
        format!("{}1;{}m{}{}0;39m", CONTROL_SEQ, 90 + $color as usize, $msg, CONTROL_SEQ)
    };
}

#[macro_export]
macro_rules! blink {
    ($msg:expr, $color:expr) => {
        format!("{}5;{}m{}{}0;39m", CONTROL_SEQ, 90 + $color as usize, $msg, CONTROL_SEQ)
    };
}

#[macro_export]
macro_rules! inverted {
    ($msg:expr, $color:expr) => {
        format!("{}7;{}m{}{}0;39m", CONTROL_SEQ, 30 + $color as usize, $msg, CONTROL_SEQ)
    };
}

pub(crate) use color;
