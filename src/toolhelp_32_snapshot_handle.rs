extern crate kernel32;
extern crate winapi;

use std::os::windows::raw::HANDLE;

use self::winapi::minwindef::FALSE;

#[derive(Debug)]
pub struct Toolhelp32SnapshotHandle(pub HANDLE);

impl Drop for Toolhelp32SnapshotHandle {
    fn drop(&mut self) {
        debug_assert_ne!(unsafe { kernel32::CloseHandle(self.0) }, FALSE);
    }
}