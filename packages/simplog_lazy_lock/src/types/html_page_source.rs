use derive_new::new;
use url::Url;

#[derive(new, Clone, Debug)]
pub struct HtmlPageSource {
    pub url: Url,
}

#[macro_export]
macro_rules! html_page_source {
    ($name:ident, $url:expr) => {
        $crate::__declare_lazy!($name, $crate::HtmlPageSource, $crate::HtmlPageSource::new($url));
    };
}
