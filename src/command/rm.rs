use std::fs;
pub fn rm(args: &[&str]) {
    if args.is_empty() {
        println!("rm: missing operand");
        return;
    }
    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == &"-r" || arg == &"--recursive" {
            recursive = true;
        } else {
            files.push(*arg);
        }
    }
    for arg in files {
        let path = std::path::Path::new(arg);
        let result = if path.is_dir() {
            if recursive {
                fs::remove_dir_all(arg)
            } else {
                println!("rm: cannot remove '{}': Is a directory", arg);
                continue;
            }
        } else {
            fs::remove_file(arg)
        };

        if let Err(e) = result {
            println!("rm: cannot remove '{}': {}", arg, e);
        }
    }
}
