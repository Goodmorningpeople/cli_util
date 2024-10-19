use clap::ArgMatches;

pub fn match_echo(echo_args: Option<&ArgMatches>) {
    match echo_args {
        Some(args) => {
            if let Some(input) = args.get_one::<String>("string-input") {
                println!("{}", input);
            }
        }
        None => {}
    }
}