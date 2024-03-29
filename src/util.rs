use std::fs;
use std::path::PathBuf;

pub(crate) const fn get_version() -> &'static str {
    return env!("CARGO_PKG_VERSION");
}

pub fn get_instances_path() -> Option<PathBuf> {
    let mut path = get_path()?;
    path.push("instances");
    return Some(path);
}

pub fn get_versions_path() -> Option<PathBuf> {
    let mut path = get_path()?;
    path.push("versions");
    return Some(path);
}

fn get_path() -> Option<PathBuf> {
    if cfg!(windows) {
        return std::env::current_exe().ok().and_then(|mut path| {
            path.pop();
            if !path.exists() {
                fs::create_dir_all(path.clone()).expect("Failed to create netherite directory");
            }
            return Some(path);
        });
    }
    return match std::env::var("HOME") {
        Ok(home) => {
            let mut path = PathBuf::from(home);
            path.push(".local/share/netherite");
            fs::create_dir_all(path.clone()).expect("Failed to create netherite directory");
            Some(path)
        }
        Err(_) => None
    }
}