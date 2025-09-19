use crate::command::{ cp, rm };

pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        println!("mv: missing file operand");
        return;
    }
    let source = args[0];
    let destination = args[1];
    cp::cp(&[source, destination]);
    rm::rm(&[source]);
}
