pub mod rm;
pub mod cp;
pub mod mv;
pub mod pwd;
mod util;

type CmdFn = fn(&[&str]);

const COMMANDS: &[(&str, CmdFn)] = &[
    ("rm", rm::rm),
    ("cp", cp::cp),
    ("mv", mv::mv),
    ("pwd", |_| pwd::pwd()),
];

pub fn dispatch(name: &str, args: &[&str]) -> bool {
    for (n, f) in COMMANDS {
        if *n == name {
            f(args);
            return true;
        }
    }
    false
}
