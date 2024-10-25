use std::env;

use clap::ArgMatches;

pub fn match_pwd(pwd_args: Option<&ArgMatches>) {
    match pwd_args {
        Some(_) => {
            let path = env::current_dir().unwrap();
            println!("{}", path.display());
        }
        None => {}
    }
}
