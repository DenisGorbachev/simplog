use derive_new::new;

#[derive(new, Clone, Debug)]
pub struct GithubRepo {
    #[new(into)]
    pub owner: String,
    #[new(into)]
    pub name: String,
}

#[macro_export]
macro_rules! github_repo {
    ($name:ident, $owner:expr, $repo_name:expr) => {
        $crate::__declare_lazy!($name, $crate::GithubRepo, $crate::GithubRepo::new($owner, $repo_name));
    };
}
