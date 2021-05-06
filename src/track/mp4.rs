use super::Track;
use std::path::Path;

pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
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
