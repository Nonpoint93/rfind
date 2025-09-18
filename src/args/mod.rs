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
    #[arg(short, long)]
    pub types: Vec<FileType>,

    /// The name of the file or directory to search for.
    #[arg(short, long)]
    pub name: String,

    /// Perform a case-insensitive search.
    #[arg(short, long, required = false, default_value_t = false)]
    pub ignore_case: bool,

    /// Filter the search by file permissions (octal format).
    #[arg(long, required = false)]
    pub perm: Option<u16>,
}

/// Represents the type of filesystem entry to search for.
///
/// This enum is used to filter the search results to include only
/// files, directories, or both.
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum FileType {

    /// Search for files.
    #[clap(name = "file")]
    File,
    /// Search for directories.
    #[clap(name = "dir")]
    Dir,
}