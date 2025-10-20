use std::io;
use std::process;

fn execute(args: &Vec<&str>)
{
    let (program, args) = args.split_at(1);
    let program = &program[0];

    process::Command::new(program)
        .args(args)
        .status();
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let args = line.split(" ").collect();

    execute(&args);
}
