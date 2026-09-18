use std::path::Path;

pub struct Document {
    pub file_name: String,
    pub content: String,
}

impl Document {
    pub fn new(file_name: String, content: String) -> Self {
        Document { file_name, content }
    }

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|str| str.to_string())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidFilename,
                    "Could not retrieve file name",
                )
            })?;
        Ok(Self::new(file_name, content))
    }
}
