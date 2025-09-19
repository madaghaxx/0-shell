use std::fs;
use std::path::Path;
pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        println!("cp: missing file operand");
        return;
    }

    let source = args[0];
    let dest = args[1];

    if !Path::new(source).exists() {
        println!("cp: cannot stat '{}': No such file or directory", source);
        return;
    }

    let dest_path = if Path::new(dest).is_dir() {
        let filename = Path::new(source).file_name().unwrap();
        Path::new(dest).join(filename)
    } else {
        Path::new(dest).to_path_buf()
    };

    match fs::copy(source, &dest_path) {
        Ok(_) => {
            return;
        }
        Err(e) => {
            println!("cp: cannot copy '{}' to '{}': {}", source, dest_path.display(), e);
        }
    }
}
