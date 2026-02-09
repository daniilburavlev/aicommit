use std::env::home_dir;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn write_data(path: &str, data: &str) -> bool {
    let file_path = Path::new(path);
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir).ok();
    }
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(path)
        && file.write_all(data.as_bytes()).is_err()
    {
        return false;
    }
    true
}

pub fn read_data(path: &str) -> Option<String> {
    if let Ok(mut file) = OpenOptions::new().read(true).open(path) {
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok() {
            return Some(data);
        }
    }
    None
}

pub fn get_home_path(path: &str) -> Option<String> {
    let mut home_dir = home_dir()?;
    home_dir.push(path);
    home_dir.as_path().to_str().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn home_path() {
        assert!(get_home_path("test").is_some());
    }

    #[test]
    fn write_read() {
        let tmpfile = NamedTempFile::new().unwrap();
        let data = "test";
        assert!(write_data(tmpfile.path().to_str().unwrap(), data));

        let saved = read_data(tmpfile.path().to_str().unwrap()).unwrap();
        assert_eq!(saved, data);
    }
}
