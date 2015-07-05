extern crate kernel32;
extern crate winapi;

use std::ptr;
use winapi::*;

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

        let command_line = kernel32::GetCommandLineW();
        let output = kernel32::GetStdHandle(winbase::STD_OUTPUT_HANDLE);
        let mut written: DWORD = 0;
        let writeResult = kernel32::WriteConsoleW(
            output,
            command_line as *const std::os::raw::c_void,
            kernel32::lstrlenW(command_line) as u32,
            &mut written,
            ptr::null_mut()
        );
        if writeResult == 0 {
            panic!("WriteConsoleW error: {}", kernel32::GetLastError());
        }

        // TODO: CreateProcess
        // TODO: WaitForSingleObject
    }
}
