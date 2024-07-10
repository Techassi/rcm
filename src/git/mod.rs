use gix::url::{parse::Error, Url};

mod clone;

pub trait ToGitUrl {
    fn to_git_url(self) -> Result<Url, Error>;
}

impl ToGitUrl for &str {
    fn to_git_url(self) -> Result<Url, Error> {
        gix::url::parse(self.into())
    }
}
