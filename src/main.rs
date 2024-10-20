use clap::{command, Arg, Command};
use cli_util::{cat::match_cat, cd::match_cd, echo::match_echo, find::match_find, grep::match_grep, ls::match_ls, pwd::match_pwd};

fn main() {
    let match_result = command!()
        .about("Basic CLI utilities written in Rust to be more efficient, faster and easily modifiable.")
        .subcommand(
            Command::new("echo").about("echo [options] [input]>, takes a argument of type <String> and prints the argument to the screen, place double-quotes around the argument to have spaces
")
                .arg(
                    Arg::new("string-input")
                        .required(true)
                )
                       )
        .subcommand(
            Command::new("cat").about("cat [options] [path-to-file], takes a path to a file and prints the content of the file to the screen, place double-quotes around the argument to have spaces
")
                .arg(
                    Arg::new("file-path-input")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("ls").about("ls [options] [path-to-directory], takes an optional path to a directory and prints the content of that directory or the current working directory if not specified
")
        .arg(
            Arg::new("directory-path-input")
        )
        )
        .subcommand(
            Command::new("find").about("find [path-to-directory] [options] [expressions], takes a path to a directory and finds a file(s) in it based on the options:
-name [file-name]: finds a file based on it's name
")
            .arg(
                Arg::new("directory-path-input")
                    .required(true)
            )
            .arg(
                Arg::new("name-option")
                    .short('n')
                    .long("name")
                    .alias("Name")
            )
        )
        .subcommand(
            Command::new("grep").about("grep [options] [pattern] [expression-name], looks for a pattern in a file and prints if the pattern is in the file or the files in a directory and prints the file(s)
")
                .arg(
                    Arg::new("pattern-input")
                        .required(true)
                )
                .arg(
                    Arg::new("expression-name-input")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("pwd").about("pwd [options], prints the current working directory
")
        )
        .subcommand(
            Command::new("cd").about("cd [options] [path-to-directory], changes the current working directory to directory specified
                ")
            .arg(
                Arg::new("directory-path-input")
            )
        )
                .get_matches();

    let echo_args = match_result.subcommand_matches("echo");
    match_echo(echo_args);

    let cat_args = match_result.subcommand_matches("cat");
    match_cat(cat_args);

    let ls_args = match_result.subcommand_matches("ls");
    match_ls(ls_args);

    let grep_args = match_result.subcommand_matches("grep");
    match_grep(grep_args);
    
    let find_args = match_result.subcommand_matches("find");
    match_find(find_args);
    
    let pwd_args = match_result.subcommand_matches("pwd");
    match_pwd(pwd_args);

    let cd_args = match_result.subcommand_matches("cd");
    match_cd(cd_args);
}
