use reqwest;
use reqwest::header::{REFERER, USER_AGENT};

pub fn get_body_from_url(url: &str) -> String {
    let req_error_msg = format!("Failed to request resource :: {}", url);
    let response_parse_error = format!("Failed to parse the response of :: {}", url);

    let client = reqwest::blocking::Client::new();
    
    let fetcher = client
    .get(url)
    .header(USER_AGENT, "Mozilla/5.0")
    .header(REFERER, url)
    .send().expect(&req_error_msg).text().expect(&response_parse_error);

    fetcher
}
