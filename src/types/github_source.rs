use crate::GithubCommit;

#[derive(Copy, Clone, Debug)]
pub struct GithubSource {
    pub commit: &'static GithubCommit,
    /// Caller guarantees a nonempty repository-relative path without `..` components, identifying a file in `commit`.
    pub path: &'static str,
    /// Inclusive, one-based bounds. Caller guarantees `1 <= start <= end <= line_count` for the file in `commit`.
    pub line_range: (usize, usize),
}
