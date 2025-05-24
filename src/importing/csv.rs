use std::io::Read;

use thiserror::Error;

use crate::models::message::Message;

#[derive(Error, Debug)]
pub enum CsvError {
    #[error("Field {0} is missing in a message record")]
    MissingField(&'static str),
    #[error("{0}")]
    InvalidFormat(String),
    #[error("{0}")]
    ProcessingError(String),
}

pub fn parse_message_record(record: &csv::StringRecord) -> Result<Message, CsvError> {
    if record.len() != 6 {
        return Err(CsvError::InvalidFormat(format!("message record should have 6 fields but {}", record.len())));
    }

    let topic_id = record.get(0)
        .ok_or(CsvError::MissingField("topic_id"))?
        .parse::<u64>().or(Err(CsvError::InvalidFormat("topic_id should be number".to_owned())))?;

    let id = record.get(1)
        .ok_or(CsvError::InvalidFormat("id".to_owned()))?
        .parse::<u64>().or(Err(CsvError::InvalidFormat("id should be number".to_owned())))?;

    let account_name = record.get(2)
        .ok_or(CsvError::MissingField("account_name"))?
        .to_owned();

    let account_fullname = record.get(3)
        .ok_or(CsvError::MissingField("account_name"))?
        .to_owned();

    let message = record.get(4)
        .ok_or(CsvError::MissingField("message"))?
        .to_owned();

    let created_at_str = record.get(5)
        .ok_or(CsvError::MissingField("created_at"))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(created_at_str)
        .or(Err(CsvError::InvalidFormat("created_at is not valid date".to_owned())))?
        .to_utc();

    Ok(Message::new(
        topic_id,
        id,
        &account_name,
        &account_fullname,
        &message,
        created_at
    ))
}

pub fn read_csv_messages<R, F, E>(rdr: R, f: F) -> Result<(), CsvError>
where
    R: Read,
    E: std::error::Error,
    F: Fn(Message) -> Result<(), E>,
{
    let mut reader = csv::Reader::from_reader(rdr);
    let mut records = reader.records();

    // Skip header
    records.next();

    for result in records {
        let record = result.map_err(|e| CsvError::ProcessingError(e.to_string()))?;
        let message = parse_message_record(&record)?;
        f(message).map_err(|e| CsvError::ProcessingError(e.to_string()))?;
    }

    Ok(())
}
