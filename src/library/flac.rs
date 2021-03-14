use super::{Parser, Track};
use anyhow::Result;
use metaflac::Tag;
use std::path::Path;

pub struct FlacParser;

impl Parser for FlacParser {
    fn parse(path: impl AsRef<Path>) -> Result<Option<Track>> {
        let tag = Tag::read_from_path(&path)?;
        Ok(parse_tags(tag, path.as_ref()))
    }
}

fn parse_tags(t: Tag, p: &Path) -> Option<Track> {
    // Some(Track {
    //     title: t.title()?.to_owned(),
    //     track_no: t.track()?,
    //     album: t.album()?.to_owned(),
    //     album_artist: t.album_artist().or_else(|| t.artist())?.to_owned(),
    //     path: p.to_owned(),
    // })
    unimplemented!()
}
