use lindera::dictionary::DictionaryKind;
use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
use lindera_tantivy::tokenizer::LinderaTokenizer;

pub fn create_lindera_tokenizer() -> LinderaTokenizer {
    let mode = Mode::Normal;
    let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC).unwrap();
    let user_dictionary = None;
    let segmenter = Segmenter::new(mode, dictionary, user_dictionary);
    LinderaTokenizer::from_segmenter(segmenter)
}
