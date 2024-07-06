use std::fmt::{self, Display, Formatter};

const BASE_PAGE: &str = "https://www.mytischtennis.de";

#[derive(Copy, Clone)]
pub(crate) enum Page {
    LoginPage,
    LoginRedirectPage,
    PlayerSearchPage,
    PlayerPopover,
    TTRHistoryPage,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let representation = match self {
            Page::LoginPage => format!("{BASE_PAGE}/community/login"),
            Page::LoginRedirectPage => format!("{BASE_PAGE}/public/home?fromlogin=1&logout=true"),
            Page::PlayerSearchPage => format!("{BASE_PAGE}/clicktt/DTTB/player/search"),
            Page::PlayerPopover => format!("{BASE_PAGE}/clicktt/DTTB/player/popover"),
            Page::TTRHistoryPage => format!("{BASE_PAGE}/community/events"),
        };

        write!(f, "{}", representation)
    }
}

impl From<Page> for String {
    fn from(value: Page) -> Self {
        value.to_string()
    }
}
