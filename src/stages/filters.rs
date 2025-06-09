use std::fmt::Debug;
use std::fs;

pub trait Filter: Debug {
    fn apply(&self, entries: &mut Vec<fs::DirEntry>);
}

#[derive(Debug)]
pub struct AllFiles;

#[derive(Debug)]
pub struct WithOutHiddenFiles;

impl Filter for AllFiles {
    fn apply(&self, _entries: &mut Vec<fs::DirEntry>) {}
}

impl Filter for WithOutHiddenFiles {
    fn apply(&self, entries: &mut Vec<fs::DirEntry>) {
        entries.retain(|file| {
            let name = file.file_name();
            let name = name.to_string_lossy();
            !name.starts_with('.')
        });
    }
}
