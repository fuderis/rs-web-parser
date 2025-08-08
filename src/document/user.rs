use crate::prelude::*;
use rand::Rng;

static RAND_USERS: Lazy<(Vec<User>, usize)> = Lazy::new(|| {
    let users = vec![
        User::ChromeWindows,
        User::ChromeMac,
        User::ChromeLinux,

        User::FirefoxWindows,
        User::FirefoxMac,
        User::FirefoxLinux,

        User::EdgeWindows,
        User::EdgeMac,

        User::OperaWindows,
        User::OperaMac,
        User::OperaLinux,

        User::YandexWindows,
        User::YandexLinux,
    ];

    let len = users.len();
    
    (users, len)
});

/// The HTTP request User-Agent
#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum User {
    #[display = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"]
    ChromeWindows,

    #[display = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"]
    ChromeMac,

    #[display = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"]
    ChromeLinux,

    #[display = "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36"]
    ChromeAndroid,

    #[display = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:114.0) Gecko/20100101 Firefox/114.0"]
    FirefoxWindows,

    #[display = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:114.0) Gecko/20100101 Firefox/114.0"]
    FirefoxMac,

    #[display = "Mozilla/5.0 (X11; Linux x86_64; rv:114.0) Gecko/20100101 Firefox/114.0"]
    FirefoxLinux,

    #[display = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.0.0"]
    EdgeWindows,

    #[display = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.0.0"]
    EdgeMac,

    #[display = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 \
(KHTML, like Gecko) Version/16.0 Safari/605.1.15"]
    SafariMac,

    #[display = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 \
(KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1"]
    SafariIOS,

    #[display = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OPR/120.0.0.0"]
    OperaWindows,

    #[display = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OPR/120.0.0.0"]
    OperaMac,

    #[display = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OPR/120.0.0.0"]
    OperaLinux,

    #[display = "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36 OPR/120.0.0.0"]
    OperaAndroid,

    #[display = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/114.0.0.0 YaBrowser/24.0.0.0 Safari/537.36"]
    YandexWindows,

    #[display = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/114.0.0.0 YaBrowser/24.0.0.0 Safari/537.36"]
    YandexLinux,

    #[display = "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/114.0.0.0 YaBrowser/24.0.0.0 Mobile Safari/537.36"]
    YandexAndroid,

    #[display = "{0}"]
    Custom(String)
}

impl User {
    /// Get random PC User-Agent
    pub fn random() -> Self {
        let index = rand::rng().random_range(0..RAND_USERS.1);
        RAND_USERS.0[index].clone()
    }
}
