mod builtins;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

pub struct OwnedArgs {
    inner: Vec<String>,
}

impl OwnedArgs {
    pub fn new() -> Self {
        OwnedArgs { inner: Vec::new() }
    }

    pub fn push(&mut self, element: String) {
        let cleaned = element.trim().to_string();

        if !cleaned.is_empty() {
            self.inner.push(cleaned);
        }
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.inner.remove(index)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[String] {
        self.inner.as_slice()
    }
}

impl<T> From<&'_ T> for OwnedArgs
where
    T: AsRef<str>,
{
    fn from(s: &'_ T) -> Self {
        OwnedArgs::from(&Args::from(s))
    }
}

impl From<&Args<'_>> for OwnedArgs {
    fn from(args: &Args) -> Self {
        let inner = args.inner.iter().map(|x| x.to_string()).collect();

        OwnedArgs { inner }
    }
}

pub struct Args<'a> {
    inner: Vec<&'a str>,
}

impl<'a> Args<'a> {
    pub fn new() -> Self {
        Args { inner: Vec::new() }
    }

    pub fn push(&mut self, element: &'a str) {
        let cleaned = element.trim();

        if !cleaned.is_empty() {
            self.inner.push(cleaned);
        }
    }

    pub fn remove(&mut self, index: usize) -> &'a str {
        self.inner.remove(index)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[&str] {
        self.inner.as_slice()
    }
}

impl<'a, T> From<&'a T> for Args<'a>
where
    T: AsRef<str>,
{
    fn from(s: &'a T) -> Self {
        let inner = s.as_ref().split_ascii_whitespace().collect();

        Args { inner }
    }
}

impl<'a> From<&'a OwnedArgs> for Args<'a> {
    fn from(args: &'a OwnedArgs) -> Self {
        let inner = args.inner.iter().map(|x| x.as_ref()).collect();

        Args { inner }
    }
}

pub fn get_cwd() -> PathBuf {
    if let Ok(cwd) = env::current_dir() {
        return cwd;
    } else {
        eprintln!("plush: Invalid Working Directory");
        process::exit(1);
    }
}

pub fn get_home() -> PathBuf {
    if let Some(home) = env::var_os("HOME") {
        return home.into();
    } else {
        eprintln!("plush: Expected an environment variable: $HOME");
        eprintln!("$HOME -> [current user's home directory]");
        process::exit(1);
    }
}

pub fn into_shell_path(path: &Path) -> String {
    let home = get_home();

    if let Ok(relative_path) = path.strip_prefix(&home) {
        return Path::new("~")
            .join(relative_path)
            .to_string_lossy()
            .to_string();
    } else {
        return path.to_string_lossy().to_string();
    }
}

pub fn execute(mut args: Args) {
    use crate::builtins::*;

    if args.len() == 0 {
        return;
    }

    let root = args.remove(0);

    for &(cmd, ref handler) in OVERWRITES.iter() {
        if root == cmd {
            handler.handle(args);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_into_shell_path() {
        let home_dir = env::var_os("HOME").unwrap();
        let home_dir = PathBuf::from(home_dir);

        let path1 = home_dir.join("Documents");
        let path2 = home_dir.join("Games");
        let path3 = PathBuf::from("/usr/bin");

        let path1 = into_shell_path(&path1);
        let path2 = into_shell_path(&path2);
        let path3 = into_shell_path(&path3);

        assert_eq!(path1, String::from("~/Documents"));
        assert_eq!(path2, String::from("~/Games"));
        assert_eq!(path3, String::from("/usr/bin"));
    }
}
