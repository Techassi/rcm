use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use directories::{ProjectDirs, UserDirs};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Path to the config file
    #[arg(short, long, default_value_os_t = default_config_file_path())]
    pub config: PathBuf,

    /// Root path where remote repositories get cloned to
    #[arg(short, long = "root", default_value_os_t = default_root_path())]
    root_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

fn default_config_file_path() -> PathBuf {
    let project_dir = ProjectDirs::from("dev.techassi", "Techassi", "rcm")
        .expect("failed to retrieve XDG project directory");

    project_dir.config_dir().to_path_buf().join("config.toml")
}

fn default_root_path() -> PathBuf {
    let user_dir = UserDirs::new().expect("failed to retrieve XDG user directory");
    user_dir.home_dir().join("src")
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clone a remote repository into the rcm root directory
    #[command(visible_alias = "clone")]
    Get(GetArgs),
}

#[derive(Debug, Args)]
pub struct GetArgs {
    pub repository_url: String,
}
