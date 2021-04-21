use super::Track;
use id3::Tag;
use std::path::Path;

pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
    fn parse_tags(t: Tag, p: &Path) -> Option<Track> {
        Some(Track {
            title: t.title()?.to_owned(),
            track_no: t.track()?,
            album: t.album()?.to_owned(),
            artist: t.album_artist().or_else(|| t.artist())?.to_owned(),
            path: p.to_owned(),
        })
    }

    let path = path.as_ref();
    let tags = Tag::read_from_path(path)?;
    Ok(parse_tags(tags, path))
}
