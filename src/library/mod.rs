mod ffmpeg;
mod mp3;
mod mp4;

use anyhow::Result;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

trait Parser {
    fn parse(path: impl AsRef<Path>) -> Result<Option<Track>>;
}

#[derive(Debug)]
pub struct Track {
    pub path: PathBuf,
    pub album_artist: String,
    pub album: String,
    pub track_no: u32,
    pub title: String,
}

impl Parser for Track {
    fn parse(p: impl AsRef<Path>) -> Result<Option<Track>> {
        let p = p.as_ref();
        Ok(if let Some(ext) = p.extension().and_then(OsStr::to_str) {
            let parser = match ext {
                // "opus" => opus::OpusParser::parse,
                // "ogg" => vorb::VorbParser::parse,
                "mp3" => mp3::MP3Parser::parse,
                "m4a" => mp4::Mp4Parser::parse,
                "jpg" | "png" | "log" => return Ok(None),
                _ => ffmpeg::AVFParser::parse, //  return Ok(None),
            };
            parser(&p)?
        } else {
            None
        })
    }
}

#[allow(dead_code)] // used by the opus and ogg parsers
fn parse_vorbis_comments(m: &HashMap<String, String>, p: &Path) -> Option<Track> {
    let get = |key: &str| m.get(key).map(|val| val.to_owned());
    Some(Track {
        album: get("ALBUM")?,
        album_artist: get("ALBUMARTIST").or_else(|| get("ARTIST"))?,
        title: get("TITLE")?,
        track_no: get("TRACKNUMBER").and_then(|n| n.parse::<u32>().ok())?,
        path: p.to_owned(),
    })
}

pub fn index<P: AsRef<Path>>(library_path: P) -> Result<Vec<Track>> {
    // let extensions = ["mp3", "opus", "ogg", "m4a", "flac"];
    let paths = WalkDir::new(library_path)
        .into_iter()
        .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?
        .iter()
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_owned())
        .collect::<Vec<PathBuf>>();
    // && e.path()
    //             .extension()
    //             .and_then(|os| os.to_str())
    //             .map_or(false, |ext| extensions.contains(&ext))
    let mut tracks: Vec<Track> = Vec::new();

    for path in paths {
        let start = Instant::now();
        if let Some(track) = Track::parse(&path)? {
            tracks.push(track);
        }
        let dur = Instant::now() - start;
        if dur.as_millis() > 10 {
            println!("{} took {} msecs", path.to_string_lossy(), dur.as_millis());
        }
    }

    Ok(tracks)
}
