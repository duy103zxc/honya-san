use std::fs::{self, File};
use crate::{file, syosetu::{fetch_chapter, fetch_novel}};


pub fn gen_epub(book_id: &str, author: &str, title: &str) {
    // File content
        let container_file = r#"
<?xml version="1.0" encoding="utf-8"?>
<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml" />
  </rootfiles>
</container>    
    "#;
    // create necessary files and folder
    fs::create_dir_all(format!("./{}", book_id)).expect("Unable to create root folder for the book");
    fs::create_dir_all(format!("./{}/META-INF", book_id)).expect("Unable to create META-INF folder for the book");
    fs::create_dir_all(format!("./{}/OEBPS", book_id)).expect("Unable to create OEBPS folder for the book");

    fs::write(format!("./{}/mimetype", book_id), "application/epub+zip").expect("Can't generate mimetype file");   
    fs::write(format!("./{}/META-INF/container.xml", book_id), container_file).expect("Can't generate container file");   
    fs::write(format!("./{}/OEBPS/style.css", book_id), file::gen_css()).expect("Can't generate container file");   
    fs::write(format!("./{}/OEBPS/titlepage.xhtml", book_id), file::gen_title_page()).expect("Can't generate container file");   
    fs::write(format!("./{}/OEBPS/titlepage.css", book_id), file::gen_css()).expect("Can't generate container file");   
    fs::write(format!("./{}/OEBPS/nav.xhtml", book_id), file::gen_toc_xhtml()).expect("Can't generate container file");   
    fs::write(format!("./{}/OEBPS/toc.ncx", book_id), file::gen_toc(book_id, title)).expect("Can't generate container file");   
    File::create(format!("./{}/OEBPS/cover_image.jpg", book_id)).expect("Can't create the image file");

    let opf = import(book_id, format!("./{}/OEBPS", book_id).as_str());
    fs::write(format!("./{}/OEBPS/content.opf", book_id), file::gen_opf(book_id, title, author, opf.0, opf.1)).expect("Can't generate container file");   
}

fn import(book_id: &str, path: &str) -> (String, String) {
  let novel_chap_list = fetch_novel(book_id);
  let mut file_num = 0;
  let mut manifest_opf = String::from("");
  let mut spine_opf = String::from("");

  for chap in novel_chap_list.2 {
    file_num += 1;
    manifest_opf += format!("<item media-type=\"application/xhtml+xml\" href=\"{}.xhtml\" id=\"_{}.xhtml\" />", file_num, file_num).as_str();
    spine_opf += format!("<itemref idref=\"_{}.xhtml\" />", file_num).as_str();
    let url = format!("https://ncode.syosetu.com/{}", chap);
    fs::write(format!("{}/{}.xhtml", path, file_num), to_xhtml(fetch_chapter(&url).1, &fetch_chapter(&url).0)).expect("");
  }
  (manifest_opf, spine_opf)
}

fn to_xhtml(content: Vec<String>, chap_title: &str) -> String {
  // Start
  let start = r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="ja" lang="ja">
    <head>
        <meta charset="utf-8" />
        <link rel="stylesheet" href="style.css" type="text/css" />
        <title>Untitles</title>
    </head>
    <body class="bodymatter" epub:type="bodymatter">
"##;
  let chapter_title = format!("\n<h2>{}</h2>\n", chap_title);
  let mut file: String = start.to_string() + chapter_title.as_str();

  for line in content {
    file += &line;
    file += "\n"
  }

  file += "\n</body>\n</html>";
  // End
  file
}
