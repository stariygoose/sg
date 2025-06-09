use std::{fs, path::PathBuf};

use clap::{ArgAction, Parser};

mod stages;
use stages::{Controller, Stage, Stages, filters};

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

fn main() {
    let args = Args::parse();

    let entries = match fs::read_dir(args.path) {
        Ok(entries) => entries,
        Err(e) => panic!("Error: {}", e),
    };
    let entries = entries.filter_map(Result::ok).collect();

    let mut controller = Controller::new(entries);

    let mut filters = vec![Stage::Filter(Box::new(filters::WithOutHiddenFiles))];

    if args.all {
        filters.remove(0);
        filters.push(Stage::Filter(Box::new(filters::AllFiles)));
    }

    let stages = Stages { filters };

    controller.register_stages(stages);
    controller.run();
}
