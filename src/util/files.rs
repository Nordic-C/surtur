use std::{fs, io, path::Path};

pub struct FileHandler<'fh> {
    pub file_extension: Option<String>,
    pub file_name: Option<String>,
    pub file_path: &'fh Path,
    pub file_content: String,
}

impl<'fh> FileHandler<'fh> {
    pub fn new(file_path: &'fh Path) -> io::Result<Self> {
        let extension = file_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());
        let name = file_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string());
        let content = fs::read_to_string(file_path)?;
        Ok(Self {
            file_extension: extension,
            file_name: name,
            file_content: content,
            file_path,
        })
    }
}
