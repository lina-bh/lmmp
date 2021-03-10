mod mp3;
mod opus;
mod vorb;

use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Track {
    pub path: PathBuf,
    pub album_artist: String,
    pub album: String,
    pub track_no: u32,
    pub title: String,
}

impl Track {
    pub fn parse(p: impl AsRef<Path>) -> Result<Option<Track>, Box<dyn Error>> {
        let p = p.as_ref();
        Ok(if let Some(ext) = p.extension().and_then(OsStr::to_str) {
            match ext {
                "mp3" => mp3::parse(p)?,
                "opus" => opus::parse(p)?,
                "ogg" => vorb::parse(p)?,
                _ => None,
            }
        } else {
            None
        })
    }
}

pub fn index<P: AsRef<Path>>(library_path: P) -> Result<Vec<Track>, Box<dyn Error>> {
    let paths = WalkDir::new(library_path)
        .into_iter()
        .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?
        .iter()
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_owned())
        .collect::<Vec<PathBuf>>();
    let mut tracks: Vec<Track> = Vec::new();

    for path in paths {
        if let Some(track) = Track::parse(&path)? {
            tracks.push(track);
        }
    }

    Ok(tracks)
}
