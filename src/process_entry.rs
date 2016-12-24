use std::path::PathBuf;

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