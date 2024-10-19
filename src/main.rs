use std::{env, fs::{self}};

use clap::{command, Arg, Command};

fn main() {
    let match_result = command!()
        .about("Basic ClI utilities\n Basic CLI utilities written in Rust to be more efficient, faster and easily modifiable.")
        .subcommand(
            Command::new("echo").about("echo [options] [input]>, takes a argument of type <String> and prints the argument to the screen, place double-quotes around the argument to have spaces")
                .arg(
                    Arg::new("string-input")
                        .required(true)
                )
                       )
        .subcommand(
            Command::new("cat").about("cat [options] [path-to-file], takes a path to a file and prints the content of the file to the screen, place double-quotes around the argument to have spaces")
                .arg(
                    Arg::new("file-path-input")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("ls").about("ls [options] [path-to-directory], takes an optional path to a directory and prints the content of that directory or the current working directory if not specified")
        .arg(
            Arg::new("directory-path-input")
        )
        )
        .subcommand(
            Command::new("find").about("find [path-to-directory] [options] [expressions], takes a path to a directory and finds a file(s) in it based on the options:
-name [file-name]: finds a file based on it's name")
            .arg(
                Arg::new("directory-path-input")
                    .required(true)
            )
            .arg(
                Arg::new("name-option-input")
                    .short('n')
                    .long("name")
                    .alias("Name")
            )
        )
        .subcommand(
            Command::new("grep").about("grep [options] [pattern] [expression-name], looks for a pattern in a file and prints if the pattern is in the file or the files in a directory and prints the file(s)")
                .arg(
                    Arg::new("pattern-input")
                        .required(true)
                )
                .arg(
                    Arg::new("expression-name-input")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("pwd").about("pwd [options], prints the current working directory")
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
            let input = args.get_one::<String>("file-path-input").unwrap();
            println!(
                "{}",
                fs::read_to_string(input).expect("File path is invalid!")
            );
        }
        None => {}
    }

    let ls_args = match_result.subcommand_matches("ls");
    match ls_args {
        Some(args) => {
            let input = args
                .get_one::<String>("directory-path-input")
                .map_or("./".to_string(), |s| s.clone());

            let paths = fs::read_dir(&input).expect("Directory path is invalid!");

            for entry in paths {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if let Some(name) = path.file_name() {
                    if name.to_str().map_or(false, |n| n.starts_with('.')) {
                        continue;
                    }

                    print!(
                        "{}    ",
                        path.display()
                            .to_string()
                            .strip_prefix("./")
                            .unwrap_or(&path.display().to_string())
                    );
                }
            }
        }
        None => {}
    }
    let find_args = match_result.subcommand_matches("find");
    match find_args {
        Some(args) => {
            let mut counter = 0;
            let dir_path = args.get_one::<String>("directory-path-input").unwrap();
            let paths = fs::read_dir(&dir_path).expect("Invalid directory path!");
            if let Some(name_option) = args.get_one::<String>("name-option-input") {
                println!("");
                for path in paths {
                    if let Some(s) = path.unwrap().path().file_name() {
                        let name = String::from(s.to_str().unwrap());
                        if &name == name_option {
                            print!("{}", name);
                            counter += 1;
                        }
                    }
                }
                println!("\n{} instance(s)", counter);
            }
        }
        None => {}
    }

    let grep_args = match_result.subcommand_matches("grep");

    match grep_args {
        Some(args) => {
            let pattern = args.get_one::<String>("pattern-input").unwrap();
            let express_name = args.get_one::<String>("expression-name-input").unwrap();
            match fs::read_to_string(express_name) {
                Ok(s) => {
                    if s.contains(pattern) {
                        println!("File contains pattern");
                    } else {
                        println!("File does not contain pattern");
                    }
                }
                Err(_) => {
                    let mut counter = 0;
                    let paths = fs::read_dir(express_name).expect("Invalid expression!");
                    println!("");
                    for path in paths {
                        if let Some(s) = path.unwrap().path().file_name() {
                            let name = String::from(s.to_str().unwrap());
                            match fs::read_to_string(format!("{}/{}", express_name, name)) {
                                Ok(s) => {
                                    if s.contains(pattern) {
                                        print!("{}    ", name);
                                        counter += 1;
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    println!("\n{} file(s) containing pattern", counter)
                }
            }
        }
        None => {}
    }
    
    let pwd_args = match_result.subcommand_matches("pwd");
    match pwd_args {
        Some(args) => {
            let path = env::current_dir().unwrap();
            println!("{}", path.display());
        }
        None => {}
    }
}
