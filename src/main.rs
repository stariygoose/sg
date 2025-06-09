use std::{fs, path::PathBuf};

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action = ArgAction::SetTrue)]
    all: bool,

    #[arg(short = 'H', long, action = ArgAction::SetTrue)]
    hidden: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    files: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    directories: bool,

    #[arg(default_value = ".")]
    path: PathBuf,
}

struct Pipeline {
    entries: Vec<fs::DirEntry>,
}

impl Pipeline {
    fn new(path: PathBuf) -> Result<Pipeline, std::io::Error> {
        let files = fs::read_dir(path)?.filter_map(Result::ok).collect();
        Ok(Self { entries: files })
    }

    fn pipe<F>(&mut self, condition: bool, f: F) -> &mut Self
    where
        F: Fn(Vec<fs::DirEntry>) -> Vec<fs::DirEntry>,
    {
        if condition {
            let old_entries = std::mem::take(&mut self.entries);
            self.entries = f(old_entries);
        }
        self
    }
}

fn all(entries: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
    entries
}

fn directories(entries: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
    entries.into_iter().filter(|f| f.path().is_dir()).collect()
}

fn show_files(entries: Vec<fs::DirEntry>) {
    for f in entries {
        let file = f.file_name();
        let file = file.to_string_lossy();
        println!("{}", file);
    }
}

fn main() {
    let args = Args::parse();

    if let Ok(mut pipeline) = Pipeline::new(args.path) {
        pipeline
            .pipe(args.all, all)
            .pipe(args.directories, directories);

        show_files(pipeline.entries);
    } else {
        eprintln!("Error");
    }
}
