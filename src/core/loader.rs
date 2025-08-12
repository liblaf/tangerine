pub fn loader(name: &str) -> Result<Option<String>, minijinja::Error> {
    let dirs = directories::ProjectDirs::from("", "liblaf", "tangerine").unwrap();
    let loader = minijinja::path_loader(dirs.data_dir().join("templates"));
    loader(name)
}
