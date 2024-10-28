use std::env;

use clap::ArgMatches;

pub fn match_pwd(pwd_args: Option<&ArgMatches>) {
    if let Some(args) = pwd_args {
        // initialize option variables

        match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display())
            }
            Err(e) => eprintln!("Error getting current working directory: {:?}", e),
        }
    }
}
