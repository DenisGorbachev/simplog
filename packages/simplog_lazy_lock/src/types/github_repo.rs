#[derive(Clone, Debug)]
pub struct GithubRepo {
    pub owner: String,
    pub name: String,
}

#[macro_export]
macro_rules! github_repo {
    ($name:ident, $owner:expr, $repo_name:expr) => {
        $crate::__declare_lazy!(
            $name,
            $crate::GithubRepo,
            $crate::GithubRepo {
                owner: ($owner).into(),
                name: ($repo_name).into()
            }
        );
    };
}
