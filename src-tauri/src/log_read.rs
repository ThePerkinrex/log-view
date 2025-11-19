use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Default, Serialize, Clone)]
pub struct RecordExtra {
    pub task_id: Option<String>,
    pub process_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    pub level: String,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub data: HashMap<String, Value>,
    #[serde(default)]
    pub extra: RecordExtra,
}

#[derive(Debug, Default)]
pub struct LogState {
    file: Option<PathBuf>,
    data: OnceLock<Option<Vec<Record>>>,
}

impl LogState {
    pub fn set_file(&mut self, file: PathBuf) {
        self.file = Some(file);
        self.data = OnceLock::new();
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
