use reqwest;
use reqwest::header::USER_AGENT;

pub fn get_body_from_url(url: &str) -> String {
    let req_error_msg = format!("Failed to request resource :: {}", url);
    let response_parse_error = format!("Failed to parse the response of :: {}", url);

    let client = reqwest::blocking::Client::new();
    
    let fetcher = client
    .get(url)
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0")
    .send().expect(&req_error_msg).text().expect(&response_parse_error);

    fetcher
}

