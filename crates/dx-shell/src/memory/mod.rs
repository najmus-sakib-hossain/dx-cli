use sled::Db;
use anyhow::Result;

#[derive(Debug)]
pub struct ShellMemory {
    _db: Db,
}

impl ShellMemory {
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { _db: db })
    }
}
