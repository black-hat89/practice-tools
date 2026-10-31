use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

mod file_compression;

fn main() {
    file_compression::compress_file();
}
