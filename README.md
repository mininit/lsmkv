# lsmkv

A CLI tool for auditing Matroska `.mkv` files. `lsmkv` reports each file's average bitrate, size, duration, and codec.

Metadata is read directly from the `.mkv` container without decoding, so it stays fast across large libraries.

## Features

- Recursively scans directories for `.mkv` files
- Reports average bitrate, size, duration, and codec for each
- Accepts directories and individual files as command line arguments

## Installation

### From source

```bash
git clone https://github.com/mininit/lsmkv.git
cd lsmkv
cargo build --release
```

The binary will be at `target/release/lsmkv`.

### Via Homebrew

```bash
brew install mininit/tap/lsmkv
```

## Usage

```
lsmkv [OPTIONS] [PATHS]...
```

If no path is given, `lsmkv` recursively scans the current directory.

### Arguments

| Argument | Description                                              |
|----------|----------------------------------------------------------|
| `PATHS`  | Files or directories to scan. Defaults to `.` if omitted |

### Options

| Flag                     | Description                                |
|--------------------------|--------------------------------------------|
| `-h`, `--help`           | Print help                                 |
| `-V`, `--version`        | Print version                              |

### Output columns

| Column   | Description                             |
|----------|------------------------------------------|
| Bitrate  | Average bitrate in Mbps                  |
| Size     | File size (binary units, e.g. `25.7M`)   |
| Duration | Runtime in minutes                       |
| Codec    | Video codec (e.g. `AV1`, `H.265`)        |

### Examples

Recursively scan the current directory:
```bash
lsmkv
```

Recursively scan a media library:
```bash
lsmkv ~/Videos
```

Check specific files:
```bash
lsmkv video_1.mkv video_2.mkv
```

Mixed files and directories:
```bash
lsmkv video_1.mkv ~/Videos
```

### Sample output

```bash
$ lsmkv .
  18.4Mbps   25.7M   11.7min   AV1 ./video_3.mkv
   5.7Mbps   11.0M   16.1min H.265 ./video_1.mkv
   3.4Mbps    7.8M   19.1min H.264 ./video_2.mkv
```

### Sorting

`lsmkv` streams output instantly and does not sort. Pipe through `sort` instead:

```bash
lsmkv ~/Videos | sort -k1,1 -rn   # highest bitrate first
```