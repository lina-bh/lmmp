use super::Track;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub fn parse(p: impl AsRef<Path>) -> Result<Option<Track>, Box<dyn Error>> {
    let p = p.as_ref();
    let op = opus_headers::parse_from_path(p)?;
    let tags = op.comments.user_comments;
    Ok(parse_tags(tags.clone(), p))
}

fn parse_tags(mut m: HashMap<String, String>, p: &Path) -> Option<Track> {
    Some(Track {
        album: m.remove("ALBUM")?,
        album_artist: m.remove("ALBUMARTIST").or_else(|| m.remove("ARTIST"))?,
        title: m.remove("TITLE")?,
        track_no: m
            .remove("TRACKNUMBER")
            .and_then(|n| n.parse::<u32>().ok())?,
        path: p.to_owned(),
    })
}
