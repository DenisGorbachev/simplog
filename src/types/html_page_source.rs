use crate::impl_const_new;

#[derive(Copy, Clone, Debug)]
pub struct HtmlPageSource {
    pub url: &'static str,
}

impl_const_new!(HtmlPageSource, url: &'static str);

#[macro_export]
macro_rules! html_page_source {
    ($name:ident, $url:expr) => {
        pub static $name: $crate::HtmlPageSource = $crate::HtmlPageSource::new($url);
    };
}
