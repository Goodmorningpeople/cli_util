use clap::ArgMatches;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn match_cat(cat_args: Option<&ArgMatches>) {
    match cat_args {
        Some(args) => {
            let file_path = args
                .get_one::<String>("file-path-input")
                .expect("Invalid file path!");
            if let Ok(lines) = read_lines(file_path) {
                let mut counter = 0;
                for line in lines {
                    if let Ok(content) = line {
                        if args.get_flag("line-number-option") {
                            print!("{}      ", counter);
                            counter += 1;
                        } else if args.get_flag("non-empty-line-number-option") {
                            if !content.is_empty() {
                                print!("{}      ", counter);
                            }
                        }
                        if args.get_flag("squeeze-line-option") {
                            if !content.is_empty() {
                                if args.get_flag("tab-character-option") {
                                    print!("{}", content.replace("\t", "^T"));
                                } else {
                                    print!("{}", content);
                                }
                                if args.get_flag("eol-special-option") {
                                    println!("$");
                                } else {
                                    println!("");
                                }
                            }
                        } else {
                            if args.get_flag("tab-character-option") {
                                print!("{}", content.replace("\t", "^T"));
                            } else {
                                print!("{}", content);
                            }
                            if args.get_flag("eol-special-option") {
                                println!("$");
                            } else {
                                println!("");
                            }
                        }
                    }
                }
            }
        }
        None => {}
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
