use gix::{
    bstr::BStr,
    url::{parse::Error, Url},
};

pub mod clone;

pub trait ToGitUrl {
    fn to_git_url(self) -> Result<Url, Error>;
}

impl ToGitUrl for &str {
    fn to_git_url(self) -> Result<Url, Error> {
        gix::url::parse(self.into())
    }
}

impl ToGitUrl for String {
    fn to_git_url(self) -> Result<Url, Error> {
        gix::url::parse(self.as_str().into())
    }
}

pub trait GitUrlIntoParts {
    fn parts(&self) -> (Option<&str>, Option<&BStr>);
    fn parts_owned(&self) -> (Option<String>, Option<String>);
}

impl GitUrlIntoParts for Url {
    fn parts(&self) -> (Option<&str>, Option<&BStr>) {
        (self.host_argument_safe(), self.path_argument_safe())
    }

    fn parts_owned(&self) -> (Option<String>, Option<String>) {
        let (host, path) = self.parts();
        (host.map(|h| h.to_owned()), path.map(|p| p.to_string()))
    }
}
