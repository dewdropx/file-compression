extern crate flate2;

use flate2::write::GzEncoder; // for writing compressed data
use flate2::Compression; // for specifying compression level
use std::env::args; // for accessing command line arguments
use std::fs::File; // for file operations
use std::io::copy; // for copying data between streams
use std::io::BufReader; // for buffered reading of files
use std::time::Instant; // for elapsed time

fn main() {

    // check valid input argument
    if args().len() != 3 {
        eprintln!("usage: `source` `target`");
        return;
    }

    // get the input and output files
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // initialize gzip encoder
    let mut encoder = GzEncoder::new(output, Compression::default());

    // elapsed time
    let start = Instant::now();

    // copy contents to compressed file
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "source size: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!(
        "target size: {:?}",
        output.metadata().unwrap().len()
    );

    println!("elapsed time: {:?}", start.elapsed());
}