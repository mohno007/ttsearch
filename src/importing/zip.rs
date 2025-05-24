use std::fs::File;
use std::path::Path;

use thiserror::Error;
use zip;
use zip::read::ZipFile;

#[derive(Error, Debug)]
pub enum ZipError {
    #[error("Failed to open the file or directory: {0}")]
    IOError(String),
    #[error("Error has occurred while processing zip file: {0}")]
    ProcessingError(String),
}

pub fn read_zipfile<F, E>(zip_path: &Path, f: F) -> Result<(), ZipError>
where
    F: Fn(ZipFile<'_, File>) -> Result<(), E>,
    E: std::fmt::Display,
{
    let file = File::open(zip_path)
        .map_err(|e| ZipError::IOError(format!("Failed to open a zip file: {:?}: {}", zip_path, e)))?;

    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| ZipError::ProcessingError(format!("Failed to process a zip file: {}", e)))?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)
            .map_err(|e| ZipError::IOError(format!("Could not find a {}th file in zip: {}", i, e)))?;

        f(file).map_err(|e| ZipError::IOError(format!("Failed to process a zip file: {}", e)))?;
    }

    Ok(())
}
