use std::fs;

use clap::ArgMatches;

pub fn match_find(find_args: Option<&ArgMatches>) {
    match find_args {
        Some(args) => {
            let mut counter = 0;
            let dir_path = args.get_one::<String>("directory-path-input").unwrap();
            let paths = fs::read_dir(&dir_path).expect("Invalid directory path!");
            if let Some(name_option) = args.get_one::<String>("name-option") {
                println!("");
                for path in paths {
                    if let Some(s) = path.unwrap().path().file_name() {
                        let name = String::from(s.to_str().unwrap());
                        if &name == name_option {
                            println!("{}", name);
                            counter += 1;
                        }
                    }
                }
                println!("\n{} instance(s)", counter);
            }
        }
        None => {}
    }
}
