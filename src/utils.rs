use reqwest;
use reqwest::header::USER_AGENT;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

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


pub fn zip_them_all(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let file = File::create(dst_file).unwrap();
    let mut zip_writer = zip::ZipWriter::new(file);

    // for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
    //     zip_writer.start_file(entry.path().display().to_string().trim(), options).expect("Nothing");
    //     zip_writer.write()?;
    // }
    
    let prefix = Path::new(src_dir);
    let mut buffer = Vec::new();
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_string = name
            .to_str()
            .map(str::to_owned).unwrap();
        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // println!("adding file {path:?} as {name:?} ...");
            zip_writer.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;
            buffer.clear();
            
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            // println!("adding dir {path_as_string:?} as {name:?} ...");
            zip_writer.add_directory(path_as_string, options)?;
        }
    }
    zip_writer.finish()?;
    Ok(())
}