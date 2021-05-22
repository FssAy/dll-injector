mod ax;

use ax::*;
use libc::*;
use std::ptr::null_mut as NULL;
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::{
    PROCESS_ALL_ACCESS,
    MEM_COMMIT,
    PAGE_EXECUTE_READWRITE,
};


pub unsafe fn inject(dll: String, pid: DWORD) {
    let h_process = OpenProcess(
        PROCESS_ALL_ACCESS,
        0,
        pid,
    );

    let dll_wstr = dll.to_win_str();
    let kernel32_wstr = "kernel32.dll\0".to_win_str();
    let dll_size = dll_wstr.sizeof() as DWORD;

    let v_mem = VirtualAllocEx(
        h_process,
        NULL(),
        dll_size,
        MEM_COMMIT,
        PAGE_EXECUTE_READWRITE,
    );

    WriteProcessMemory(
        h_process,
        v_mem,
        dll_wstr.as_ptr() as *const c_void,
        dll_size,
        NULL(),
    );

    let kernel32_lib = LoadLibraryW(kernel32_wstr.as_ptr());
    let addr_load_library = GetProcAddress(
        kernel32_lib,
        b"LoadLibraryW\0".as_ptr(),
    );

    let mut thread_id = 0_u32;
    let remote_thread = CreateRemoteThread(
        h_process,
        NULL(),
        0,
        addr_load_library,
        v_mem,
        0,
        &mut thread_id,
    );

    WaitForSingleObject(remote_thread, INFINITE);

    FreeLibrary(kernel32_lib);
    CloseHandle(remote_thread);
    CloseHandle(h_process);
}
