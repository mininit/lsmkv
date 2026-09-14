mod lsmkv;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lsmkv", version)]
struct CommandLineArguments {
    paths: Vec<PathBuf>,
}

fn main() {
    let arguments = CommandLineArguments::parse();

    let starting_paths = if arguments.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        arguments.paths
    };

    let mut mkv_files = Vec::new();

    for path in &starting_paths {
        if path.is_dir() {
            lsmkv::find_mkv_files_in_directory(path, &mut mkv_files);
        } else if path.is_file() {
            mkv_files.push(path.clone());
        } else {
            eprintln!("No such file or directory: {}", path.display());
        }
    }

    for path in &mkv_files {
        lsmkv::print_mkv_info_line(path);
    }
}
