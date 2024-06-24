use std::borrow::Borrow;
use std::str::FromStr;

use scraper::{Html, Selector};

use crate::interface::Source;
use crate::model::*;
use crate::utils::file;
use crate::utils::http;
use crate::utils::text;


#[derive(Debug)]
pub struct Syosetu {}

impl Source for Syosetu {
    fn metadata(&self) -> DataSource {
        DataSource {
            name: "Syosetu".to_string(),
            base_url: "https://ncode.syosetu.com/".to_string(),
        }
    }

    fn fetching(&self, novel_id: &str) -> Novel {
        let base_url = self.metadata().base_url + novel_id;
        let body = http::get_body_from_url(&base_url);
        let document = Html::parse_document(&body);
        let title_selector = Selector::parse("p.novel_title").unwrap();
        let author = Selector::parse("div.novel_writername").unwrap();
        // let list_selector = Selector::parse("div.index_box").unwrap();
        // let id_selector = Selector::parse("div.novel_sublist2").unwrap();
        // let link_selector = Selector::parse("a").unwrap();
        
        let name = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");
        let author = document.select(&author).next().unwrap().text().collect::<Vec<_>>().join(" ");
        let url = String::from(novel_id);
        let chapters = vec![Chapter {name: String::from("1h"), chapter_id: 2}];

        Novel {
            name: name,
            author: author,
            id: 0,
            url: url,
            chapters: chapters
        }
    }
}
