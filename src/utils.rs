use reqwest;
use reqwest::header::USER_AGENT;
use regex::Regex;
use std::fs;

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


pub fn write_txt_to_file(filename: &str, text: String) {
    let write_err = format!("Failed writing to file : {}", filename);
    fs::write(filename, text).expect(&write_err)
}

pub fn read_txt_from_file(filename: &str) -> String {
    let read_err = format!("Failed reading from file : {}", filename);
    fs::read_to_string(filename).expect(&read_err)
}


pub fn clean(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn parse_int_from_str(s: &str) -> &str {
    let re = Regex::new(r"\d+").unwrap();
    re.find(s).unwrap().as_str()
}
