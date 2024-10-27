use clap::ArgMatches;

pub fn match_echo(echo_args: Option<&ArgMatches>) {
    if let Some(args) = echo_args {
        // initialize required variables
        let string_input = args.get_one::<String>("string-input").unwrap();

        // initialize option variables
        let newline_option = args.get_flag("newline-option");
        let enable_special_option = args.get_flag("enable-special-option");

        // if both options used
        if newline_option && enable_special_option {
            print!("{}", string_input);
        // if newline-option used
        } else if newline_option {
            print!(r"{}", string_input);
        // if enable-special-option used
        } else if enable_special_option {
            println!("{}", string_input);
        // if no options used
        } else {
            println!(r"{}", string_input)
        }
    }
}
