extern crate kernel32;
extern crate winapi;

use std::io::{Error, Result};
use std::fmt;

use self::winapi::minwindef::FALSE;
use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::tlhelp32::TH32CS_SNAPPROCESS;
use self::winapi::winerror::ERROR_NO_MORE_FILES;

use process_entry::ProcessEntry;
use toolhelp_32_snapshot_handle::Toolhelp32SnapshotHandle;

pub struct ProcessIterator {
    handle: Toolhelp32SnapshotHandle,
    first: Option<ProcessEntry>,
}

impl ProcessIterator {
    pub fn new() -> Result<ProcessIterator> {
        let handle = unsafe { kernel32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }
        let handle = Toolhelp32SnapshotHandle(handle);

        let mut entry = ProcessEntry::new();
        unsafe {
            if kernel32::Process32FirstW(handle.0, &mut entry.0) == FALSE {
                return Err(Error::last_os_error());
            }
        }

        Ok(ProcessIterator {
            handle: handle,
            first: Some(entry),
        })
    }
}

impl Iterator for ProcessIterator {
    type Item = Result<ProcessEntry>;

    fn next(&mut self) -> Option<Result<ProcessEntry>> {
        match self.first.take() {
            Some(process_entry) => Some(Ok(process_entry)),
            None => unsafe {
                let mut entry = ProcessEntry::new();
                if kernel32::Process32NextW(self.handle.0, &mut entry.0) == FALSE {
                    if kernel32::GetLastError() == ERROR_NO_MORE_FILES {
                        None
                    } else {
                        Some(Err(Error::last_os_error()))
                    }
                } else {
                    Some(Ok(entry))
                }
            },
        }
    }
}

impl fmt::Debug for ProcessIterator {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
               "ProcessIterator {{ handle: {:?}, first: {} }}",
               self.handle,
               match self.first {
                   Some(_) => "Some(_)",
                   None => "None",
               })
    }
}