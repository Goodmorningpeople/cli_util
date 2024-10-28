use std::env;

use clap::ArgMatches;

pub fn match_pwd(pwd_args: Option<&ArgMatches>) {
    if let Some(_) = pwd_args {
        // initialize option variables

        // error handling for get working directory
        match env::current_dir() {
            Ok(path) => {
                // final output
                println!("{}", path.display())
            }
            Err(e) => eprintln!("Error getting current working directory: {:?}", e),
        }
    }
}
