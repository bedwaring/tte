use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    let path = Path::new(filename);
    let mut file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => String::new(),
    };
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        match input {
            ":q" => {
                println!("Quitting...");
                break;
            }
            ":w" => {
                save_file(&filename, &file_content);
                println!("File saved successfully.");
            }
            ":wq" => {
                save_file(&filename, &file_content);
                println!("Saving and quitting...");
            }
            _ => {
                file_content.push_str(input);
                file_content.push('\n');
            }
        }
    }
}

fn save_file(filename: &str, content: &str) {
    fs::write(filename, content).expect("Failed to write file");
}
