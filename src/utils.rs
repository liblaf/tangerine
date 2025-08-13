use color_eyre::Result;
use std::path::{Path, PathBuf};

const PATTERNS: &[&str] = &[
    ".config/copier/.copier-answers.*.yaml",
    // ".config/copier/.copier-answers.*.yml",
    // ".config/copier/.copier-answers.yaml",
    // ".config/copier/.copier-answers.yml",
    // ".copier-answers.*.yaml",
    // ".copier-answers.*.yml",
    // ".copier-answers.yaml",
    // ".copier-answers.yml",
];

#[tracing::instrument(err, level = "trace", ret)]
pub fn git_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let cwd = match gix_discover::upwards(&cwd) {
        Ok((path, _)) => match path {
            gix_discover::repository::Path::LinkedWorkTree { work_dir, .. } => work_dir,
            gix_discover::repository::Path::WorkTree(root) => root,
            gix_discover::repository::Path::Repository(root) => root,
        },
        Err(err) => {
            tracing::error!("{}", err);
            cwd
        }
    };
    Ok(cwd)
}

#[tracing::instrument(err, level = "trace", ret)]
pub fn load_copier_answers() -> Result<minijinja::Value> {
    let cwd = git_root()?;
    let mut values: Vec<minijinja::Value> = Vec::new();
    for pattern in PATTERNS {
        let pattern = cwd.join(pattern);
        let pattern = pattern.to_str().unwrap();
        for path in glob::glob(pattern)? {
            match path {
                Ok(path) => match load_yaml(&path) {
                    Ok(value) => values.push(value),
                    Err(err) => tracing::error!("{}", err),
                },
                Err(err) => tracing::error!("{}", err),
            }
        }
    }
    Ok(minijinja::value::merge_maps(values))
}

fn load_yaml(path: &Path) -> Result<minijinja::Value> {
    let reader = std::fs::File::open(path)?;
    let value = serde_yml::from_reader(reader)?;
    Ok(value)
}
