use std::fmt::{Display, Formatter, self};

#[derive(Copy, Clone)]
pub(crate) enum Page {
    BasePage,
    LoginPage,
    LoginRedirectPage
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let representation = match self {
            Page::BasePage=> "https://www.mytischtennis.de",
            Page::LoginPage=> "community/login",
            Page::LoginRedirectPage=> "public/home?fromlogin=1&logout=true"
        };

        write!(f, "{}", representation)
    }
}

pub(crate) fn get_page(page: Page) -> String {
    format!("{}/{}", Page::BasePage, page)
}