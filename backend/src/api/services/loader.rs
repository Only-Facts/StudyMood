use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicInfo {
    file: String,
    pub path: String,
    pub mime: String,
}

#[derive(Debug)]
pub struct AppState {
    pub music_dir: PathBuf,
    pub tracks: Mutex<HashMap<String, MusicInfo>>,
}

impl AppState {
    pub fn new(music_dir: PathBuf, tracks: Mutex<HashMap<String, MusicInfo>>) -> Self {
        AppState { music_dir, tracks }
    }
}

pub async fn load_music_files(music_dir: &Path) -> Result<HashMap<String, MusicInfo>, String> {
    let mut tracks = HashMap::new();
    println!("Scanning music directory: {music_dir:?}");

    if !music_dir.exists() {
        return Err(format!("Music directory does not exist: {music_dir:?}"));
    }
    if !music_dir.is_dir() {
        return Err(format!("Music directory is not a directory: {music_dir:?}"));
    }

    for entry in WalkDir::new(music_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let mime_type = mime_guess::from_path(path)
                .first_or_text_plain()
                .to_string();

            if mime_type.starts_with("audio/")
                && let Some(filename) = path.file_name().and_then(|s| s.to_str())
            {
                let relative_path = path
                    .strip_prefix(music_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .into_owned();
                let mut file = filename.to_string();
                file.truncate(file.len() - 4);
                let track_info = MusicInfo {
                    file,
                    path: relative_path.clone(),
                    mime: mime_type,
                };
                tracks.insert(relative_path, track_info);
                println!("Found music file: {filename}");
            }
        }
    }
    println!("Finished scanning. Found {} music files.", tracks.len());
    Ok(tracks)
}
