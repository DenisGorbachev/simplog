use crate::{GithubSource, HtmlPageSource};
use derive_more::From;

#[derive(From, Copy, Clone, Debug)]
pub enum Source {
    Github(GithubSource),
    HtmlPage(HtmlPageSource),
}

#[macro_export]
macro_rules! source_github {
    ($name:ident, $commit:expr, $path:expr, $line_range:expr) => {
        pub static $name: $crate::Source = $crate::Source::Github($crate::GithubSource::new($commit, $path, $line_range));
    };
}

#[macro_export]
macro_rules! source_html_page {
    ($name:ident, $url:expr) => {
        pub static $name: $crate::Source = $crate::Source::HtmlPage($crate::HtmlPageSource::new($url));
    };
}
