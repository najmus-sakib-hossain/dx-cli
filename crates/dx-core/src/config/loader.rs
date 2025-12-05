use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use tracing::debug;

use crate::config::schema::DxConfig;
use crate::error::{DxError, DxResult};

pub fn load() -> DxResult<DxConfig> {
    let default_path = PathBuf::from("dx.toml");
    if default_path.exists() {
        load_from_path(&default_path)
    } else {
        Ok(DxConfig::default())
    }
}

pub fn load_from_path(path: impl AsRef<Path>) -> DxResult<DxConfig> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| DxError::FileSystem {
        path: path.to_path_buf(),
        source,
    })?;

    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("toml");
    debug!("loading config" = %path.display(), ext);

    let config = match ext {
        "json" => parse::<DxConfig, _, _>(&content, |s| serde_json::from_str::<DxConfig>(s))?,
        "yaml" | "yml" => parse::<DxConfig, _, _>(&content, |s| serde_yaml::from_str::<DxConfig>(s))?,
        _ => parse::<DxConfig, _, _>(&content, |s| toml::from_str::<DxConfig>(s))?,
    };

    Ok(config)
}

fn parse<T, F, E>(content: &str, parser: F) -> DxResult<T>
where
    T: DeserializeOwned,
    for<'de> F: FnOnce(&'de str) -> Result<T, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    parser(content).map_err(|err| DxError::Config {
        message: format!("Failed to parse configuration: {err}"),
        path: None,
    })
}
