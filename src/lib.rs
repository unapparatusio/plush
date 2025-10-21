use std::process;
use std::path::{PathBuf, Path};
use std::env;

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

pub fn get_cwd() -> PathBuf
{
    if let Ok(cwd) = env::current_dir() {
        return cwd;
   } else {
        eprintln!("plush: Invalid Working Directory");
        process::exit(1);
    }
}

pub fn get_home() -> PathBuf
{
    if let Some(home) = env::var_os("HOME") {
        return home.into();
    } else {
        eprintln!("plush: Expected an environment variable: $HOME");
        eprintln!("$HOME -> [current user's home directory]");
        process::exit(1);
    }
}

pub fn into_shell_path(path: &Path) -> String
{
    let home = get_home();

    if let Ok(relative_path) = path.strip_prefix(&home) {
        return Path::new("~")
            .join(relative_path)
            .to_string_lossy()
            .to_string();
    } else {
        return path
            .to_string_lossy()
            .to_string();
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
