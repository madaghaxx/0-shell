use std::fs;
use crate::command::{ cp, rm };
use crate::command::util::{ expand_path, require_min_args };

pub fn mv(args: &[&str]) {
    if !require_min_args("mv", args, 1, "missing file operand") {
        return;
    }
    if args.len() == 1 {
        println!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }
    let src_path = expand_path(args[0]);
    let dest_path = expand_path(args[1]);

    if !src_path.exists() {
        eprintln!("mv: cannot move '{}': No such file or directory", args[0]);
        return;
    }
    let final_dest = if dest_path.is_dir() {
        match src_path.file_name() {
            Some(name) => dest_path.join(name),
            None => {
                eprintln!("mv: invalid source path '{}': missing file name", args[0]);
                return;
            }
        }
    } else {
        dest_path
    };
    if let (Ok(csrc), Ok(cdst)) = (src_path.canonicalize(), final_dest.canonicalize()) {
        if csrc == cdst {
            println!("mv: '{}' and '{}' are the same file", args[0], args[1]);
            return;
        }
        if csrc.is_dir() && cdst.starts_with(&csrc) {
            println!(
                "mv: cannot move '{}' to a subdirectory of itself, '{}'",
                args[0],
                final_dest.display()
            );
            return;
        }
    }
    match fs::rename(&src_path, &final_dest) {
        Ok(()) => {
            return;
        }
        Err(e) if e.raw_os_error() == Some(libc::EXDEV) => {
            let src_s = src_path.to_string_lossy().into_owned();
            let dst_s = final_dest.to_string_lossy().into_owned();
            let src_is_dir = src_path.is_dir();
            if src_is_dir {
                let cp_args: [&str; 3] = ["-r", &src_s, &dst_s];
                cp::cp(&cp_args);
                if std::path::Path::new(&dst_s).exists() {
                    let rm_args: [&str; 2] = ["-r", &src_s];
                    rm::rm(&rm_args);
                }
            } else {
                let cp_args: [&str; 2] = [&src_s, &dst_s];
                cp::cp(&cp_args);
                if std::path::Path::new(&dst_s).exists() {
                    let rm_args: [&str; 1] = [&src_s];
                    rm::rm(&rm_args);
                }
            }
        }
        Err(e) => {
            eprintln!("mv: cannot move '{}' to '{}': {}", args[0], final_dest.display(), e);
        }
    }
}
