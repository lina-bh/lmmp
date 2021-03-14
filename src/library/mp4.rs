use super::{Parser, Track};
use anyhow::Result;
use mp4ameta::Tag;
use std::path::Path;

pub struct Mp4Parser;

impl Parser for Mp4Parser {
    fn parse(path: impl AsRef<Path>) -> Result<Option<Track>> {
        let tag = mp4ameta::Tag::read_from_path(&path)?;
        Ok(parse_tags(tag, path.as_ref()))
    }
}

fn parse_tags(t: Tag, p: &Path) -> Option<Track> {
    let track = match t.track() {
        (Some(t), _) => t as u32,
        _ => return None,
    };
    Some(Track {
        title: t.title()?.to_owned(),
        track_no: track,
        album: t.album()?.to_owned(),
        album_artist: t.album_artist().or_else(|| t.artist())?.to_owned(),
        path: p.to_owned(),
    })
}
