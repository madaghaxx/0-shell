use std::env;
use std::path::PathBuf;

pub fn require_min_args(cmd: &str, args: &[&str], n: usize, msg: &str) -> bool {
    if args.len() < n {
        println!("{}: {}", cmd, msg);
        return false;
    }
    true
}

pub fn expand_path(path: &str) -> PathBuf {
    let path = if path.starts_with("~/") {
        if let Some(home) = env::var_os("HOME") {
            PathBuf::from(home).join(&path[2..])
        } else {
            PathBuf::from(path)
        }
    } else if path == "~" {
        if let Some(home) = env::var_os("HOME") { PathBuf::from(home) } else { PathBuf::from(path) }
    } else {
        PathBuf::from(path)
    };

    path.canonicalize().unwrap_or(path)
}
