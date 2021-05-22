use super::WSTR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem::size_of;


pub trait ToWinStr {
    fn to_win_str(&self) -> WSTR;
}

pub trait WinStrSize {
    fn sizeof(&self) -> usize;
}


impl ToWinStr for str {
    fn to_win_str(&self) -> WSTR {
        OsStr::new(self)
            .encode_wide()
            .collect::<WSTR>()
    }
}

impl WinStrSize for WSTR {
    fn sizeof(&self) -> usize {
        size_of::<u16>() * self.len()
    }
}
