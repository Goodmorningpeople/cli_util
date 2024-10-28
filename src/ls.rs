use chrono::{self, DateTime, Local};
use clap::ArgMatches;
use file_owner::PathExt;
use humansize::{format_size, DECIMAL};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

pub fn match_ls(ls_args: Option<&ArgMatches>) {
    if let Some(args) = ls_args {
        // Initialize option variables
        let detailed_output_option = args.get_flag("detailed-output-option");
        let show_hidden_option = args.get_flag("show-hidden-option");
        let readable_option = args.get_flag("readable-option");
        let recursive_option = args.get_flag("recursive-option");
        let append_option = args.get_flag("append-option");

        // Check if directory path has been specified, default to current dir if not
        let directory_path_input = args
            .get_one::<String>("directory-path-input")
            .map_or("./", |s| s.as_str());

        // Call the recursive function
        recursive_ls(
            Path::new(directory_path_input),
            args,
            detailed_output_option,
            show_hidden_option,
            readable_option,
            append_option,
            recursive_option,
        );
    }
}

fn recursive_ls(
    directory_path: &Path,
    args: &ArgMatches,
    detailed_output: bool,
    show_hidden: bool,
    readable: bool,
    append: bool,
    recursive: bool,
) {
    match fs::read_dir(directory_path) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();
                        let name = path.file_name().unwrap();

                        // If file is hidden and show hidden option is not used
                        if !show_hidden && name.to_str().map_or(false, |n| n.starts_with('.')) {
                            continue;
                        }

                        // If detailed output option is used
                        if detailed_output {
                            if let Ok(meta) = path.metadata() {
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

                                if let Ok(mod_system_time) = meta.modified() {
                                    let mod_datetime: DateTime<Local> = mod_system_time.into();
                                    let owner = path.owner().unwrap();
                                    let group = path.group().unwrap();
                                    let link_number = meta.nlink();

                                    // Final output
                                    if readable {
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
                            }
                        }

                        // If append option is used
                        if append {
                            let meta = path.metadata().unwrap();
                            let mode = meta.permissions().mode();
                            if meta.is_dir() {
                                print!("/");
                            } else if meta.is_symlink() {
                                print!("@");
                            } else if mode & 0o100 != 0 || mode & 0o010 != 0 || mode & 0o001 != 0 {
                                print!("*");
                            }
                        }

                        // If recursive option is used and path is a directory
                        if recursive && path.is_dir() {
                            println!("");
                            println!("{}:", path.strip_prefix(directory_path).unwrap_or(&path).display());
                            recursive_ls(
                                &path,
                                args,
                                detailed_output,
                                show_hidden,
                                readable,
                                append,
                                recursive,
                            );
                        } else {
                            println!(
                                "{}",
                                path.strip_prefix(directory_path).unwrap_or(&path).display()
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
