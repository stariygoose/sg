use std::fmt::Debug;
use std::fs;

pub trait Output: Debug {
    fn print(&self, entries: &[fs::DirEntry]);
    fn convert(&self, file: &fs::DirEntry) -> Result<String, std::io::Error> {
        let name = file.file_name().to_string_lossy().into_owned();
        match file.metadata() {
            Ok(meta) if meta.is_dir() => Ok(format!("{}/", name)),
            Ok(_) => Ok(name),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct InlineOutput;

impl Output for InlineOutput {
    fn print(&self, entries: &[fs::DirEntry]) {
        entries.iter().for_each(|f: &fs::DirEntry| {
            if let Ok(name) = self.convert(f) {
                println!("{}", name);
            }
        })
    }
}
