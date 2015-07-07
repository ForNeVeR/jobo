extern crate kernel32;
extern crate winapi;

use std::ptr;
use winapi::*;

mod command_line;
mod string;

fn main() {
    unsafe {
        let job = kernel32::CreateJobObjectW(ptr::null_mut(), ptr::null_mut());
        if job.is_null() {
            panic!("CreateJobObjectW error: {}", kernel32::GetLastError());
        }

        let this = kernel32::GetCurrentProcess();
        let status = kernel32::AssignProcessToJobObject(job, this);
        if status == 0 {
            panic!("AssignProcessToJobObject error: {}", kernel32::GetLastError());
        }

        let command_line = command_line::get_arguments();

        // TODO: CreateProcess
        // TODO: WaitForSingleObject
    }
}
