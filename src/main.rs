use plush::{self, get_cwd};
use std::io::{self, Write};

fn main() {
    loop {
        let mut prompt = plush::into_shell_path(&get_cwd());

        prompt.push_str("$:- ");

        let mut line = String::new();

        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        if !line.ends_with('\n') {
            println!();
            continue;
        }

        let args = plush::Args::from(&line);

        plush::execute(args);
    }
}
