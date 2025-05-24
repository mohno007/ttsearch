use std::path::PathBuf;

use homedir::my_home;

pub fn get_data_path() -> Option<PathBuf> {
    let mut path = my_home().unwrap_or(None)?;
    path.push(".local/state/ttsearch/");
    Some(path)
}
