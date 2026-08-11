/// This enum defines the possible web pages to be opened.
#[derive(Debug, Clone)]
pub enum WebPage {
    /// Personal website.
    MySite,
}

impl WebPage {
    pub fn get_url(&self) -> &str {
        match self {
            WebPage::MySite => "https://nicolasjacques.com",
        }
    }
}
