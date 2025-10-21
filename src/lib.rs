use std::process;

mod builtins {
    use std::env;
    use std::process;
    
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
            let _ = env::set_current_dir(args[0])
                .map_err(|e| eprintln!("cd: {e}"));
        } else {
            eprintln!("cd: one argument needed, found: {}", args.len());
        }
    }
}

pub fn execute(args: &Vec<&str>) {
    use crate::builtins::*;

    let (root, args) = args.split_at(1);
    let root: &str = root[0];

    for &(cmd, subr) in BUILTINS.iter() {
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
