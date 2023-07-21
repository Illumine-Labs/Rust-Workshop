use super::*;
use std::io::Result as IoResult;

impl Store {
    pub fn dump(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }

    pub fn dump_to_file(&self, path: &str) -> IoResult<()> {
        let mut file = std::fs::File::create(path)?;

        serde_json::to_writer(&mut file, &self.inner)?;

        Ok(())
    }
}
