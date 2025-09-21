use std::fs;
use std::path::Path;
use crate::command::util::{ expand_path, require_min_args };

pub fn cp(args: &[&str]) {
    if !require_min_args("cp", args, 2, "missing file operand") {
        return;
    }
    let (recursive, file_args) = if args[0] == "-r" {
        if args.len() < 3 {
            println!("cp: missing file operand");
            return;
        }
        (true, &args[1..])
    } else {
        (false, args)
    };

    let (source, dest) = (file_args[0], file_args[1]);
    let source_path = expand_path(source);
    let dest_path = expand_path(dest);
    if (source_path == dest_path || source == dest) && !source_path.is_dir() {
        println!("cp: \'{}\' and \'{}\' are the same file", source, dest);
        return;
    }
    if source_path.is_dir() && !recursive {
        println!("cp: -r not specified; omitting directory '{}'", source);
        return;
    }

    let final_dest_path = if dest_path.is_dir() {
        dest_path.join(source_path.file_name().unwrap())
    } else {
        dest_path
    };
    if recursive && source_path.is_dir() {
        if
            let (Ok(canonical_src), Ok(canonical_dest)) = (
                source_path.canonicalize(),
                final_dest_path.canonicalize(),
            )
        {
            if canonical_dest.starts_with(&canonical_src) {
                println!("cp: cannot copy a directory, '{}', into itself, '{}'", source, dest);
                return;
            }
        }
    }

    let result = if recursive && source_path.is_dir() {
        copy_dir_recursive(&source_path, &final_dest_path)
    } else {
        fs::copy(&source_path, &final_dest_path).map(|_| ())
    };

    if let Err(e) = result {
        eprintln!("cp: cannot copy '{}' to '{}': {}", source, final_dest_path.display(), e);
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}
