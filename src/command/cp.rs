use std::fs;
use std::path::Path;
pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        println!("cp: missing file operand");
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
    let source_path = Path::new(source);
    if source_path.is_dir() && !recursive {
        println!("cp: -r not specified; omitting directory '{}'", source);
        return;
    }
    let dest_path = if Path::new(dest).is_dir() {
        Path::new(dest).join(source_path.file_name().unwrap())
    } else {
        Path::new(dest).to_path_buf()
    };

    let result = if recursive && source_path.is_dir() {
        copy_dir_recursive(source_path, &dest_path)
    } else {
        fs::copy(source, &dest_path).map(|_| ())
    };

    if let Err(e) = result {
        eprintln!("cp: cannot copy '{}' to '{}': {}", source, dest_path.display(), e);
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
