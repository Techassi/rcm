use clap::Parser;
use snafu::Snafu;

use crate::cli::Cli;

mod cli;
mod git;

#[derive(Debug, Snafu)]
pub enum Error {
    InvalidUrl,
}

pub trait SnafuOptionExt<T>: Sized {
    fn context<C, E>(self, context: C) -> Result<T, E>
    where
        C: snafu::IntoError<E, Source = snafu::NoneError>,
        E: std::error::Error + snafu::ErrorCompat;
}

impl<I> SnafuOptionExt<(I, I)> for (Option<I>, Option<I>) {
    fn context<C, E>(self, context: C) -> Result<(I, I), E>
    where
        C: snafu::IntoError<E, Source = snafu::NoneError>,
        E: std::error::Error + snafu::ErrorCompat,
    {
        match (self.0, self.1) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(context.into_error(snafu::NoneError)),
        }
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    cli.run().unwrap();

    Ok(())
}
