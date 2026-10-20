use crate::{GitOid, GithubRepo};
use std::sync::LazyLock;

#[derive(Copy, Clone, Debug)]
pub struct GithubCommit {
    pub repo: &'static LazyLock<GithubRepo>,
    pub oid: GitOid,
}

#[macro_export]
macro_rules! github_commit {
    ($name:ident, $repo:path, $oid:expr) => {
        $crate::__declare_lazy!(
            $name,
            $crate::GithubCommit,
            $crate::GithubCommit {
                repo: &$repo,
                oid: $oid
            }
        );
    };
}
