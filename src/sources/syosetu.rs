// use std::borrow::Borrow;
use std::fs::OpenOptions;
// use std::str::FromStr;

use scraper::{Html, Selector};
use std::io::Write;
use crate::interface::Source;
use crate::model::*;
// use crate::utils::file;
use crate::utils::http;
// use crate::utils::text;


#[derive(Debug)]
pub struct Syosetu {}

impl Source for Syosetu {
    fn metadata(&self) -> DataSource {
        DataSource {
            name: String::from("Syosetu"),
            base_url: "https://ncode.syosetu.com/".to_string(),
        }
    }

    fn fetching(&self, novel_id: &str) -> Novel {
        // element selectors
        let base_url = self.metadata().base_url + novel_id;
        let body = http::get_body_from_url(&base_url);
        let document = Html::parse_document(&body);
        let title_selector = Selector::parse("p.novel_title").unwrap();
        let author = Selector::parse("div.novel_writername").unwrap();
        // List of chapters
        let list_selector = Selector::parse("div.index_box").unwrap();
        let id_selector = Selector::parse("a").unwrap();
                
        let name = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");
        let author = document.select(&author).next().unwrap().text().collect::<Vec<_>>().join("");
        let url = base_url;
        let ul = document.select(&list_selector).next().unwrap();
        let each_chap_url = ul.select(&id_selector).into_iter().map(|element| element.value().attr("href").unwrap()).collect::<Vec<_>>();
        
        let mut current_index: u32 = 0;
        for chap in each_chap_url {
            let current_url = self.metadata().base_url + chap;
            current_index += 1;
            fetch_chapter(current_url.as_str(), current_index);
        }
        
        Novel {
            name: name,
            author: author,
            id: 0,
            url: url,
            chapters: vec![String::from("temp")]
        }
    }
    
    
}

fn fetch_chapter(chap_link: &str, current_index: u32) {
    // Fetch from the page
    let body = http::get_body_from_url(&chap_link);
    let document = Html::parse_document(&body);
    // Selector
    let title_selector = Selector::parse("p.novel_subtitle").unwrap();
    let novel_selector = Selector::parse("div.novel_view").unwrap();
    let p_selector = Selector::parse("p").unwrap();
        
    let chap_name = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");    
    let ul = document.select(&novel_selector).next().unwrap();
    
    let text = ul.select(&p_selector)
    .map(|txt| txt.html())
    .collect::<Vec<_>>();
        
    // Create file
    let mut chapter_file = OpenOptions::new().write(true).append(true).create(true).open(format!("{}. {}.html", current_index, chap_name)).expect("Can't create file");
    
    for line in text {
        writeln!(chapter_file, "{}", line).expect("Can't write to the chapter file");
    }
    
}

