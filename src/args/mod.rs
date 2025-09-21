use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use glob::Pattern;
use glob::MatchOptions;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;

/// A command-line tool for finding files and directories in a filesystem.
///
/// This tool allows users to search for files and directories by name, type,
/// and permissions, with an option for a case-insensitive search.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {

    /// The path to the directory to start searching from. Defaults to the current directory.
    #[arg(short, long, default_value_t = String::from("."))]
    pub path: String,

    /// Filter the search by file type. Can be used multiple times to search for different types.
    #[arg(short, long, default_values = ["dir", "file"])]
    pub types: Vec<FileType>,

    /// The name of the file or directory to search for.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Perform a case-sensitive search (default: true).
    #[arg(short, long, default_value_t = true)]
    pub case_sensitive: bool,

    /// Filter the search by file permissions (octal format).
    #[arg(long, required = false)]
    pub perm: Option<String>,

    /// Show error messages when directories can't be read.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Show files executable by others (permission bit --x for others).
    #[arg(long, default_value_t = false)]
    pub exec_other: bool,

    #[arg(long, default_value_t = false)]
    pub suid: bool,

    #[arg(long, default_value_t = false)]
    pub sgid: bool,

    #[arg(long, default_value_t = false)]
    pub owned_by_root: bool,

}

impl Args {

    pub fn matches_perm(&self, perm: &Option<String>, mode: u32) -> bool {
        match perm {
            Some(p) if p.starts_with('/') => {
                parse_octal(&p[1..]).map_or(false, |mask: u32| mode & mask != 0)
            }
            Some(p) => {
                parse_octal(&p).map_or(false, |exact: u32| (mode & 0o777) == exact)
            }
            None => true,
        }
    }

    pub fn matches_name(&self, entry_path: &PathBuf,) -> bool {
        
        let name: &str = self.name.as_deref().unwrap_or("");

        if name != "" {

            let pattern = match Pattern::new(name) {
                Ok(p) => p,
                Err(_) => return false,
            };

            let options = MatchOptions{
                case_sensitive: self.case_sensitive,
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

    pub fn matches_flags(&self, mode: u32, entry: &DirEntry) -> bool {

        if !(self.suid || self.sgid || self.exec_other || self.owned_by_root) {
            return true;
        }

        if self.suid && mode & 0o4000 == 0 {
            return false;
        }
        if self.sgid && mode & 0o2000 == 0 {
            return false;
        }
        if self.exec_other && mode & 0o001 == 0 {
            return false;
        }
        if self.owned_by_root && !self.is_owned_by_root(entry) {
            return false;
        }
        true
    }

    pub fn is_owned_by_root(&self, entry: &DirEntry) -> bool {
        match entry.metadata() {
            Ok(meta) => meta.uid() == 0,
            Err(_) => false, // si no se puede leer, no lo consideramos root
        }
    }


    pub fn matches_all(&self, entry: &DirEntry,) -> bool {

        let path: PathBuf = entry.path();
        let mode: u32 = entry.metadata()
        .map(|m| m.permissions().mode())
        .unwrap_or(0);

        let name_ok: bool = self.matches_name(&path);
        let perm_ok: bool = self.matches_perm(&self.perm, mode);
        let flags_ok: bool = self.matches_flags(mode, entry);

        name_ok && perm_ok && flags_ok
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            path: ".".to_string(),
            types: vec![FileType::File],
            name: None,
            case_sensitive: true,
            perm: None,
            verbose: false,
            exec_other: false,
            suid: false,
            sgid: false,
            owned_by_root: false,
        }
    }
}


fn parse_octal(s: &str) -> Option<u32> {
    u32::from_str_radix(s, 8).ok().filter(|v| *v <= 0o7777)
}


/// Represents the type of filesystem entry to search for.
///
/// This enum is used to filter the search results to include only
/// files, directories, or both.
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Hash, Copy)]
pub enum FileType {

    /// Search for files.
    #[clap(name = "file")]
    File,
    /// Search for directories.
    #[clap(name = "dir")]
    Dir,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_matches_name_exact() {
        let args = Args {
            name: Some("file.txt".to_string()),
            case_sensitive: true,
            ..Default::default()
        };
        let path = PathBuf::from("file.txt");
        assert!(args.matches_name(&path));
    }

    #[test]
    fn test_matches_name_pattern() {
        let args = Args {
            name: Some("*.txt".to_string()),
            case_sensitive: true,
            ..Default::default()
        };
        let path = PathBuf::from("report.txt");
        assert!(args.matches_name(&path));
    }

    #[test]
    fn test_matches_perm_exact() {
        let args = Args {
            perm: Some("644".to_string()),
            ..Default::default()
        };
        assert!(args.matches_perm(&args.perm, 0o100644));
    }

    #[test]
    fn test_matches_perm_mask() {
        let args = Args {
            perm: Some("/4000".to_string()),
            ..Default::default()
        };
        assert!(args.matches_perm(&args.perm, 0o4755));
    }

    #[test]
    fn test_matches_flags_suid_only() {
        let args = Args {
            suid: true,
            ..Default::default()
        };
        let dummy_mode = 0o4755;
        let entry = dummy_entry_with_mode(dummy_mode);
        assert!(args.matches_flags(dummy_mode, &entry));
    }

    fn dummy_entry_with_mode(mode: u32) -> std::fs::DirEntry {

        use std::fs::{File};
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;
        use std::path::Path;

        let path = Path::new("/tmp/test_rfind_dummy");
        let _ = File::create(&path).and_then(|mut f| f.write_all(b"dummy"));
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode));
        std::fs::read_dir("/tmp")
            .unwrap()
            .find(|e| e.as_ref().unwrap().path() == path)
            .unwrap()
            .unwrap()
    }
}
