use clap::ArgMatches;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
pub fn match_cat(cat_args: Option<&ArgMatches>) {
    if let Some(args) = cat_args {
        // initialize required variables
        let file_path = args.get_one::<String>("file-path-input").unwrap();

        // initialize option variables
        let line_number_option = args.get_flag("line-number-option");
        let non_empty_line_number_option = args.get_flag("non-empty-line-number-option");
        let eol_special_option = args.get_flag("eol-special-option");
        let squeeze_line_option = args.get_flag("squeeze-line-option");
        let tab_character_option = args.get_flag("tab-character-option");

        match read_lines(file_path) {
            Ok(mut lines) => {
                let mut counter = 0;
                while let Some(entry) = lines.next() {
                    match entry {
                        Ok(mut line) => {
                            if squeeze_line_option && !line.is_empty() {
                                continue;
                            }
                            if line_number_option {
                                print!("{}", counter);
                            } else if non_empty_line_number_option && !line.is_empty() {
                                print!("{}", counter);
                            }
                            if tab_character_option {
                                line = line.replace("\t", "^T");
                            }
                            print!("{}", line);
                            if eol_special_option {
                                println!("$");
                            } else {
                                println!("");
                            }
                            counter += 1;
                        }
                        Err(e) => eprintln!("Error reading line entry: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read file content of {}: {:?}", file_path, e),
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
