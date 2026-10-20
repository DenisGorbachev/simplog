use crate::GithubCommit;

#[derive(Copy, Clone, Debug)]
pub struct GithubSource {
    pub commit: &'static GithubCommit,
    /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
    pub path: &'static str,
    /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
    pub line_range: (usize, usize),
}

impl GithubSource {
    pub const fn new(commit: &'static GithubCommit, path: &'static str, (start, end): (usize, usize)) -> Self {
        let line_range = if start <= end { (start, end) } else { (end, start) };
        Self {
            commit,
            path,
            line_range,
        }
    }
}

#[macro_export]
macro_rules! github_source {
    ($name:ident, $commit:expr, $path:expr, $line_range:expr) => {
        pub static $name: $crate::GithubSource = $crate::GithubSource::new($commit, $path, $line_range);
    };
}
