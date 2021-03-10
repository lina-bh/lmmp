use super::Track;
use id3::Tag;
use std::error::Error;
use std::path::PathBuf;

pub fn parse(path: PathBuf) -> Result<Option<Track>, Box<dyn Error>> {
    let tags = Tag::read_from_path(&path)?;
    Ok(parse_tags(tags, path))
}

fn parse_tags(t: Tag, p: PathBuf) -> Option<Track> {
    Some(Track {
        title: t.title()?.to_owned(),
        track_no: t.track()?,
        album: t.album()?.to_owned(),
        album_artist: t.album_artist().or_else(|| t.artist())?.to_owned(),
        path: p,
    })
}
