use chrono::{self, DateTime, Local};
use clap::ArgMatches;
use file_owner::PathExt;
use humansize::{format_size, DECIMAL};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

pub fn match_ls(ls_args: Option<&ArgMatches>) {
    if let Some(args) = ls_args {
        // initialize option variables
        let detailed_output_option = args.get_flag("detailed-output-option");
        let show_hidden_option = args.get_flag("show-hidden-option");
        let readable_option = args.get_flag("readable-option");
        let recursive_option = args.get_flag("recursive-option");
        let append_option = args.get_flag("append-option");

        // check if directory path has been specified, default to current dir if not
        let directory_path_input = args
            .get_one::<String>("directory-path-input")
            .map_or("./", |s| s.as_str());

        match fs::read_dir(directory_path_input) {
            Ok(mut paths) => {
                while let Some(entry) = paths.next() {
                    match entry {
                        Ok(dir_entry) => {
                            let path = dir_entry.path();
                            let name = path.file_name().unwrap();
                            // if file is hidden file and show hidden option is not used
                            if !show_hidden_option
                                && name.to_str().map_or(false, |n| n.starts_with('.'))
                            {
                                continue;
                            }
                            // if detailed output option is used
                            if detailed_output_option {
                                // error handing for getting metadata
                                match path.metadata() {
                                    Ok(meta) => {
                                        // initialize metadata variables
                                        let mode = meta.permissions().mode();
                                        let permission_string = format_permissions(mode);
                                        let file_type = if meta.is_dir() {
                                            "d"
                                        } else if meta.is_symlink() {
                                            "l"
                                        } else {
                                            "-"
                                        };
                                        let file_size = meta.len();
                                        // error handling for getting modification date
                                        match meta.modified() {
                                            Ok(mod_system_time) => {
                                                let mod_datetime: DateTime<Local> =
                                                    mod_system_time.into();
                                                let owner = path.owner().unwrap();
                                                let group = path.group().unwrap();
                                                let link_number = meta.nlink();
                                                // final output
                                                // if readable option is used
                                                if readable_option {
                                                    print!(
                                                        "{}{} {} {} {} {} {} ",
                                                        file_type,
                                                        permission_string,
                                                        link_number,
                                                        owner,
                                                        group,
                                                        format_size(file_size, DECIMAL),
                                                        mod_datetime.format("%d/%m/%Y %T")
                                                    );
                                                // if readable option is not used
                                                } else {
                                                    print!(
                                                        "{}{} {} {} {} {} {} ",
                                                        file_type,
                                                        permission_string,
                                                        link_number,
                                                        owner,
                                                        group,
                                                        file_size,
                                                        mod_datetime.format("%d/%m/%Y %T")
                                                    );
                                                }
                                            }
                                            Err(e) => eprintln!(
                                                "Failed to get modification date of {}: {:?}",
                                                path.display().to_string(),
                                                e
                                            ),
                                        }
                                    }
                                    Err(e) => eprintln!(
                                        "Failed to get metadata of {}: {:?}",
                                        path.display().to_string(),
                                        e
                                    ),
                                }
                            }
                            // if  append option is used
                            if append_option {
                                let meta = path.metadata().unwrap();
                                let mode = meta.permissions().mode();
                                if meta.is_dir() {
                                    print!("/");
                                } else if meta.is_symlink() {
                                    print!("@");
                                } else if mode & 0o100 != 0
                                    || mode & 0o010 != 0
                                    || mode & 0o001 != 0
                                {
                                    print!("*");
                                }
                            }

                            // if recursive option is used and path is directory
                            if recursive_option && path.is_dir() {
                                println!("");
                                println!(
                                    "{}:",
                                    path.strip_prefix(directory_path_input)
                                        .unwrap_or(&path)
                                        .display()
                                );
                                // call recursive function
                                recursive_ls(name.to_str().as_ref().unwrap(), args);
                            // if recursive option is not used
                            } else {
                                println!(
                                    "{}",
                                    path.strip_prefix(directory_path_input)
                                        .unwrap_or(&path)
                                        .display()
                                );
                            }
                        }
                        Err(e) => eprintln!("Failed to read file entry: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory: {:?}", e),
        }
    }
}
fn recursive_ls(directory_path_input: &str, args: &ArgMatches) {
    // initialize option variables
    let detailed_output_option = args.get_flag("detailed-output-option");
    let show_hidden_option = args.get_flag("show-hidden-option");
    let readable_option = args.get_flag("readable-option");
    let recursive_option = args.get_flag("recursive-option");
    let append_option = args.get_flag("append-option");

    match fs::read_dir(directory_path_input) {
        Ok(mut paths) => {
            while let Some(entry) = paths.next() {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();
                        let name = path.file_name().unwrap();
                        // if file is hidden file and show hidden option is not used
                        if !show_hidden_option
                            && name.to_str().map_or(false, |n| n.starts_with('.'))
                        {
                            continue;
                        }
                        // if detailed output option is used
                        if detailed_output_option {
                            // error handing for getting metadata
                            match path.metadata() {
                                Ok(meta) => {
                                    // initialize metadata variables
                                    let mode = meta.permissions().mode();
                                    let permission_string = format_permissions(mode);
                                    let file_type = if meta.is_dir() {
                                        "d"
                                    } else if meta.is_symlink() {
                                        "l"
                                    } else {
                                        "-"
                                    };
                                    let file_size = meta.len();
                                    // error handling for getting modification date
                                    match meta.modified() {
                                        Ok(mod_system_time) => {
                                            let mod_datetime: DateTime<Local> =
                                                mod_system_time.into();
                                            let owner = path.owner().unwrap();
                                            let group = path.group().unwrap();
                                            let link_number = meta.nlink();
                                            // final output
                                            // if readable option is used
                                            if readable_option {
                                                print!(
                                                    "{}{} {} {} {} {} {} ",
                                                    file_type,
                                                    permission_string,
                                                    link_number,
                                                    owner,
                                                    group,
                                                    format_size(file_size, DECIMAL),
                                                    mod_datetime.format("%d/%m/%Y %T")
                                                );
                                            // if readable option is not used
                                            } else {
                                                print!(
                                                    "{}{} {} {} {} {} {} ",
                                                    file_type,
                                                    permission_string,
                                                    link_number,
                                                    owner,
                                                    group,
                                                    file_size,
                                                    mod_datetime.format("%d/%m/%Y %T")
                                                );
                                            }
                                        }
                                        Err(e) => eprintln!(
                                            "Failed to get modification date of {}: {:?}",
                                            path.display().to_string(),
                                            e
                                        ),
                                    }
                                }
                                Err(e) => eprintln!(
                                    "Failed to get metadata of {}: {:?}",
                                    path.display().to_string(),
                                    e
                                ),
                            }
                        }
                        // if recursive option is used and path is directory
                        if recursive_option && path.is_dir() {
                            println!("");
                            println!(
                                "{}:",
                                path.strip_prefix(directory_path_input)
                                    .unwrap_or(&path)
                                    .display()
                            );
                            recursive_ls(name.to_str().as_ref().unwrap(), args);
                        }
                    }
                    Err(e) => eprintln!("Failed to read file entry: {:?}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory: {:?}", e),
    }
}

fn format_permissions(mode: u32) -> String {
    let mut permissions = String::new();

    // Owner permissions
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    // Group permissions
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    // Other permissions
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    permissions
}
