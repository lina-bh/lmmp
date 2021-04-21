mod avf;
mod mp3;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct Track {
    path: PathBuf,
    artist: String,
    album: String,
    track_no: u32,
    title: String,
}

impl Track {
    pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            match ext {
                "mp3" => mp3::parse(&path),
                "m4a" => Self::parse_mp4(&path),
                "ogg" | "flac" | "opus" => avf::parse(&path),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn parse_mp4(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
        use mp4ameta::Tag;

        fn parse_tags(t: Tag, p: &Path) -> Option<Track> {
            let track = match t.track() {
                (Some(t), _) => t as u32,
                _ => return None,
            };
            Some(Track {
                title: t.title()?.to_owned(),
                track_no: track,
                album: t.album()?.to_owned(),
                artist: t.album_artist().or_else(|| t.artist())?.to_owned(),
                path: p.to_owned(),
            })
        }

        let tag = mp4ameta::Tag::read_from_path(&path)?;
        Ok(parse_tags(tag, path.as_ref()))
    }
}
