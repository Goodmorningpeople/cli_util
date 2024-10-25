use clap::ArgMatches;

pub fn match_echo(echo_args: Option<&ArgMatches>) {
    match echo_args {
        Some(args) => {
            if let Some(s) = args.get_one::<String>("string-input") {
                if echo_args.unwrap().get_flag("enable-special-option") {
                    if echo_args.unwrap().get_flag("newline-option") {
                        print!("{}", s);
                    }
                    println!("{}", s);
                } else if echo_args.unwrap().get_flag("newline-option") {
                    print!("r{}", s);
                } else {
                    println!(r"{}", s);
                }
            }
        }
        None => {}
    }
}
