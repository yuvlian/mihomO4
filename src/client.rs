use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct FetchError {
    url: String,
    status: StatusCode,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to fetch URL: {}. Status: {}",
            self.url, self.status
        )
    }
}

impl FetchError {
    fn new(url: &str, status: StatusCode) -> Self {
        FetchError {
            url: String::from(url),
            status: status,
        }
    }
}

impl Error for FetchError {}

pub async fn fetch_json(url: &str, client: &reqwest::Client) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let status = response.status();

    match status.is_success() {
        true => response.text().await.map_err(Into::into),
        _ => Err(Box::new(FetchError::new(url, status))),
    }
}
