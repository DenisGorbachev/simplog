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
