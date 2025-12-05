use sled::Db;
use anyhow::Result;

#[derive(Debug)]
pub struct ShellMemory {
    db: Db,
}

impl ShellMemory {
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
}
