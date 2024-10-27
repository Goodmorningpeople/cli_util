use std::{fs, time::SystemTime};
use clap::ArgMatches;
use file_owner::PathExt;

pub fn match_find(find_args: Option<&ArgMatches>) {
    if let Some(args) = find_args {
        let dir_path = args.get_one::<String>("directory-path-input").unwrap();
        let mut paths = fs::read_dir(&dir_path).expect("Invalid directory path!");
        let mut counter = 0;

        let name_option = args.get_one::<String>("name-option");
        let type_option = args.get_one::<String>("type-option");
        let owner_option = args.get_one::<String>("owner-option");
        let group_option = args.get_one::<String>("group-option");
        let mtime_option = args.get_one::<String>("mtime-option");

        let mtime_days: Option<i32> = mtime_option.and_then(|s| s.parse().ok());

        while let Some(entry) = paths.next() {
            match entry {
                Ok(path) => {
                    let tpath = path.path();
                    let metadata = match tpath.metadata() {
                        Ok(meta) => meta,
                        Err(e) => {
                            eprintln!("Error getting metadata for {:?}: {:?}", tpath, e);
                            continue;
                        }
                    };

                    // Check type
                    if let Some(type_option) = type_option {
                        if (type_option == "d" && !metadata.is_dir()) ||
                           (type_option == "f" && metadata.is_dir()) {
                            continue;
                        }
                    }

                    // Check owner
                    if let Some(owner_option) = owner_option {
                        if tpath.owner().unwrap().to_string() != *owner_option {
                            continue;
                        }
                    }

                    // Check group
                    if let Some(group_option) = group_option {
                        if tpath.group().unwrap().to_string() != *group_option {
                            continue;
                        }
                    }

                    // Check modification time
                    if let Some(mtime_days) = mtime_days {
                        let mod_time = metadata.modified().unwrap();
                        let current_time = SystemTime::now();
                        let days_passed = match current_time.duration_since(mod_time) {
                            Ok(duration) => duration.as_secs() / (60 * 60 * 24),
                            Err(e) => {
                                eprintln!("Error getting duration: {:?}", e);
                                continue;
                            }
                        };

                        if (mtime_days < 0 && mtime_days > days_passed as i32) ||
                           (mtime_days > 0 && mtime_days < days_passed as i32) {
                            continue;
                        }
                    }

                    // Check name
                    if let Some(name_option) = name_option {
                        if let Some(s) = tpath.file_name() {
                            let name = s.to_string_lossy();
                            if name == *name_option {
                                println!("{}", name);
                                counter += 1;
                            }
                        }
                    } else if let Some(s) = tpath.file_name() {
                        let name = s.to_string_lossy();
                        println!("{}", name);
                        counter += 1;
                    }
                }
                Err(e) => eprintln!("Error reading directory entry: {:?}", e),
            }
        }

        println!("\n{} instance(s)", counter);
    }
}