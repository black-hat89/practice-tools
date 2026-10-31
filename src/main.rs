use std::env::args;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn check_file(path: &str) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(info) => {
            if info.is_file() {
                Ok(())
            } else {
                Err(format!("'{}' is not a file ", path))
            }
        }
        Err(_) => Err(format!("Cannot find '{}'", path)),
    }
}

fn count_file(path: &str) -> Result<(usize, usize), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total_line: usize = 0;
    let mut total_words: usize = 0;

    for line in reader.lines() {
        let line = line?;
        total_line += 1;
        total_words += line.split_whitespace().count();
    }

    Ok((total_line, total_words))
}

fn write_csv(output_path: &str, lines: usize, words: usize) -> Result<(), io::Error> {
    let content = format!("name,value\ntotal_lines,{}\ntotal_words,{}\n", lines, words);

    fs::write(output_path, content)
}
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Please give me some input.");
        println!("Example: cargo run -- hello world");
        return;
    }
    let path = &args[1];

    match check_file(path) {
        Ok(()) => println!("Good news the file '{}' exists.", path),
        Err(message) => println!("Error: {}", message),
    }

    match count_file(path) {
        Ok((lines, words)) => {
            println!("Total lines: {}", lines);
            println!("Total words: {}", words);

            match write_csv("report.csv", lines, words) {
                Ok(()) => println!("Saved results to report.csv"),
                Err(error) => println!("Could not write the CSV file: {}", error),
            }
        }
        Err(error) => println!("Could not read the file: {}", error),
    }
}
