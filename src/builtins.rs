use crate::*;
use std::env;
use std::process;

pub const OVERWRITES: [(&str, CommandHandler); 2] = [
    ("exit", CommandHandler::Builtin(exit)), 
    ("cd", CommandHandler::Builtin(cd))
];

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

pub(crate) enum CommandHandler {
    Builtin (fn(Args)),
    Alias (OwnedArgs),
}

impl CommandHandler {
    pub(crate) fn handle(&self, args: Args)
    {
        match self {
            CommandHandler::Builtin (subr) => subr(args),
            CommandHandler::Alias (replacement) => {
                todo!()
            }
        }
    }
}
