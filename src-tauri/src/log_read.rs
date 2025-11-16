use std::{
    cell::{LazyCell, OnceCell},
    collections::HashMap,
    path::PathBuf,
};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Record {
    level: String,
    target: String,
    message: String,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
    data: HashMap<String, Value>,
}

#[derive(Debug, Default)]
pub struct LogState {
    file: Option<PathBuf>,
    data: OnceCell<Option<Vec<Record>>>,
}

impl LogState {
    pub fn set_file(&mut self, file: PathBuf) {
        self.file = Some(file);
        self.data = OnceCell::new();
    }

    pub fn data(&self) -> Option<&[Record]> {
        self.data
            .get_or_init(|| {
                Some(
                    std::fs::read_to_string(self.file.as_ref()?)
                        .inspect_err(|e| log::warn!("{e:?}"))
                        .ok()?
                        .lines()
                        .filter_map(|line| {
                            serde_json::from_str(line)
                                .inspect_err(|e| log::warn!("{e:?}"))
                                .ok()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .as_ref()
            .map(Vec::as_slice)
    }
}
