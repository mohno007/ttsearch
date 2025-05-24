mod constants;
mod indexes;
mod models;
mod importing;
mod lindera_tokenizer;
mod printer;

use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use env_logger;
use tantivy::Order;

use crate::models::message::Message;
use crate::indexes::messages::create_message_document;
use crate::printer::Printer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, clap::ValueEnum)]
enum OutputMode {
    Pretty,
    Oneline,
    JSON,
}

impl OutputMode {
    fn to_printer(&self) -> Printer {
        match *self {
            Self::Pretty => Printer::Pretty,
            Self::Oneline => Printer::Oneline,
            Self::JSON => Printer::JSON,
        }
    }
}

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
    #[command(flatten)]
    color: colorchoice_clap::Color,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[clap(alias = "i")]
    Import {
        #[arg(value_name = "PATH", help = "file or directory to import")]
        path: String,
    },
    #[clap(alias = "s")]
    Search {
        #[arg(value_name = "QUERY", help = "query")]
        query: String,
        #[arg(short, long, default_value_t = OutputMode::Pretty, value_enum)]
        output: OutputMode,
    },
}

fn import_messages_csv(path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let index = indexes::messages::open_or_create()?;
    let mut index_writer = index.writer(50_000_000)?;

    // let index_reader = index.reader()?;
    // let searcher = index_reader.searcher();
    // let field_id = index.schema().get_field("id")?;
    // let collector = TopDocs::with_limit(1);

    let path = PathBuf::from(&path_str);
    importing::read_files(&path, &|message| {
        // let term = Term::from_field_u64(field_id.clone(), message.id());
        // let term_query = TermQuery::new(term, IndexRecordOption::Basic);
        // let result = searcher.search(&term_query, &collector)?;
        // if result.len() == 0 {
            let doc = create_message_document(&index.schema(), &message);
            let _ = index_writer.add_document(doc)?;
        //}
        Ok::<(), tantivy::error::TantivyError>(())
    })?;

    index_writer.commit()?;
    Ok(())
}

fn search(s: &str, printer: Printer) -> Result<(), Box<dyn Error>> {
    let index = indexes::messages::open_or_create()?;
    let index_reader = index.reader()?;
    let schema = index.schema();

    let searcher = index_reader.searcher();
    let collector = TopDocs::with_limit(50).order_by_u64_field("created_at", Order::Desc);

    let default_fields = vec![schema.get_field("message")?];
    let parser = QueryParser::new(schema.clone(), default_fields, index.tokenizers().clone());
    let query = parser.parse_query(s)?;
    let result = searcher.search(&query, &collector)?;

    for (_i, doc_addr) in result {
        let message = searcher.doc::<Message>(doc_addr)?;
        printer.print_message(&message);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Import { path } => {
            import_messages_csv(&path)
        }
        SubCommands::Search { query, output } => {
            search(&query, output.to_printer())
        }
    }
}
