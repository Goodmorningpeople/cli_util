use std::fs;

use clap::ArgMatches;

pub fn match_ls(ls_args: Option<&ArgMatches>) {
    match ls_args {
        Some(args) => {
            let input = args
                .get_one::<String>("directory-path-input")
                .map_or("./".to_string(), |s| s.clone());

            let paths = fs::read_dir(&input).expect("Directory path is invalid!");

            for entry in paths {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if let Some(name) = path.file_name() {
                    if name.to_str().map_or(false, |n| n.starts_with('.')) {
                        continue;
                    }

                    print!(
                        "{}    ",
                        path.display()
                            .to_string()
                            .strip_prefix("./")
                            .unwrap_or(&path.display().to_string())
                    );
                }
            }
        }
        None => {}
    }
}
