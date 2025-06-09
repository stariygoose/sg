use std::fmt::Debug;
use std::fs;

pub trait Filter: Debug {
    fn apply(&self, entries: Vec<fs::DirEntry>) -> Vec<fs::DirEntry>;
}

#[derive(Debug)]
pub struct AllFiles;

#[derive(Debug)]
pub struct WithOutHiddenFiles;

impl Filter for AllFiles {
    fn apply(&self, entries: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
        entries
    }
}

impl Filter for WithOutHiddenFiles {
    fn apply(&self, entries: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
        entries
            .into_iter()
            .filter(|f| !f.file_name().to_string_lossy().starts_with("."))
            .collect()
    }
}
