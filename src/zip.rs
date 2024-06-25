// use zip::write::FileOptions;

// fn zip(_name: &str, _list: Vec<&str>) -> zip::result::ZipResult<()>
// {
//     let path = std::path::Path::new(_name);
//     let file = std::fs::File::create(&path).unwrap();
//     let mut zip = zip::ZipWriter::new(file);
//     for i in _list.iter() {
//         zip.start_file(i as &str, FileOptions::default())?;
//     }
//     zip.finish()?;
//     Ok(())
// }