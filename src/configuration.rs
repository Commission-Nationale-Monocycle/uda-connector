use derive_getters::Getters;
use url::Url;

#[derive(Debug, Getters)]
pub struct Configuration {
    instances_list_url: Url,
}

impl Configuration {
    pub fn new(instances_list_url: String) -> Self {
        let instances_list_url = Url::parse(&instances_list_url).expect("Invalid URL supplied");
        Self { instances_list_url }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_with_valid_url() {
        Configuration::new("https://reg.unicycling-software.com/tenants?locale=en".to_owned());
    }

    #[test]
    #[should_panic(expected = "Invalid URL supplied")]
    fn build_with_invalid_url() {
        Configuration::new("123456".to_owned());
    }
}
