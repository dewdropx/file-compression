extern crate flate2;

use flate2::write::GzEncoder; // for writing compressed data
use flate2::Compression; // for specifying compression level
use std::env::args; // for accessing command line arguments
use std::fs::File; // for file operations
use std::io::copy; // for copying data between streams
use std::io::BufReader; // for buffered reading of files
use std::time::Instant; // for elapsed time

fn main() {
    
}