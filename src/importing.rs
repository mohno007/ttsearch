use std::fs::File;
use std::fs;
use std::path::PathBuf;

use csv::CsvError;
use thiserror::Error;

use crate::models::message::Message;

pub mod csv;
pub mod zip;

#[derive(Error, Debug)]
pub enum ImportingError {
    #[error("File not found: {0}")]
    FileNotFound(String),
}

pub fn read_files<F, E>(path: &PathBuf, f: &F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(Message) -> Result<(), E>,
    E: std::error::Error,
{
    if !path.exists() {
        return Err(Box::new(ImportingError::FileNotFound(path.to_str().unwrap_or_default().to_owned())))
    }

    if path.is_file() {
        if path.to_string_lossy() == "messages.csv" {
            log::info!("Importing {}", path.to_string_lossy());
            let file = File::open(path)?;
            csv::read_csv_messages(file, f)?
        } else if path.extension().unwrap_or_default().to_ascii_lowercase() == "zip" {
            zip::read_zipfile(&path, |file| {
                let file_path = PathBuf::from(file.name());
                if file_path.file_name().unwrap_or_default() == "messages.csv" {
                    log::info!("Importing {} in {}", file.name(), path.to_string_lossy());
                    csv::read_csv_messages(file, f)?;
                }
                Ok::<(), CsvError>(())
            })?;
        }
    } else if path.is_dir() {
        let read_dir = fs::read_dir(path)?;

        for e in read_dir {
            let entry = e?;
            read_files(&entry.path(), f)?
        }
    }

    Ok(())
}
