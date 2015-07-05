extern crate kernel32;
extern crate winapi;

mod string;

fn main() {
    let job = kernel32::CreateJobObjectW(ptr::null_mut(), ptr::null_mut());
    if job.is_null() {
        panic!("CreateJobObjectW error: {}", kernel32::GetLastError());
    }

    let this = kernel32::GetCurrentProcess();
    let status = kernel32::AssignProcessToJobObject(jon, this);
    if status == 0 {
        panic!("AssignProcessToJobObject error: {}", kernel32::GetLastError());
    }

    // TODO: GetCommandLine
    // TODO: CreateProcess
    // TODO: WaitForSingleObject
}
