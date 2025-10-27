use crate::*;
use std::env;
use std::process;

pub const BUILTINS: [(&str, fn(Args)); 2] = [("exit", exit), ("cd", cd)];

fn exit(args: Args) {
    if args.len() == 0 {
        process::exit(0);
    }

    eprintln!("The command `exit` takes no arguments");
}

fn cd(args: Args) {
    if args.len() == 1 {
        let _ = env::set_current_dir(args.as_slice()[0]).map_err(|e| eprintln!("cd: {e}"));
    } else {
        eprintln!("cd: one argument needed, found: {}", args.len());
    }
}
