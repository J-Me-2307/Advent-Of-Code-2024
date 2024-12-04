use std::{fs::File, path::Path};

pub fn get_file(path: &str) -> Result<File, std::io::Error> {
    let path = Path::new(path);

    let file = File::open(path)?;

    Ok(file)
}
