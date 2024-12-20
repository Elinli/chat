use std::path::{Path, PathBuf};

use sha1::{Digest, Sha1};

use super::ChatFile;

impl ChatFile {
    pub fn new(filename: &str, data: &[u8]) -> Self {
        let hash = Sha1::digest(data);
        Self {
            ext: filename.split('.').last().unwrap_or("txt").to_string(),
            hash: hex::encode(hash),
        }
    }

    pub fn url(&self, ws_id: u64) -> String {
        format!("/files/{}/{}", ws_id, self.hash_to_path())
    }

    pub fn path(&self, base_url: &Path) -> PathBuf {
        base_url.join(self.hash_to_path())
    }
    pub fn hash_to_path(&self) -> String {
        let (first_part, second_part) = self.hash.split_at(6);
        let (second_part, third_part) = second_part.split_at(6);

        format!("{}/{}/{}.{}", first_part, second_part, third_part, self.ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_file_new() {
        let file = ChatFile::new("test.txt", b"hello");
        assert_eq!(file.ext, "txt");
        assert_eq!(file.hash, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }
}
