use std::path::Path;

use color_eyre::Result;

const PATTERNS: [&str; 4] = [
    ".copier-answers.*.yaml",
    ".copier-answers.*.yml",
    ".copier-answers.yaml",
    ".copier-answers.yml",
];

pub fn load_copier_answers() -> Result<minijinja::Value> {
    let cwd = match git2::Repository::discover(".") {
        Ok(repo) => repo.workdir().unwrap().to_path_buf(),
        Err(err) => {
            tracing::error!("{}", err);
            std::env::current_dir().unwrap()
        }
    };
    let mut values: Vec<minijinja::Value> = vec![];
    for pattern in PATTERNS {
        let pattern = cwd.join("**").join(pattern);
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
