use std::fmt::Debug;
use std::fs;

pub trait Sorter: Debug {
    fn sort(&self, entries: &mut Vec<fs::DirEntry>);
}

#[derive(Debug)]
pub struct DirsFirst;
#[derive(Debug)]
pub struct FilesFirst;

impl Sorter for DirsFirst {
    fn sort(&self, entries: &mut Vec<fs::DirEntry>) {
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

            b_is_dir.cmp(&a_is_dir)
        });
    }
}

impl Sorter for FilesFirst {
    fn sort(&self, entries: &mut Vec<fs::DirEntry>) {
        entries.sort_by(|a, b| {
            let a_is_file = a.file_type().map(|ft| ft.is_file()).unwrap_or(false);
            let b_is_file = b.file_type().map(|ft| ft.is_file()).unwrap_or(false);

            b_is_file.cmp(&a_is_file)
        });
    }
}
