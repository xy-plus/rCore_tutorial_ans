#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::io::*;
use user::syscall::{sys_close, sys_open, sys_write};

const FILE: &'static str = "temp\0";
const TEXT: &'static str = "Hello world!\0";

#[no_mangle]
pub fn main() -> usize {
    // 将字符串写到文件 temp 中
    let write_fd = sys_open(FILE.as_ptr(), O_WRONLY);
    sys_write(write_fd as usize, TEXT.as_ptr(), TEXT.len());
    println!("write to file 'temp' successfully...");
    sys_close(write_fd as i32);
    0
}
