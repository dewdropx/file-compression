# Rust File Compressor

A simple command-line file compression tool written in Rust using the `flate2` crate and the Gzip compression format.

## Features

- Compresses files using Gzip.
- Accepts source and target file paths as command-line arguments.
- Displays the original file size and compressed file size.
- Measures and displays the time taken for compression.
- Uses buffered reading for efficient file handling.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Usage

Clone the repository:

```bash
git clone https://github.com/dewdropx/file-compression.git
cd file-compression
```

Run the program:

```bash
cargo run <source-file> <target-file>
```

Example:

```bash
cargo run input.pdf output.gz
```

This compresses `input.pdf` into `output.gz`.

## Example Output

```text
source size: 102400
target size: 85432
elapsed time: 2.34ms
```

*Output values are illustrative and will vary depending on the input file and system.*

## Dependencies

- [`flate2`](https://crates.io/crates/flate2) - Gzip compression and decompression utilities.

## Limitations

- Currently supports compression only, decompression is not implemented.
- The output is a Gzip-compressed file, not a directly readable PDF or other original file format.
- Invalid file paths and I/O errors may cause the program to panic.
