#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod wstr;

pub use wstr::*;
use libc::*;


pub type LPVOID = *const c_void;
pub type HANDLE = LPVOID;
pub type HMODULE = LPVOID;

pub type LPWSTR = *const u16;
pub type LPCSTR = *const u8;
pub type WSTR = Vec<u16>;

pub type DWORD = u32;
pub type LPDWORD = *mut DWORD;
pub type SIZE_T = DWORD;
pub type BOOL = i32;

pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPTHREAD_START_ROUTINE = extern "system" fn(lpThreadParameter: LPVOID);


#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL
}


#[link(name = "kernel32")]
extern "system" {
    pub fn OpenProcess(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwProcessId: DWORD
    ) -> HANDLE;

    pub fn VirtualAllocEx(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: DWORD,
        flAllocationType: DWORD,
        flProtect: DWORD
    ) -> LPVOID;

    pub fn WriteProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPVOID,
        lpBuffer: LPVOID,
        nSize: DWORD,
        lpNumberOfBytesWritten: LPDWORD
    ) -> BOOL;

    pub fn LoadLibraryW(
        lpLibFileName: LPWSTR
    ) -> HMODULE;

    pub fn GetProcAddress(
        hModules: HMODULE,
        lpProcName: LPCSTR
    ) -> LPTHREAD_START_ROUTINE;

    pub fn CreateRemoteThread(
        hProcess: HANDLE,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: LPDWORD
    ) -> HANDLE;

    pub fn WaitForSingleObject(
        hHandle: HANDLE,
        dwMilliseconds: DWORD
    ) -> DWORD;

    pub fn FreeLibrary(
        hLibModule: HMODULE
    );

    pub fn CloseHandle(
        hObject: HANDLE
    );
}
