use crate::GithubCommit;
use std::path::PathBuf;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
pub struct GithubSource {
    pub commit: &'static LazyLock<GithubCommit>,
    /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
    pub path: PathBuf,
    /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
    pub line_range: (usize, usize),
}

impl GithubSource {
    pub fn new(commit: &'static LazyLock<GithubCommit>, path: impl Into<PathBuf>, (start, end): (usize, usize)) -> Self {
        let line_range = if start <= end { (start, end) } else { (end, start) };
        Self {
            commit,
            path: path.into(),
            line_range,
        }
    }
}

#[macro_export]
macro_rules! github_source {
    ($name:ident, $commit:path, $path:expr, $line_range:expr) => {
        $crate::__declare_lazy!($name, $crate::GithubSource, $crate::GithubSource::new(&$commit, $path, $line_range));
    };
}
