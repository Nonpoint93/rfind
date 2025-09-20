use clap::Parser;
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use args::FileType;
use std::collections::HashSet;

mod args;

fn main() {

    let args: args::Args = args::Args::parse();

    let path: PathBuf = PathBuf::from(&args.path);

    if let Err(e) = read_all(&args, &path) {
        if args.verbose {
            eprintln!("[ ERROR ] {:?}", e);
        }
    }
}

fn read_all(args: &args::Args, path: &PathBuf) -> Result<(), std::io::Error> {

    let types: HashSet<args::FileType> = args.types.clone().into_iter().collect();
    //let perm: Option<String> = args.perm.clone();
    let verbose: bool = args.verbose;

    let entries: fs::ReadDir = match fs::read_dir(&path) {
        Ok(e) => e,
        Err(e) => {
            if verbose {
                eprintln!("[ ERROR ] Failed to read directory {}: {}", path.display(), e);
            }
            return Ok(());
        }
    };

    for entry in entries {

       let entry: DirEntry = match entry {
            Ok(e) => e,
            Err(e) => {
                if verbose {
                    eprintln!("[ ERROR ] Failed to read entry: {}", e);
                }
                continue;
            }
        };

        let entry_path: PathBuf = entry.path();        
        let file_type: fs::FileType = entry.file_type()?;

        //let mode: u32 = entry.metadata()?.permissions().mode();
        
        //let matches_perm: bool = args.matches_perm(&perm, mode);
        //let matches_name: bool = args.matches_name(&entry_path);
        
        if file_type.is_dir() {
            if types.contains(&FileType::Dir) && args.matches_all(&entry) {
                println!("{}", entry_path.display());
            }
            read_all(args, &entry.path())?;
        } else {
            if types.contains(&FileType::File) && args.matches_all(&entry) {
                println!("{}", entry_path.display());
            }
        }
    }

    Ok(())
}
