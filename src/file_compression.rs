use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

pub fn compress_file() {
    if args().len() != 3 {
        println!("Error Source and target is missing!!: `Source` `target`");
        return;
    } else {
        println!("Success");
    }
    let input_file = File::open(args().nth(1).unwrap()).unwrap();

    let mut input = BufReader::new(input_file);
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start: Instant = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("Source Len: {:?}", input.get_ref().metadata().unwrap());
    println!("target Len: {:?}", output.metadata().unwrap().len());
    println!("Elapssed: {:?}", start.elapsed());
}
