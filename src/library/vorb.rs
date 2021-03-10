use super::Track;

use lewton::inside_ogg::OggStreamReader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn parse(p: impl AsRef<Path>) -> Result<Option<Track>, Box<dyn Error>> {
    let p = p.as_ref();
    let f = File::open(p)?;
    let vf = OggStreamReader::new(f)?;
    let m = vf
        .comment_hdr
        .comment_list
        .clone()
        .into_iter()
        .collect::<HashMap<String, String>>();

    Ok(parse_tags(m, p))
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
