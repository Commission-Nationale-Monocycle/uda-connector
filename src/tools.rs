#[cfg(test)]
pub mod tests {
    use crate::error::UdaError;
    use crate::error::UdaError::ConnectionFailed;
    use reqwest::Client;

    pub fn build_client() -> Result<Client, UdaError> {
        reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .map_err(|_| ConnectionFailed)
    }
}
