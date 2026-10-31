use chrono::{DateTime, Local}; // it is a Date and time library,
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt; // StuctOpt is allow us to generate automatically command line argument

//  the #[derive] that means the implementation will be generate by rust
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

pub fn ls_cli() {
    let options: Opt = Opt::from_args();
    // handle error if we call the run() method without this the rust complier will give error
    if let Err(ref e) = run(options.path) {
        println!("{}", e);
        process::exit(1);
    }
}

// the run function the Type is Result, it will fail or will success
fn run(directory: PathBuf) -> Result<(), Box<dyn Error>> {
    if directory.is_dir() {
        for enter in fs::read_dir(directory)? {
            let entry = enter?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid Entry: {:?}", f)))?;
            let metadata: fs::Metadata = entry.metadata()?;
            let get_permission: String = get_windows_permissions(&file_name);
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            // the {:>5} that mean after print there will be a 5 padding on left side a gap basically
            println!(
                "{:>5} {:>5} {} {}",
                get_permission,
                size,
                modified.format("%_d %b %H:%M").to_string(),
                file_name,
            );
        }
    }
    Ok(())
}

// because we work in window so we have add this to get the file permission in windows
fn get_windows_permissions(file_path: &str) -> String {
    if let Ok(metadata) = fs::metadata(file_path) {
        let permissions = metadata.permissions();
        if permissions.readonly() {
            return "r--".to_string();
        } else {
            return "rw--".to_string();
        }
    }

    "--".to_string()
}
