use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error;
use std::ffi::OsStr;

mod args;

fn main() {
    let args: args::Args = args::Args::parse();
    let path: PathBuf = args.path.into();
    let name: &str = &args.name;
    
    println!("Agrs: Path: {:?} Name: {}", path, name);
    if let Err(e) = read_all(&path, name) {
        eprintln!("[ ERROR ] Failed to read directory: {}", e);
    }
}

fn read_all(path: &Path, name: &str) -> Result<(), Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            read_all(&path, name)?;
        } else {
            if let Some(file_name) = path.file_name(){
                if file_name == OsStr::new(name) {
                    println!("{}", path.display());
                }
            }
            
        }
    }
    Ok(())
}