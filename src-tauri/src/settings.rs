use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub recent: Vec<PathBuf>,
}

impl Settings {
    pub fn load() -> Self {
        std::fs::File::open(PROJECT_DIRS.settings_file())
            .ok()
            .and_then(|file| serde_json::from_reader(file).ok())
            .unwrap_or_default()
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        std::fs::File::create(PROJECT_DIRS.settings_file())
            .map(|file| serde_json::to_writer_pretty(file, self).unwrap())
            .unwrap();
    }
}

fn default_local_path() -> &'static Path {
    Path::new(".log-view-settings/")
}

fn create_dir_and_return<T: AsRef<Path>>(data: T) -> Result<T, std::io::Error> {
    std::fs::create_dir_all(data.as_ref()).map(|_| data)
}

#[derive(Debug, Default)]
pub enum Dirs {
    Proj(Box<ProjectDirs>),
    #[default]
    FromBase,
}

impl Dirs {
    // pub fn project_path(&self) -> &Path {
    //     match self {
    //         Self::FromBase => default_local_path(),
    //         Self::Proj(p) => p.project_path(),
    //     }
    // }

    pub fn config_dir(&self) -> Cow<'_, Path> {
        create_dir_and_return(match self {
            Self::Proj(project_dirs) => Cow::Borrowed(project_dirs.config_dir()),
            Self::FromBase => Cow::Owned(default_local_path().join("config")),
        })
        .expect("create config dir")
    }

    pub fn settings_file(&self) -> PathBuf {
        self.config_dir().join("settings.json")
    }
}

static PROJECT_DIRS: LazyLock<Dirs> = LazyLock::new(|| {
    ProjectDirs::from("io.github", "theperkinrex", "log-view")
        .map(Box::new)
        .map(Dirs::Proj)
        .unwrap_or_default()
});
