use std::fs;

use clap::{command, Arg, Command};

fn main() {
    let match_result = command!()
        .about("Basic ClI utilities\n Basic CLI utilities written in Rust to be more efficient, faster and easily modifiable.")
        .subcommand(
            Command::new("echo").about("echo <String>, takes a argument of type <String> and prints the argument to the screen, place double-quotes around the argument to have spaces")
                .arg(
                    Arg::new("string-input")
                        .required(true)
                )
                       )
        .subcommand(
            Command::new("cat").about("cat <path-to-file>, takes a path to a file and prints the content of the fileto the screen, place double-quotes around the argument to have spaces")
                .arg(
                    Arg::new("file-path-input")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("ls").about("ls <path-to-directory>, takes an optional path to a directory and prints the content of that directory or the current working directory if not specified")
        .arg(
            Arg::new("directory-path-input")
        )
        )
        .get_matches();

    let echo_args = match_result.subcommand_matches("echo");
    match echo_args {
        Some(args) => {
            if let Some(input) = args.get_one::<String>("string-input") {
                println!("{}", input);
            }
        }
        None => {}
    }

    let cat_args = match_result.subcommand_matches("cat");
    match cat_args {
        Some(args) => {
            if let Some(input) = args.get_one::<String>("file-path-input") {
                println!(
                    "{}",
                    fs::read_to_string(input).expect("File path is invalid!")
                );
            }
        }
        None => {}
    }

    let ls_args = match_result.subcommand_matches("ls");
    match ls_args {
        Some(args) => {
            if let Some(input) = args.get_one::<String>("directory-path-input") {
                let paths = fs::read_dir(input).expect("Directory path is invalid!");
                println!("");
                paths.for_each(|path| print!("{}", path.unwrap().path().display()));
            } else {
                let paths = fs::read_dir("./").unwrap();
                println!("");
                paths.for_each(|path| print!("{}  ", path.unwrap().path().display()));
            }
        }
        None => {},
    }
}
