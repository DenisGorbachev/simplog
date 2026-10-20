use crate::{GithubSource, HtmlPageSource};
use derive_more::From;

#[derive(From, Clone, Debug)]
pub enum Source {
    Github(GithubSource),
    HtmlPage(HtmlPageSource),
}

#[macro_export]
macro_rules! source_github {
    ($name:ident, $commit:path, $path:expr, $line_range:expr) => {
        $crate::lazy_lock!($name, $crate::Source, {
            let (start, end) = $line_range;
            $crate::Source::Github($crate::GithubSource::new(&$commit, $path, if start <= end { (start, end) } else { (end, start) }))
        });
    };
}

#[macro_export]
macro_rules! source_html_page {
    ($name:ident, $url:tt) => {
        $crate::lazy_lock!($name, $crate::Source, $crate::Source::HtmlPage($crate::HtmlPageSource::new($crate::__url!($url))));
    };
}
