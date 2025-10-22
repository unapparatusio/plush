use std::process;
use std::path::{PathBuf, Path};
use std::env;

pub struct Args<'a> {
    inner: Vec<&'a str>,
}

impl<'a> Args<'a> {
    pub fn new() -> Self {
        Args
        {
            inner: Vec::new(),
        }
    }

    pub fn add(&mut self, element: &'a str) {
        let cleaned = element.trim();

        if !cleaned.is_empty() {
            self.inner.push(cleaned);
        }
    }

    pub fn remove(&mut self, index: usize) -> &'a str
    {
        self.inner.remove(index)
    }

    pub fn len(&self) -> usize
    {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[&str]
    {
        self.inner.as_slice()
    }
}

impl<'a, T> From<&'a T> for Args<'a>
    where T: AsRef<str> {

    fn from(s: &'a T) -> Self {
        let inner = s.as_ref()
            .split_ascii_whitespace()
            .collect();

        Args
        {
            inner
        }
    }
}

mod builtins {
    use std::env;
    use std::process;
    use crate::*;
    
    pub const BUILTINS: [(&str, fn(Args)); 2] = [
        ("exit", exit),
        ("cd", cd),
    ];
    
    fn exit(args: Args) {
        if args.len() == 0 {
            process::exit(0);
        }

        eprintln!("The command `exit` takes no arguments");
    }

    fn cd(args: Args) {
        if args.len() == 1 {
            let _ = env::set_current_dir(args.as_slice()[0])
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

pub fn execute(mut args: Args) {
    use crate::builtins::*;

    if args.len() == 0 {
            return;
    }

    let root = args.remove(0);

    for &(cmd, subr) in BUILTINS.iter() {
        if root == cmd {
            subr(args);
            return;
        }
    }

    launch(root, args);

}

fn launch(program: &str, args: Args) {
    let _ = process::Command::new(program)
        .args(args.as_slice())
        .status()
        .map_err(|e| eprintln!("plush: error while executing program: {e}"));
}
