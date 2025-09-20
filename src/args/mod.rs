use clap::{Parser, ValueEnum};

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
    #[arg(long, required = false, value_parser = parse_perm)]
    pub perm: Option<u16>,

    /// Show error messages when directories can't be read.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

fn parse_perm(s: &str) -> Result<u16, String> {
    u16::from_str_radix(s, 8)
        .map_err(|e| format!("Invalid permission value: {}", e))
}

/// Represents the type of filesystem entry to search for.
///
/// This enum is used to filter the search results to include only
/// files, directories, or both.
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Hash)]
pub enum FileType {

    /// Search for files.
    #[clap(name = "file")]
    File,
    /// Search for directories.
    #[clap(name = "dir")]
    Dir,
}