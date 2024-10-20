use std::fs;

use clap::ArgMatches;

pub fn match_cat(cat_args: Option<&ArgMatches>) {
    match cat_args {
        Some(args) => {
            let file_path = args.get_one::<String>("file-path-input").unwrap();
            println!(
                "{}",
                fs::read_to_string(file_path).expect("File path is invalid!")
            );
        }
        None => {}
    }
}
