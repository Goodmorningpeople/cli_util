use chrono::{self, DateTime, Local};
use clap::ArgMatches;
use file_owner::PathExt;
use humansize::{format_size, DECIMAL};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

pub fn match_ls(ls_args: Option<&ArgMatches>) {
    if let Some(args) = ls_args {
        let dir_path = args
            .get_one::<String>("directory-path-input")
            .map_or("./", |s| s.as_str());

        let paths = fs::read_dir(dir_path).expect("Directory path is invalid!");

        for entry in paths {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if let Some(name) = path.file_name() {
                // Skip hidden files if "show-hidden-option" is not enabled
                if !args.get_flag("show-hidden-option")
                    && name.to_str().map_or(false, |n| n.starts_with('.'))
                {
                    continue;
                }

                if args.get_flag("detailed-output-option") {
                    let metadata = path.metadata().unwrap();
                    let permissions = metadata.permissions();
                    let mode = permissions.mode();
                    let permission_string = format_permissions(mode);
                    let file_type = if metadata.is_dir() {
                        "d"
                    } else if metadata.is_symlink() {
                        "l"
                    } else {
                        "-"
                    };
                    let file_size = metadata.len();
                    let mod_system_time = metadata.modified().unwrap();
                    let mod_datetime: DateTime<Local> = mod_system_time.into();
                    let owner = path.owner().unwrap();
                    let group = path.group().unwrap();
                    let link_number = metadata.nlink();

                    if args.get_flag("readable-option") {
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
                if args.get_flag("append-option") {
                    let metadata = path.metadata().unwrap();
                    let mode = metadata.permissions().mode();
                    if metadata.is_dir() {
                        print!("/");
                    } else if metadata.is_symlink() {
                        print!("@");
                    } else if mode & 0o100 != 0 || mode & 0o010 != 0 || mode & 0o001 != 0 {
                        print!("*");
                    }
                }
                if path.is_dir() && args.get_flag("recursive-option") {
                    println!("");
                    println!(
                        "{}:",
                        path.strip_prefix(dir_path).unwrap_or(&path).display()
                    );
                    recursive_ls(&path, args);
                } else {
                    println!("{}", path.strip_prefix(dir_path).unwrap_or(&path).display());
                }
            }
        }
    }
}

fn recursive_ls(dir_path: &std::path::Path, args: &ArgMatches) {
    let paths = fs::read_dir(dir_path).expect("Directory path is invalid!");

    for entry in paths {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if let Some(name) = path.file_name() {
            // Skip hidden files if "show-hidden-option" is not enabled
            if !args.get_flag("show-hidden-option")
                && name.to_str().map_or(false, |n| n.starts_with('.'))
            {
                continue;
            }
            // Recurse into directories if "recursive-option" is enabled
            if path.is_dir() {
                println!("");
                println!(
                    "{}:",
                    path.strip_prefix(dir_path).unwrap_or(&path).display()
                );
                recursive_ls(&path, args);
            } else {
                println!("{}", path.strip_prefix(dir_path).unwrap_or(&path).display());
            }
        }
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
