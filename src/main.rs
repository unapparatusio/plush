use std::io::{self, Write};
use std::path::{PathBuf, Path};
use std::env;
use plush;

fn main() {
    loop {
        let mut prompt = env::current_dir().expect("Invalid CWD (Current Working Directory)");
        if let Ok(relative_path) = prompt
            .strip_prefix(
                env::var_os("HOME")
                 .expect("Expected an environment variable: $HOME; $HOME should point to user's home directory")
            )
        {
                prompt = PathBuf::from("~").join(relative_path);
        }


        let mut prompt = prompt.to_string_lossy().to_string();
        prompt.push_str("$:- ");

        let mut line = String::new();

        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            println!();
            continue;
        }
        
        let args = line.split(" ").collect();

        plush::execute(&args);
        
    }
}
