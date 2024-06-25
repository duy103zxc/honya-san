pub fn gen_opf(id: &str, title: &str, author: &str, manifest_xml: String, spine_ref: String) -> String {
    let content: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" unique-identifier="identifier0" version="3.0" prefix="ibooks: http://vocabulary.itunes.apple.com/rdf/ibooks/vocabulary-extensions-1.0/">
    <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
        <meta refines="#identifier0" property="identifier-type" scheme="xsd:string">uuid</meta>
    "##;

    let id = format!("<dc:identifier id=\"identifier0\">{}</dc:identifier>", id);

    let author_opf = format!("<dc:title id=\"title0\">{}</dc:title>\n<dc:creator id=\"creator0\">{}</dc:creator>", author, title );

    let toc = r##"<meta refines="#title0" property="display-seq">1</meta>
    <meta refines="#creator0" property="display-seq">1</meta>
        <dc:format>application/epub+zip</dc:format>
        <dc:language>ja</dc:language>
        <meta property="ibooks:specified-fonts">true</meta>
        <meta property="dcterms:modified">2001-01-01T00:00:00Z</meta>
    </metadata>
    <manifest>
        <item media-type="text/css" href="titlepage.css" id="_titlepage.css" />
        <item media-type="text/css" href="style.css" id="_style.css" />
        <item media-type="application/xhtml+xml" href="titlepage.xhtml" id="_titlepage.xhtml" />
        <item media-type="application/xhtml+xml" href="nav.xhtml" id="_nav.xhtml" properties="nav" />
        <item media-type="application/x-dtbncx+xml" href="toc.ncx" id="_toc.ncx" />"##;
    

    let manifest_part = r##"
    </manifest>
    <spine page-progression-direction="rtl" toc="_toc.ncx">
        <itemref idref="_titlepage.xhtml" />
        <itemref idref="_nav.xhtml" />    
    "##;
    let spine=r##"
    </spine>
    <guide>
        <reference type="titlepage" title="cover" href="titlepage.xhtml" />
    </guide>
</package>
"##;

    let xml_file = content.to_owned() + id.as_str() + author_opf.as_str() + toc + manifest_xml.as_str() + manifest_part + spine_ref.as_str() + spine;
    xml_file
} 

pub fn gen_css() -> String {
    let css_file = r#"
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

body {
text-align: justify;
text-justify: inter-ideograph;
vertical-align: baseline;
word-wrap: break-word;
}

h1, h2, h3, h4, h5, h6 {
font-family: serif, sans-serif;
font-weight: normal;
color: inherit;
}
"#;

css_file.to_string()
}

pub fn gen_title_page() -> String{
    let page = r##"<?xml version='1.0' encoding='utf-8'?>
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

    page.to_string()
}

pub fn gen_toc(id: &str, title: &str) -> String {
    let content=r##"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns:ncx="http://www.daisy.org/z3986/2005/ncx/"
    xmlns="http://www.daisy.org/z3986/2005/ncx/"
    version="2005-1"
    xml:lang="ja">
    <head>
    "##;
    let added_gen_id = r##"
        <meta name="dtb:depth" content="1"/>
        <meta name="dtb:generator" content="syosetu2ebook"/>
        <meta name="dtb:totalPageCount" content="0"/>
        <meta name="dtb:maxPageNumber" content="0"/>
    </head>

    <docTitle>
"##;
    let gen_id = format!("<meta name=\"dtb:uid\" content=\"{}\"/> \n", id);

    let sakuhin = format!("<text>{}</text>", title);

    let last_part = r##"
    </docTitle>
    <navMap>
    </navMap>
</ncx>
"##;

    let r#final: String = content.to_owned() + gen_id.as_str() + added_gen_id + sakuhin.as_str() + last_part;
    r#final
}

pub fn gen_toc_xhtml() -> String {
    let content=r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="ja" lang="ja">
    <head>
        <meta charset="utf-8" />
        <link rel="stylesheet" href="style.css" type="text/css" />
        <title>Untitled</title>
        <style type="text/css">
            nav ol {
                list-style-type: none;
            }
        </style>
    </head>
    <body class="frontmatter" epub:type="frontmatter">
        <nav id="toc" class="toc" epub:type="toc">
            <h2>目次</h2>
            <ol>
            </ol>
        </nav>
        <nav id="landmarks" class="landmarks" epub:type="landmarks" hidden="hidden">
            <ol>
                <li><a epub:type="titlepage" href="titlepage.xhtml"><span>扉</span></a></li>
                <li><a epub:type="toc" href="nav.xhtml"><span>目次</span></a></li>
            </ol>
        </nav>
    </body>
</html>    
    "##;
    content.to_string()
}