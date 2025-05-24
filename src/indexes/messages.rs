use std::fs;

use tantivy::schema::document::{DeserializeError, DocumentDeserialize};
use tantivy::schema::{IndexRecordOption, OwnedValue, Schema, TextFieldIndexing, TextOptions, FAST, INDEXED, STORED, STRING};
use tantivy::{DateTime, Index, TantivyDocument};

use crate::constants;
use crate::lindera_tokenizer;
use crate::models::message::Message;

pub fn create_messages_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_u64_field("topic_id", INDEXED | STORED);
    schema_builder.add_u64_field("id", INDEXED | STORED);
    schema_builder.add_text_field("account_name", STRING | STORED);
    schema_builder.add_text_field("account_fullname", STRING | STORED);
    schema_builder.add_text_field(
        "message", 
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("lang_ja")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );
    schema_builder.add_date_field("created_at", INDEXED | STORED | FAST);

    schema_builder.build()
}

pub fn create_message_document(messages_schema: &Schema, message: &Message) -> TantivyDocument {
    let mut document = TantivyDocument::default();

    let field_topic_id = messages_schema.get_field("topic_id").unwrap();
    let field_id = messages_schema.get_field("id").unwrap();
    let field_account_name = messages_schema.get_field("account_name").unwrap();
    let field_account_fullname = messages_schema.get_field("account_fullname").unwrap();
    let field_message = messages_schema.get_field("message").unwrap();
    let field_created_at = messages_schema.get_field("created_at").unwrap();

    document.add_u64(field_topic_id, message.topic_id());
    document.add_u64(field_id, message.id());
    document.add_text(field_account_name, message.account_name());
    document.add_text(field_account_fullname, message.account_fullname());
    document.add_text(field_message, message.message());
    document.add_date(field_created_at, DateTime::from_timestamp_secs(message.created_at().timestamp()));

    document
}

pub fn open_or_create() -> tantivy::Result<Index> {
    let mut index_path = constants::get_data_path()
        .ok_or(tantivy::TantivyError::InternalError("failed to open".to_owned()))?;
    index_path.push("messages");

    let index = if fs::exists(&index_path)? {
        Index::open_in_dir(&index_path)?
    } else {
        fs::create_dir_all(&index_path)?;
        let schema = create_messages_schema();
        Index::create_in_dir(&index_path, schema.clone())?
    };

    let tokenizer = lindera_tokenizer::create_lindera_tokenizer();
    index.tokenizers().register("lang_ja", tokenizer);

    Ok(index)
}

impl DocumentDeserialize for Message {
    fn deserialize<'de, D>(mut deserializer: D) -> Result<Self, tantivy::schema::document::DeserializeError>
    where D: tantivy::schema::document::DocumentDeserializer<'de>
    {
        let mut topic_id: Option<u64> = None;
        let mut id: Option<u64> = None;
        let mut account_name: Option<String> = None;
        let mut account_fullname: Option<String> = None;
        let mut message: Option<String> = None;
        let mut created_at: Option<tantivy::DateTime> = None;

        loop {
            if let Some((field, owned_value)) = deserializer.next_field::<OwnedValue>()? {
                match (field.field_id(), owned_value) {
                    (0, OwnedValue::U64(v)) => { topic_id.replace(v); },
                    (1, OwnedValue::U64(v)) => { id.replace(v); },
                    (2, OwnedValue::Str(v)) => { account_name.replace(v); },
                    (3, OwnedValue::Str(v)) => { account_fullname.replace(v); },
                    (4, OwnedValue::Str(v)) => { message.replace(v); },
                    (5, OwnedValue::Date(v)) => { created_at.replace(v); },
                    _ => {
                        Err(DeserializeError::Custom(format!("Unknown field {}", field.field_id())))?
                    }
                };
            } else {
                break
            }
        }

        if let (
            Some(v_topic_id),
            Some(v_id),
            Some(v_account_name),
            Some(v_account_fullname),
            Some(v_message),
            Some(v_created_at),
        ) = (
            topic_id,
            id,
            account_name,
            account_fullname,
            message,
            created_at
        ) {
            Ok(Message::new(
                v_topic_id,
                v_id,
                &v_account_name,
                &v_account_fullname,
                &v_message,
                chrono::DateTime::from_timestamp_nanos(v_created_at.into_timestamp_nanos()),
            ))
        } else {
            Err(tantivy::schema::document::DeserializeError::Custom("bbb".to_owned()))
        }
    }
}
