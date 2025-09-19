pub struct Command;

impl Command {
    pub fn rm(args: &[&str]) {
        rm::rm(args);
    }

    pub fn cp(args: &[&str]) {
        cp::cp(args);
    }
}

mod rm;
mod cp;
