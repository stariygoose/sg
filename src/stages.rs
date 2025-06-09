use std::fs;

pub mod filters;
use filters::Filter;

pub enum Stage {
    Filter(Box<dyn Filter>),
}

pub struct Stages {
    pub filters: Vec<Stage>,
}

#[derive(Debug)]
pub struct Controller {
    entries: Vec<fs::DirEntry>,
    filters: Vec<Box<dyn Filter>>,
}

impl Controller {
    pub fn new(entries: Vec<fs::DirEntry>) -> Self {
        Self {
            entries,
            filters: Vec::new(),
        }
    }

    pub fn register_stages(&mut self, stages: Stages) {
        stages.filters.into_iter().for_each(|f| self.register(f));
    }

    pub fn run(&mut self) {
        self.filters.iter().for_each(|f| {
            let old = std::mem::take(&mut self.entries);
            self.entries = f.apply(old);
        });

        println!("{:?}", self);
    }

    fn register(&mut self, stage: Stage) {
        match stage {
            Stage::Filter(filter) => self.filters.push(filter),
        }
    }
}
