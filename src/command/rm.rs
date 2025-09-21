use std::fs;
use std::path::Path;
use crate::command::util::{ expand_path, require_min_args };

pub fn rm(args: &[&str]) {
    if !require_min_args("rm", args, 1, "missing operand") {
        return;
    }
    let (recursive, file_args) = parse_flags(args);
    for raw in file_args {
        let path = expand_path(raw);
        let result = if path.is_dir() {
            if recursive {
                remove_dir_all(&path)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "is a directory"))
            }
        } else {
            fs::remove_file(&path)
        };

        if let Err(e) = result {
            eprintln!("rm: cannot remove '{}': {}", raw, e);
        }
    }
}
fn parse_flags<'a>(args: &'a [&'a str]) -> (bool, &'a [&'a str]) {
    let mut recursive = false;
    let mut idx = 0;
    while idx < args.len() && args[idx].starts_with('-') {
        for ch in args[idx][1..].chars() {
            match ch {
                'r' | 'R' => {
                    recursive = true;
                }
                _ => {}
            }
        }
        idx += 1;
    }
    (recursive, &args[idx..])
}

fn remove_dir_all(path: &Path) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}
