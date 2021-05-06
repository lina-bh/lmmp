use super::Track;
use ffmpeg_next as ffmpeg;
use std::path::Path;

pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
    fn parse_tags(m: &ffmpeg::DictionaryRef, p: &Path) -> Option<Track> {
        let get = |key| m.get(key).map(|val| val.to_owned());
        let track_total = get("track")?;
        let track = track_total
            .split("/")
            .take(1)
            .collect::<Vec<&str>>()
            .get(0)
            .and_then(|n| n.parse::<u32>().ok())?;
        let title = get("title")?;
        let album = get("album")?;
        let artist = get("album_artist").or_else(|| get("artist"))?;
        Some(Track {
            path: p.to_owned(),
            title,
            album,
            artist,
            track_no: track,
        })
    }
    let avf = match ffmpeg::format::input(&path) {
        Ok(f) => f,
        Err(e) => match e {
            ffmpeg::Error::InvalidData => return Ok(None),
            u => return Err(u.into()),
        },
    };
    let met = avf.metadata();
    let tags = parse_tags(&met, path.as_ref());
    if tags.is_some() {
        Ok(tags)
    } else {
        for stream in avf.streams() {
            let met = stream.metadata();
            let tags = parse_tags(&met, path.as_ref());
            if tags.is_some() {
                return Ok(tags);
            }
        }
        Ok(None)
    }
}
