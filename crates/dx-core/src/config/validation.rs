use crate::config::schema::DxConfig;
use crate::error::{DxError, DxResult};

pub fn validate(config: &DxConfig) -> DxResult<()> {
    if config.ai.temperature < 0.0 || config.ai.temperature > 1.0 {
        return Err(DxError::Validation(
            "AI temperature must be between 0.0 and 1.0".to_string(),
        ));
    }
    Ok(())
}
