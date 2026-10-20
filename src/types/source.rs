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
        pub static $name: $crate::Source = {
            let (start, end) = $line_range;
            $crate::Source::Github($crate::GithubSource {
                commit: $commit,
                path: $path,
                line_range: if start <= end { (start, end) } else { (end, start) },
            })
        };
    };
}

#[macro_export]
macro_rules! source_html_page {
    ($name:ident, $url:tt) => {
        pub static $name: $crate::Source = $crate::Source::HtmlPage($crate::HtmlPageSource {
            url: $url,
        });
    };
}
