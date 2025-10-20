use std::io::{self, Write};
use std::process;

fn execute(args: &Vec<&str>)
{
    let (program, args) = args.split_at(1);
    let program = &program[0];

    let _ = process::Command::new(program)
        .args(args)
        .status()
        .map_err(|e| eprintln!("plush: error while executing program: {e}"));
}

fn main() {
    loop
    {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty()
        {
            println!();
            continue;
        }
        
        let args = line.split(" ").collect();

        execute(&args);
        
    }
}
