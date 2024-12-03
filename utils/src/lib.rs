pub mod input {
    use std::env;
    use std::fs;
    use std::path;

    pub fn read_file(filename: &str) -> String {
        let dir = match env::var("CARGO_MANIFEST_DIR") {
            Ok(dir) => path::PathBuf::from(dir),
            Err(_) => env::current_dir().expect("Failed to get current directory"),
        };
        let path = dir.join(filename);
        fs::read_to_string(&path).expect(&format!("Failed to read file {}", path.display()))
    }
}
