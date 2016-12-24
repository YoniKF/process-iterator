extern crate kernel32;
extern crate winapi;

use std::io::{Error, Result};
use std::fmt;
use std::mem;
use std::os::windows::raw::HANDLE;
use std::path::PathBuf;

use winapi::minwindef::{DWORD, MAX_PATH};
use winapi::shlobj::INVALID_HANDLE_VALUE;
use winapi::tlhelp32::{PROCESSENTRY32W, TH32CS_SNAPPROCESS};

pub struct ProcessIterator {
    handle: Toolhelp32SnapshotHandle,
    first: Option<PROCESSENTRY32W>,
}

#[derive(Debug)]
struct Toolhelp32SnapshotHandle(HANDLE);

impl ProcessIterator {
    pub fn new() -> Result<ProcessIterator> {
        let handle = unsafe { kernel32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0 as DWORD) };
        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: mem::size_of::<PROCESSENTRY32W>() as DWORD,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; MAX_PATH],
        };
        unsafe {
            if kernel32::Process32FirstW(handle, &mut entry) == 0 {
                return Err(Error::last_os_error());
            }
        }

        Ok(ProcessIterator {
            handle: Toolhelp32SnapshotHandle(handle),
            first: Some(entry),
        })
    }
}

impl Iterator for ProcessIterator {
    type Item = Result<ProcessEntry>;

    fn next(&mut self) -> Option<Result<ProcessEntry>> {
        unimplemented!();
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

pub struct ProcessEntry;

impl ProcessEntry {
    pub fn process_id(&self) -> u32 {
        unimplemented!();
    }

    pub fn threads(&self) -> u32 {
        unimplemented!();
    }

    pub fn parent_process_id(&self) -> u32 {
        unimplemented!();
    }

    pub fn base_priority(&self) -> i32 {
        unimplemented!();
    }

    pub fn executable_path(&self) -> PathBuf {
        unimplemented!();
    }
}