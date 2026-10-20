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
        $crate::__declare_lazy!($name, $crate::Source, {
            let (start, end) = $line_range;
            $crate::Source::Github($crate::GithubSource {
                commit: &$commit,
                path: ($path).into(),
                line_range: if start <= end { (start, end) } else { (end, start) },
            })
        });
    };
}

#[macro_export]
macro_rules! source_html_page {
    ($name:ident, $url:tt) => {
        $crate::__declare_lazy!(
            $name,
            $crate::Source,
            $crate::Source::HtmlPage($crate::HtmlPageSource {
                url: $crate::__url!($url)
            })
        );
    };
}
