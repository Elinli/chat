use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use sha1::{Digest, Sha1};
use tracing::info;

use crate::AppError;

use super::ChatFile;

impl ChatFile {
    pub fn new(ws_id: u64, filename: &str, data: &[u8]) -> Self {
        let hash = Sha1::digest(data);
        Self {
            ext: filename.split('.').last().unwrap_or("txt").to_string(),
            hash: hex::encode(hash),
            ws_id: ws_id,
        }
    }

    pub fn url(&self) -> String {
        format!("/files/{}", self.hash_to_path())
    }

    pub fn path(&self, base_url: &Path) -> PathBuf {
        base_url.join(self.hash_to_path())
    }
    pub fn hash_to_path(&self) -> String {
        let (first_part, second_part) = self.hash.split_at(6);
        let (second_part, third_part) = second_part.split_at(6);

        format!(
            "{}/{}/{}/{}.{}",
            self.ws_id, first_part, second_part, third_part, self.ext
        )
    }
}

impl FromStr for ChatFile {
    type Err = AppError;

    // convert /files/s/339/807/e635afbeab088ce33206fdf4223a6bb156.png to ChatFile
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("/files/") else {
            return Err(AppError::ChatFileError(format!(
                "Invalid chat file path: {}",
                s
            )));
        };

        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 4 {
            return Err(AppError::ChatFileError(format!(
                "File path {} does not valid {:?}",
                s, parts
            )));
        }

        let Ok(ws_id) = parts[0].parse::<u64>() else {
            return Err(AppError::ChatFileError(format!(
                "Invalid workspace id: {}",
                parts[1]
            )));
        };

        let Some((part3, ext)) = parts[3].split_once('.') else {
            return Err(AppError::ChatFileError(format!(
                "Invalid file name: {}",
                parts[3]
            )));
        };

        let hash = format!("{}{}{}", parts[1], parts[2], part3);
        Ok(Self {
            ws_id,
            ext: ext.to_string(),
            hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_file_new() {
        let file = ChatFile::new(1, "test.txt", b"hello");
        assert_eq!(file.ext, "txt");
        assert_eq!(file.hash, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }
}
