mod avf;
mod mp3;
mod mp4;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct Track {
    pub path: PathBuf,
    pub artist: String,
    pub album: String,
    pub track_no: u32,
    pub title: String,
}

impl Track {
    pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            match ext {
                "mp3" => mp3::parse(&path),
                "m4a" => mp4::parse(&path),
                "ogg" | "flac" | "opus" => avf::parse(&path),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
