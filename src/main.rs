use clap::{command, Arg, ArgAction, Command};
use cli_util::{
    cat::match_cat, echo::match_echo, find::match_find, grep::match_grep, ls::match_ls,
    pwd::match_pwd,
};

fn main() {
    let match_result = command!()
        .about("Basic CLI utilities written in Rust to be more efficient, faster and easily modifiable.")
        .subcommand(
            Command::new("echo").about("echo [options] [string]: takes a argument of type <String> and prints the argument to the screen, place double-quotes around the argument to have spaces
-n: Do not output the trailing newline, allows you to print on the same line without moving onto the next
-e: Enable interpretation of backspace escapes and special characters
")
                .arg(
                    Arg::new("string-input")
                )
                .arg(
                    Arg::new("newline-option")
                    .short('n')
                    .long("newline")
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("enable-special-option")
                        .short('e')
                        .long("enable-special")
                        .action(ArgAction::SetTrue)
                )
                       )
        .subcommand(
            Command::new("cat").about("cat [options] [path-to-file]: takes a path to a file and prints the content of the file to the screen
-n: Number all output lines
-b: Number all non-empty output lines
-s: Squeeze multiple adjacent blank lines, making the output more compact
-e: Display a $ at the end of each line
-t: Show tab characters as ^I
")
                .arg(
                    Arg::new("file-path-input")
                        .required(true)
                )
                .arg(
                    Arg::new("line-number-option")
                        .short('n')
                        .action(ArgAction::SetTrue)
                        .conflicts_with("non-empty-line-number-option")
                )
                .arg(
                     Arg::new("non-empty-line-number-option")
                         .short('b')
                         .action(ArgAction::SetTrue)
                    )
                .arg(
                    Arg::new("squeeze-line-option")
                        .short('s')
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("eol-special-option")
                        .short('e')
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("tab-character-option")
                        .short('t')
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("ls").about("ls [options] [path-to-directory]: takes an optional path to a directory and prints the content of that directory or the current working directory if not specified
-l: Returns a detailed output with file type, permissions, link count, the owner, the group, file size and modification timestamp
-a: Output includes all files, even hidden files
-r: Makes file size outputs human-readable
-R: Recursively outputs directories and their contents (including the files in subdirectories)
-F: Appends a character to each file name to indicate its type (e.g., '/' for directories, '*' for executables)
")
        .arg(
            Arg::new("directory-path-input")
        )
        .arg(
            Arg::new("detailed-output-option")
            .short('l')
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("show-hidden-option")
            .short('a')
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("readable-option")
            .short('r')
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("recursive-option")
            .short('R')
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("append-option")
                .short('F')
                .action(ArgAction::SetTrue)
        )
        )
        .subcommand(
            Command::new("find").about("find [path-to-directory] [options] [expressions]: takes a path to a directory and finds a file(s) in it
-n [file-name]: finds a file based on it's name
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
            Command::new("grep").about("grep [options] [pattern] [expression-name]: looks for a pattern in a file and prints if the pattern is in the file or the files in a directory and prints the file(s)
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
            Command::new("pwd").about("pwd [options]: prints the current working directory
")
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
}
