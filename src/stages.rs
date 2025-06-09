use std::fs;

pub mod filters;
pub mod outputs;
pub mod sorters;

use filters::Filter;
use outputs::Output;
use sorters::Sorter;

pub enum Stage {
    Filter(Box<dyn Filter>),
    Output(Box<dyn Output>),
    Sorter(Box<dyn Sorter>),
}

pub struct Stages {
    pub filters: Vec<Stage>,
    pub output: Stage,
    pub sorters: Vec<Stage>,
}

#[derive(Debug)]
pub struct Controller {
    entries: Vec<fs::DirEntry>,
    filters: Option<Vec<Box<dyn Filter>>>,
    sorters: Option<Vec<Box<dyn Sorter>>>,
    output: Option<Box<dyn Output>>,
}

impl Controller {
    pub fn new(entries: Vec<fs::DirEntry>) -> Self {
        Self {
            entries,
            filters: None,
            sorters: None,
            output: None,
        }
    }

    pub fn register(&mut self, stages: Stages) {
        let mut filters: Vec<Box<dyn Filter>> = Vec::new();
        let mut sorters: Vec<Box<dyn Sorter>> = Vec::new();

        for stage in stages.filters {
            match stage {
                Stage::Filter(f) => filters.push(f),
                _ => panic!("Unexpected output stage in filters vector"),
            }
        }

        for stage in stages.sorters {
            match stage {
                Stage::Sorter(s) => sorters.push(s),
                _ => panic!("Unexpected output stage in sorters vector"),
            }
        }

        self.filters = Some(filters);
        self.sorters = Some(sorters);

        match stages.output {
            Stage::Output(o) => self.output = Some(o),
            _ => panic!("Expected output stage, got filter"),
        }
    }

    pub fn build(self) -> Controller {
        Controller {
            entries: self.entries,
            filters: self.filters,
            sorters: self.sorters,
            output: self.output,
        }
    }

    pub fn run(&mut self) {
        match &self.filters {
            Some(filters) => {
                for filter in filters {
                    filter.apply(&mut self.entries);
                }
            }
            None => panic!("No filters registered"),
        }

        match &self.sorters {
            Some(sorters) => {
                for sorter in sorters {
                    sorter.sort(&mut self.entries);
                }
            }
            None => panic!("No sorters registered"),
        }

        match &self.output {
            Some(output) => {
                output.print(&self.entries);
            }
            None => panic!("No output registered"),
        }
    }
}
