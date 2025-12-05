use uuid::Uuid;

#[derive(Debug)]
pub struct Repository {
    pub id: Uuid,
    pub path: std::path::PathBuf,
}
