use std::path::{Path, PathBuf};

use base64::{engine::general_purpose, Engine as _};
use rand::{distributions::Alphanumeric, Rng};

use crate::error::StabilityAIError;

pub(crate) async fn save_b64<P: AsRef<Path>>(
    b64: &str,
    dir: P,
) -> Result<PathBuf, StabilityAIError> {
    let filename: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let filename = format!("{filename}.png");

    let path = PathBuf::from(dir.as_ref()).join(filename);

    tokio::fs::write(
        path.as_path(),
        general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| StabilityAIError::FileSaveError(e.to_string()))?,
    )
    .await
    .map_err(|e| StabilityAIError::FileSaveError(format!("{e}, path: {}", path.display())))?;

    Ok(path)
}
