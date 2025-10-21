use std::io::{self, Write};
use plush::{self, get_cwd};

fn main() {
    loop {
        let mut prompt = plush::into_shell_path(&get_cwd());

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
