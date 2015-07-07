extern crate kernel32;
extern crate winapi;

use std::env;
use winapi::*;

pub fn get_arguments() -> LPCWSTR {
    let command_line = unsafe { kernel32::GetCommandLineW() };
    let mut args = env::args_os();
    println!("{:?}", args.nth(0));

    command_line
}
