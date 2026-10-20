use crate::{GithubRepo, impl_const_new};

#[derive(Copy, Clone, Debug)]
pub struct GithubCommit {
    pub repo: &'static GithubRepo,
    pub oid: &'static str,
}

impl_const_new!(GithubCommit, repo: &'static GithubRepo, oid: &'static str);

#[macro_export]
macro_rules! github_commit {
    ($name:ident, $repo:expr, $oid:expr) => {
        pub static $name: $crate::GithubCommit = $crate::GithubCommit::new($repo, $oid);
    };
}
