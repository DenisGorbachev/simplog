use derive_new::new;
use url::Url;

#[derive(new, Clone, Debug)]
pub struct HtmlPageSource {
    pub url: Url,
}
