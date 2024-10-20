use std::{env, path::Path};

use clap::ArgMatches;

pub fn match_cd(cd_args: Option<&ArgMatches>) {
    match cd_args {
        Some(args) => {
            let dir_path = args.get_one::<String>("directory-path-input").unwrap();
            let path = Path::new(&dir_path);
            env::set_current_dir(&path).expect("Invalid directory path!");
            println!("Successfully changed working directory to {}", dir_path);
        }
        None => {}
    }
}