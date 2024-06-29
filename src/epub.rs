use std::fs::OpenOptions;

use crate::model::{Chapter, Novel};
// use crate::syosetu::Syosetu;

// Giữ nguyên, kiểm tra lại phần Import và phân vạch ID
fn gen_content(
    epub_gen: &mut epub_builder::EpubBuilder<epub_builder::ZipLibrary>, 
    novel: Novel
) -> &mut epub_builder::EpubBuilder<epub_builder::ZipLibrary> {
    let mut chap_num = 0;
    for chapter in novel.chapter_list {
        chap_num += 1;
        epub_gen.add_content(
            epub_builder::EpubContent::new(
                format!("chapter_{}.xhtml", chap_num), 
                import(chapter).as_bytes()
            )
        .title(format!("chapter_{}", chap_num))
        .reftype(epub_builder::ReferenceType::Text)).expect("Nothing");
    }
    epub_gen
}

// Thay Source từ Syosetu sang
pub fn gen_epub(book: Novel) -> epub_builder::Result<()> {
    let coverpage = r##"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="ja" xml:lang="ja">
	<head>
		<title></title>
		<link rel="stylesheet" type="text/css" href="css/epub.css" />
	</head>
	<body>
		<img src="images/cover.png" alt="cover" title="cover" />
	</body>
</html>    
    "##;
    let titlepage = r##"<?xml version='1.0' encoding='utf-8'?>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en">
    <head>
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
        <meta name="calibre:cover" content="true"/>
        <title>Cover</title>
        <style type="text/css" title="override_css">
            @page {padding: 0pt; margin:0pt}
            body { text-align: center; padding:0pt; margin: 0pt; }
        </style>
    </head>
    <body>
        <div>
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="100%" height="100%" viewBox="0 0 1200 1600" preserveAspectRatio="none">
                <image width="1200" height="1600" xlink:href="cover_image.jpg"/>
            </svg>
        </div>
    </body>
</html>
"##;
    let dummy_image = "cover.jpg";
    let css_snippet = r##"
@charset "UTF-8";
@namespace "http://www.w3.org/1999/xhtml";
@namespace epub "http://www.idpf.org/2007/ops";
html {
    -ms-writing-mode: tb-rl;
    -epub-writing-mode: vertical-rl;
    -webkit-writing-mode: vertical-rl;
    writing-mode: vertical-rl;
    font-family: serif, sans-serif;
}   
    "##;

    // TODO
    // Change to the GEN type for supporting multiple sources
    // let novel = Syosetu::fetch_novel(book_id);
    
    // DO NOT CHANGE FROM HERE
    let output = OpenOptions::new().write(true).create(true).open(format!("{}.epub", &book.title))
    .expect("Something");
    // Create a new EpubBuilder using the zip library
    let mut binding = epub_builder::EpubBuilder::new(epub_builder::ZipLibrary::new()?)?;
    let gen_epub = binding
    // Set some metadata
        .metadata("author", &book.author)?
        .metadata("title", &book.title)?
    // Set epub version to 3.0
        .epub_version(epub_builder::EpubVersion::V30)
    // Set the stylesheet (create a "stylesheet.css" file in EPUB that is used by some generated files)
        .stylesheet(css_snippet.as_bytes())?
    // Add a image cover file
        .add_cover_image("cover.jpg", dummy_image.as_bytes(), "image/jpg")?
    // Add a cover page
        .add_content(epub_builder::EpubContent::new("cover.xhtml", coverpage.as_bytes())
                     .title("Cover")
                     .reftype(epub_builder::ReferenceType::Cover))?
        .add_content(epub_builder::EpubContent::new("title.xhtml", titlepage.as_bytes())
                     .title("Title")
                     .reftype(epub_builder::ReferenceType::TitlePage))?;
    // END    

    // Fix it here
    let generated_content = gen_content(gen_epub, book);
    // End
    
    // Generate a toc inside of the document, that will be part of the linear structure.
    generated_content.inline_toc().generate(output).expect("Error generating");
    Ok(())
}



// Trả ra XHTML dựa trên ID truyện.
// TODO
fn import(chap_content: Chapter) -> String {
    to_xhtml(chap_content.content, &chap_content.name)
}



// Giữ nguyên
fn to_xhtml(content: Vec<String>, chap_title: &str) -> String {
  // Start
  let start = r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="ja" lang="ja">
    <head>
        <meta charset="utf-8" />
        <link rel="stylesheet" href="stylesheet.css" type="text/css" />
        <title>Untitled</title>
    </head>
    <body>
"##;
  let chapter_title = format!("\n<h2>{}</h2>\n", chap_title);
  let mut file: String = start.to_string() + chapter_title.as_str();

  for line in content {
    file += &line;
  }
  file += "\n</body>\n</html>";
  // End
  file
}