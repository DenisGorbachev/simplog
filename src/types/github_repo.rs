use crate::impl_const_new;

#[derive(Copy, Clone, Debug)]
pub struct GithubRepo {
    pub owner: &'static str,
    pub name: &'static str,
}

impl_const_new!(GithubRepo, owner: &'static str, name: &'static str);

#[macro_export]
macro_rules! github_repo {
    ($name:ident, $owner:expr, $repo_name:expr) => {
        pub static $name: $crate::GithubRepo = $crate::GithubRepo::new($owner, $repo_name);
    };
}
