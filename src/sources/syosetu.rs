use scraper::{Html, Selector};
use crate::{interface::Source, model::{Chapter, DataSource, Novel}, utils};


pub struct Syosetu {}

impl Source for Syosetu {

    fn metadata(&self) -> DataSource {
        DataSource {
            name: String::from("Syosetu"),
            base_url: String::from("https://ncode.syosetu.com/")
        }
    }
    fn fetch_novel(&self, novel_id: &str) -> Novel {
        // element selectors
        let base_url = self.metadata().base_url;
        let novel_url = base_url + novel_id;
        let body = utils::get_body_from_url(&novel_url);
        let document = Html::parse_document(&body);
        // Metadata
        let title_selector = Selector::parse("p.novel_title").unwrap();
        let author = Selector::parse("div.novel_writername").unwrap();
        // List of chapters
        let list_selector = Selector::parse("div.index_box").unwrap();
        let id_selector = Selector::parse("a").unwrap();
                
        let title = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");
        let author = document.select(&author).next().unwrap().text().collect::<Vec<_>>().join("");
        let ul = document.select(&list_selector).next().unwrap();
        let each_chap_url = ul.select(&id_selector).into_iter()
        .map(|element| self.fetch_chapter(element.value().attr("href").unwrap())).collect::<Vec<_>>();
        
        Novel {
            title,
            author,
            url: novel_url,
            chapter_list: each_chap_url
        }
    }

    fn fetch_chapter(&self, chap_link: &str) -> Chapter {
        // Fetch from the page
        let base_link = self.metadata().base_url + chap_link;
        let body = utils::get_body_from_url(&base_link);
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
         
        // (chap_name, text) 
        Chapter {
            chapter_id: String::from(chap_link),
            name: chap_name,
            content: text,
        }
    }
}

