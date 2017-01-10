extern crate kernel32;
extern crate winapi;

use std::io::{Error, Result};
use std::os::windows::raw::HANDLE;

use self::winapi::minwindef::FALSE;

use process_entry::ProcessEntry;

#[derive(Debug)]
pub struct Toolhelp32SnapshotHandle(HANDLE);

impl Toolhelp32SnapshotHandle {
    pub fn new(handle: HANDLE) -> Toolhelp32SnapshotHandle {
        Toolhelp32SnapshotHandle(handle)
    }

    pub fn first(&mut self) -> Result<ProcessEntry> {
        let mut entry: ProcessEntry = Default::default();
        if unsafe { kernel32::Process32FirstW(self.0, entry.raw()) } == FALSE {
            Err(Error::last_os_error())
        } else {
            Ok(entry)
        }
    }

    pub fn next(&mut self) -> Result<ProcessEntry> {
        let mut entry: ProcessEntry = Default::default();
        if unsafe { kernel32::Process32NextW(self.0, entry.raw()) } == FALSE {
            Err(Error::last_os_error())
        } else {
            Ok(entry)
        }
    }
}

impl Drop for Toolhelp32SnapshotHandle {
    fn drop(&mut self) {
        debug_assert_ne!(unsafe { kernel32::CloseHandle(self.0) }, FALSE);
    }
}