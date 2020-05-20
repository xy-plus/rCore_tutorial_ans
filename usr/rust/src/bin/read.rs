#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

use user::io::*;
use user::syscall::{sys_close, sys_open, sys_read};

const BUFFER_SIZE: usize = 20;
const FILE: &'static str = "temp\0";
// const TEXT: &'static str = "Hello world!\0";

#[no_mangle]
pub fn main() -> usize {
    // 将字符串从文件 temp 读入内存
    let read_fd = sys_open(FILE.as_ptr(), O_RDONLY);
    let mut read = [0u8; BUFFER_SIZE];
    sys_read(read_fd as usize, &read[0] as *const u8, BUFFER_SIZE);
    println!("read from file 'temp' successfully...");

    // 检查功能是否正确
    let len = (0..BUFFER_SIZE).find(|&i| read[i] as u8 == 0).unwrap();
    print!("content = ");
    for i in 0usize..len {
        // assert!(read[i] == TEXT.as_bytes()[i]);
        putchar(read[i] as char);
    }
    putchar('\n');
    sys_close(read_fd as i32);
    0
}
