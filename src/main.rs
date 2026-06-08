use std::path::PathBuf;
use std::process::ExitCode;

use exiftool_rs::cli::{self, Action};
use exiftool_rs::tag::ExtractedTag;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("{}", cli::HELP);
        return ExitCode::FAILURE;
    }

    let action = match cli::parse_args(&args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {e}\n\n{}", cli::HELP);
            return ExitCode::FAILURE;
        }
    };

    let opts = match action {
        Action::Version => {
            println!("{}", exiftool_rs::VERSION);
            return ExitCode::SUCCESS;
        }
        Action::Help => {
            println!("{}", cli::HELP);
            return ExitCode::SUCCESS;
        }
        Action::Write { files, edits, overwrite_original } => {
            let had_error = cli::run_write(&files, &edits, overwrite_original);
            return if had_error { ExitCode::FAILURE } else { ExitCode::SUCCESS };
        }
        Action::Run(o) => o,
    };

    let multi = opts.files.len() > 1;
    let mut had_error = false;
    let mut json_files: Vec<(PathBuf, Vec<ExtractedTag>)> = Vec::new();

    for path in &opts.files {
        match exiftool_rs::extract_from_path(path) {
            Ok(tags) => {
                if opts.json {
                    json_files.push((path.clone(), tags));
                } else {
                    if multi {
                        cli::print_file_header(path);
                    }
                    cli::print_human(&tags, &opts);
                }
            }
            Err(e) => {
                had_error = true;
                if opts.json {
                    eprintln!("Error: {} - {}", path.display(), e);
                } else {
                    println!("Error: {} - {}", path.display(), e);
                }
            }
        }
    }

    if opts.json {
        cli::print_json(&json_files, &opts);
    }

    if had_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
