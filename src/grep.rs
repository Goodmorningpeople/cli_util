use std::fs;

use clap::ArgMatches;

pub fn match_grep(grep_args: Option<&ArgMatches>) {
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
}
