use derive_getters::Getters;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, PartialEq, Clone, Default)]
pub struct UdaCredentials {
    /// Should be something like `https://cfm2019training.reg.unicycling-software.com`
    /// Beware of not including anything after the TLD. Otherwise, it may not work.
    #[getter(skip)]
    uda_url: String,
    login: String,
    password: String,
}

impl UdaCredentials {
    pub fn uda_url(&self) -> &String {
        &self.uda_url
    }
}

impl Debug for UdaCredentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Uda Credentials {{uda={}, login={}, password=MASKED}}",
            self.uda_url, self.login
        )
    }
}

#[cfg(any(test, feature = "test"))]
impl UdaCredentials {
    pub fn new(uda_url: String, login: String, password: String) -> Self {
        Self {
            uda_url,
            login,
            password,
        }
    }
}
