use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use directories::{ProjectDirs, UserDirs};
use snafu::Snafu;

use crate::{
    git::{self, GitUrlIntoParts, ToGitUrl},
    SnafuOptionExt,
};

#[derive(Debug, Snafu)]
pub enum Error {
    InvalidUrl,
}

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Path to the config file
    #[arg(short, long, default_value_os_t = default_config_file_path())]
    pub config: PathBuf,

    /// Root path where remote repositories get cloned to
    #[arg(short, long = "root", default_value_os_t = default_root_path())]
    pub root_path: PathBuf,

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

impl Cli {
    pub fn run(&self) -> Result<(), Error> {
        match &self.command {
            Commands::Get(args) => args.run(self)?,
            Commands::List(args) => args.run(self)?,
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clone a remote repository into the rcm root directory
    #[command(visible_alias = "clone")]
    Get(GetArgs),

    /// List locally clones repositories
    #[command(visible_alias = "ls")]
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Remote repository url
    pub repository_url: String,

    /// Detect the language of the repository and clone the repository into '<ROOT>/<LANGUAGE>/<PATH>'
    #[arg(short = 'l', long = "language")]
    pub use_language_dir: bool,
}

impl GetArgs {
    pub fn run(&self, cli: &Cli) -> Result<(), Error> {
        let url = self.repository_url.as_str().to_git_url().unwrap();
        let (host, path) = url.parts_owned().context(InvalidUrlSnafu)?;

        let path = path.trim_end_matches(".git");

        println!("Host: {host}, Path: {path}");

        let final_path = cli.root_path.join(host.to_lowercase()).join(path);

        if !final_path.exists() {
            // Create dir
            std::fs::create_dir_all(final_path.clone()).unwrap();
            git::clone::from_remote(url, final_path.clone()).unwrap();
            // If fails, remove dir
        } else {
            // Check if git repo, ask to fetch or pull
            println!("Exists, exiting.")
        }

        println!("{}", final_path.display());
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Print full paths to the repository root instead of relative ones
    #[arg(short = 'p', long)]
    full_path: bool,
}

impl ListArgs {
    pub fn run(&self, cli: &Cli) -> Result<(), Error> {
        let read_dir = std::fs::read_dir(&cli.root_path).unwrap();

        let entries = read_dir
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        for entry in entries {
            println!("{}", entry.display())
        }

        Ok(())
    }
}
