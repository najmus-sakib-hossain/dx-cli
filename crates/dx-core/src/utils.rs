use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

pub fn safe_join(base: &Path, user_path: &str) -> DxResult<PathBuf> {
    let full_path = base.join(user_path);
    let canonical = full_path
        .canonicalize()
        .map_err(|source| DxError::FileSystem {
            path: full_path.clone(),
            source,
        })?;

    if !canonical.starts_with(base) {
        return Err(DxError::Validation("Path traversal attempt detected".into()));
    }

    Ok(canonical)
}
