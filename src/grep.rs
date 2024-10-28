use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use clap::ArgMatches;
pub fn match_grep(grep_args: Option<&ArgMatches>) {
    if let Some(args) = grep_args {
        // initialize required variables
        let file_name = args.get_one::<String>("file-name-input").unwrap();
        let mut pattern_input = args.get_one::<String>("pattern-input").unwrap().to_string();

        // initialize option variables
        let line_number_option = args.get_flag("number-line-option");
        let count_option = args.get_flag("count-option");
        let ignore_case_option = args.get_flag("ignore-case-option");
        let invert_match_option = args.get_flag("invert-match-option");

        // error handling for read lines function
        match read_lines(file_name) {
            Ok(mut lines) => {
                let mut line_number = 1;
                let mut counter = 0;
                while let Some(entry) = lines.next() {
                    match entry {
                        Ok(line) => {
                            let mut tline = line.clone();
                            // if ignore case option is used
                            if ignore_case_option {
                                tline = tline.to_lowercase();
                                pattern_input = pattern_input.to_lowercase();
                            }
                            // if invert match option is used
                            if invert_match_option && tline.contains(&pattern_input) {
                                counter += 1;
                                continue;
                            // if invert match option is used
                            } else if !invert_match_option && !tline.contains(&pattern_input) {
                                line_number += 1;
                                continue;
                            }
                            // if line number option is used
                            if line_number_option {
                                line_number += 1;
                                print!("{}    ", line_number);
                            }
                            // final output
                            println!("{}", line);
                             // make sure that counter only gets added to when the line matches the pattern 
                            if !invert_match_option {
                                counter += 1;
                            }
                        }
                        Err(e) => eprintln!("Failed to read line: {:?}", e),
                    }
                }
                if count_option {
                    println!("");
                    println!("{} instance(s)", counter);
                }
            }
            Err(e) => eprintln!("Failed to read lines of {}: {:?}", pattern_input, e),
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
