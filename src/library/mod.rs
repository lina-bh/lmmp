use crate::track::Track;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

type ArtistMap = HashMap<String, AlbumMap>;
type AlbumMap = HashMap<TrackK, TrackV>;
#[derive(PartialEq, Eq, Hash, Serialize)]
struct TrackK {
    track: u32,
    disc: Option<u32>,
}
#[derive(Serialize)]
struct TrackV {
    path: PathBuf,
    title: String,
}

#[derive(Default, Serialize)]
pub struct Library {
    path: PathBuf,
    artists: HashMap<String, ArtistMap>,
}

impl Library {
    pub fn index(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut self_ = Self {
            path: path.as_ref().to_owned(),
            artists: Default::default(),
        };

        let ents = WalkDir::new(path)
            .into_iter()
            .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?;

        let paths = ents
            .iter()
            .filter(|e| !e.file_type().is_dir())
            .map(|e| e.path());

        for path in paths {
            if let Some(tr) = Track::parse(path)? {
                self_.add(tr)?;
            }
        }

        Ok(self_)
    }

    fn add(&mut self, tr: Track) -> anyhow::Result<()> {
        let Track {
            track_no,
            title,
            artist,
            album,
            path,
        } = tr;
        let albums = self.artists.entry(artist).or_insert(Default::default());
        let album = albums.entry(album).or_insert(Default::default());
        album.insert(
            TrackK {
                track: track_no,
                disc: None,
            },
            TrackV {
                path: path.strip_prefix(&self.path)?.to_owned(),
                title,
            },
        );
        Ok(())
    }
}
