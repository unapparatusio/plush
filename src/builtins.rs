use crate::*;
use std::env;
use std::process;
use std::cell::RefCell;

thread_local! {
pub static OVERWRITES: RefCell<Vec<(String, CommandHandler)>> = 
    RefCell::new(vec![
        (String::from("exit"), CommandHandler::Builtin(exit)), 
        (String::from("cd"), CommandHandler::Builtin(cd)),
        (String::from("alias"), CommandHandler::Builtin(alias)),
    ]);
}

pub(crate) fn find_and_handle(root: &str, args: &Args) -> bool {

    let handler = OVERWRITES.with_borrow(|v| {
        v.iter()
            .find(|(cmd_name, _)| cmd_name == root)
            .map(|(_, handler)| handler.clone())
    });

    if let Some(handler) = handler {
        handler.handle(args);
        return true
    }

    false
}

fn exit(args: &Args) {
    if args.len() == 0 {
        process::exit(0);
    }

    eprintln!("The command `exit` takes no arguments");
}

fn cd(args: &Args) {
    if args.len() == 1 {
        let _ = env::set_current_dir(args.as_slice()[0]).map_err(|e| eprintln!("cd: {e}"));
    } else {
        eprintln!("cd: one argument needed, found: {}", args.len());
    }
}

fn alias(args: &Args) {
    let mut args = OwnedArgs::from(args);
    let key = String::from(args.remove(0).unwrap());

    assert_eq!("as", args.remove(0).unwrap().as_str());

    OVERWRITES.with_borrow_mut(|v| {
        v.push((key, CommandHandler::Alias(args)));
    });
}

#[derive(Clone, Debug)]
enum CommandHandler {
    Builtin (fn(&Args)),
    Alias (OwnedArgs),
}

impl CommandHandler {
    fn handle(&self, args: &Args)
    {
        match self {
            CommandHandler::Builtin (subr) => subr(args),
            CommandHandler::Alias (replacement) => {
                todo!()
            }
        }
    }
}
