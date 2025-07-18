use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

#[derive(Debug)]
pub struct Module {
    pub path: PathBuf,
}

impl Module {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Module {
            path: path.as_ref().into(),
        })
    }
}
