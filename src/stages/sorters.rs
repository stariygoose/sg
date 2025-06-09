use std::fmt::Debug;
use std::fs;

pub trait Sorter: Debug {
    fn sort(&self, entries: &mut Vec<fs::DirEntry>);
}

#[derive(Debug)]
pub struct DirsFirst;

impl Sorter for DirsFirst {
    fn sort(&self, entries: &mut Vec<fs::DirEntry>) {
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

            b_is_dir.cmp(&a_is_dir)
        });
    }
}
