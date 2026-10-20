use crate::{GitOid, GithubRepo};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Copy, Clone, Debug)]
pub struct GithubCommit {
    pub repo: &'static LazyLock<GithubRepo>,
    pub oid: GitOid,
}

#[macro_export]
macro_rules! github_commit {
    ($name:ident, $repo:path, $oid:expr) => {
        $crate::lazy_lock!($name, $crate::GithubCommit, $crate::GithubCommit::new(&$repo, $oid));
    };
}
