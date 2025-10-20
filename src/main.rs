use std::io::{self, Write};
use std::process;

mod builtins {
    use std::process;
    use std::env;
    
    pub const BUILTINS: [(&str, fn(&[&str])); 2] = [
        ("exit", exit),
        ("cd", cd),
    ];
    
    fn exit(args: &[&str]) {
        if args.len() == 0 {
            process::exit(0);
        }

        eprintln!("The command `exit` takes no arguments");
    }

    fn cd(args: &[&str]) {
        if args.len() == 1 {
            env::set_current_dir(args[0])
                .map_err(|e| eprintln!("cd: {e}"));
        } else {
            eprintln!("cd: one argument needed, found: {}", args.len());
        }
    }
}

fn execute(args: &Vec<&str>) {
    use crate::builtins::*;

    let (root, args) = args.split_at(1);
    let root = &root[0];

    for (cmd, subr) in BUILTINS.iter() {
        if root == cmd {
            subr(args);
            return;
        }
    }

    launch(root, args);

}

fn launch(program: &str, args: &[&str]) {
    let _ = process::Command::new(program)
        .args(args)
        .status()
        .map_err(|e| eprintln!("plush: error while executing program: {e}"));
}

fn main() {
    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            println!();
            continue;
        }
        
        let args = line.split(" ").collect();

        execute(&args);
        
    }
}
