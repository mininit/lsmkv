use clap::Parser;
use matroska::{Matroska, Tracktype};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "lsmkv", version, about)]
struct CommandLineArguments {
    paths: Vec<PathBuf>,
}

fn is_mkv_file(path: &Path) -> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()), Some("mkv"))
}

fn find_mkv_files_in_directory(directory: &Path, found_files: &mut Vec<PathBuf>) {
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if entry.file_type().is_file() && is_mkv_file(path) {
            found_files.push(path.to_path_buf());
        }
    }
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{size:.0}{}", UNITS[unit_index])
    } else {
        format!("{size:.1}{}", UNITS[unit_index])
    }
}

fn format_codec_name(codec_id: &str) -> &str {
    match codec_id {
        "V_MPEG4/ISO/AVC" => "H.264",
        "V_MPEGH/ISO/HEVC" => "H.265",
        "V_VP9" => "VP9",
        "V_VP8" => "VP8",
        "V_AV1" => "AV1",
        other => other,
    }
}

fn find_video_codec(mkv: &Matroska) -> &str {
    mkv.tracks
        .iter()
        .find(|track| matches!(track.tracktype, Tracktype::Video))
        .map(|track| format_codec_name(&track.codec_id))
        .unwrap_or("unknown")
}

fn print_mkv_info(path: &Path) {
    let display_path = path.display();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Couldn't open {display_path}: {error}");
            return;
        }
    };

    let file_size = match file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(error) => {
            eprintln!("Couldn't stat {display_path}: {error}");
            return;
        }
    };

    let mkv = match Matroska::open(BufReader::new(file)) {
        Ok(mkv) => mkv,
        Err(error) => {
            eprintln!("Couldn't parse {display_path}: {error:?}");
            return;
        }
    };

    let Some(duration) = mkv.info.duration else {
        eprintln!("No duration in {display_path}");
        return;
    };

    let duration_seconds = duration.as_secs_f64();
    let megabits_per_second = if duration_seconds > 0.0 {
        file_size as f64 * 8.0 / duration_seconds / 1_000_000.0
    } else {
        0.0
    };

    println!(
        "{:>6.1}Mbps {:>7} {:>6.1}min {:>5} {display_path}",
        megabits_per_second,
        format_file_size(file_size),
        duration_seconds,
        find_video_codec(&mkv),
    );
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
            find_mkv_files_in_directory(path, &mut mkv_files);
        } else if path.is_file() {
            mkv_files.push(path.clone());
        } else {
            eprintln!("Not found: {}", path.display());
        }
    }

    for path in &mkv_files {
        print_mkv_info(path);
    }
}
