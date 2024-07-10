use std::path::PathBuf;

use gix::{interrupt::IS_INTERRUPTED, progress::Discard, url::Url, Repository};
use snafu::{ResultExt, Snafu};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to prepare git clone"))]
    PrepareClone { source: gix::clone::Error },

    #[snafu(display("failed to fetch and checkout git repository"))]
    FetchAndCheckout { source: gix::clone::fetch::Error },

    #[snafu(display("failed to checkout main worktree"))]
    CheckoutWorktree {
        source: gix::clone::checkout::main_worktree::Error,
    },
}

pub fn from_remote(repository_url: Url, destination_path: PathBuf) -> Result<Repository> {
    let mut prepare_fetch =
        gix::prepare_clone(repository_url, destination_path).context(PrepareCloneSnafu)?;

    let (mut prepare_checkout, _) = prepare_fetch
        .fetch_then_checkout(Discard, &IS_INTERRUPTED)
        .context(FetchAndCheckoutSnafu)?;

    let (repo, _) = prepare_checkout
        .main_worktree(Discard, &IS_INTERRUPTED)
        .context(CheckoutWorktreeSnafu)?;

    Ok(repo)
}
