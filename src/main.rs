use clap::Parser;
use std::fs::{self, DirEntry};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use args::FileType;
use std::collections::HashSet;
use glob::{MatchOptions, Pattern};

mod args;

fn main() {

    let args: args::Args = args::Args::parse();
    let path: PathBuf = args.path.into();
    let name: &str = args.name.as_deref().unwrap_or("");
    let types: HashSet<args::FileType> = args.types.into_iter().collect();
    let perm: Option<u16> = args.perm;
    let case_sensitive: bool = args.case_sensitive;
    let verbose: bool = args.verbose;
    
    println!("Agrs:\nPath: {:?} \nName: {} \nTypes: {:?} \nPerm: {:?} \nCase-Sensitive: {} \nVerbose: {} \n", 
    path, name, types, perm, case_sensitive, verbose);

    if let Err(e) = read_all(&path, name, &types, perm, case_sensitive, verbose) {
        if verbose {
            eprintln!("[ ERROR ] {:?}", e);
        }
    }
}

fn read_all(path: &Path, name: &str,
     types: &HashSet<args::FileType>, perm: Option<u16>,
    case_sensitive: bool, verbose: bool) -> Result<(), std::io::Error> {


    let entries: fs::ReadDir = match fs::read_dir(path) {
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

        let matches_perm = if let Some(user_perm) = perm {
            (entry.metadata()?.permissions().mode() & 0o777) == user_perm.into()
        } else {
            true
        };
        
        let matches_name = is_matches_name(name, &entry_path, case_sensitive);
        
        if file_type.is_dir() {
            if types.contains(&FileType::Dir) && (matches_perm || matches_name){
                println!("{}", entry_path.display());
            }
            read_all(&entry_path, name, types, perm, case_sensitive, verbose)?;
        } else {
            if types.contains(&FileType::File) && matches_perm && matches_name {
                println!("{}", entry_path.display());
            }
        }
    }

    Ok(())
}

fn is_matches_name(name: &str, entry_path: &PathBuf, case_sensitive: bool) -> bool {
    
    if name != "" {

        let pattern = match Pattern::new(name) {
            Ok(p) => p,
            Err(_) => return false,
        };

        let options = MatchOptions{
            case_sensitive: case_sensitive,
            require_literal_separator: true,
            ..Default::default()
        };

        entry_path.file_name()
            .and_then(|name| name.to_str())
            .map(|file_name| pattern.matches_with(file_name, options))
            .unwrap_or(false)
    }else {
        true
    }
}