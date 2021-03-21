mod ffmpeg;
mod mp3;
mod mp4;

use defaultmap::DefaultHashMap;
use indexmap::IndexSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::usize;
use walkdir::WalkDir;

#[derive(Default)]
pub struct Library {
    artists: IndexSet<String>,
    artist_albums: DefaultHashMap<usize, IndexSet<String>>,
    tracks: Vec<LibraryTrack>,
}

#[derive(Debug)]
pub struct _Track<T> {
    pub path: PathBuf,
    pub album_artist: T,
    pub album: T,
    pub track_no: u32,
    pub title: String,
}

pub type Track = _Track<String>;

type LibraryTrack = _Track<usize>;

impl<T> _Track<T> {
    pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Option<Track>> {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            match ext {
                "mp3" => mp3::parse(&path),
                "m4a" => mp4::parse(&path),
                "ogg" | "flac" | "opus" => ffmpeg::parse(&path),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl Library {
    pub fn index(path: impl AsRef<Path>) -> anyhow::Result<Library> {
        let mut lib = Library::default();

        let ents = WalkDir::new(path)
            .into_iter()
            .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?;

        let paths = ents
            .iter()
            .filter(|e| !e.file_type().is_dir())
            .map(|e| e.path());

        for path in paths {
            lib.load_file(path)?;
        }

        Ok(lib)
    }

    fn load_file(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let Track {
            path,
            album_artist,
            album,
            track_no,
            title,
        } = match Track::parse(&path)? {
            None => return Ok(()),
            Some(tr) => tr,
        };
        let (artist_idx, _) = self.artists.insert_full(album_artist);
        let albums = self.artist_albums.get_mut(artist_idx);
        let (album_idx, _) = albums.insert_full(album);
        let tr = LibraryTrack {
            album: album_idx,
            album_artist: artist_idx,
            path,
            track_no,
            title,
        };
        self.tracks.push(tr);

        Ok(())
    }

    pub fn artists(&self) -> impl Iterator<Item = &str> {
        self.artists.iter().map(String::as_str)
    }

    pub fn albums(&self, artist: &str) -> Option<impl Iterator<Item = &str>> {
        self.artists
            .get_index_of(artist)
            .map(|idx| self.artist_albums.get(&idx).iter().map(String::as_str))
    }

    pub fn album(&self, artist: &str, album: &str) -> Vec<Track> {
        let (artist_idx, album_idx) = match self.artists.get_index_of(artist).and_then(|idx| {
            let album_idx = self.artist_albums[idx].get_index_of(album);
            Some(idx).zip(album_idx)
        }) {
            Some(p) => p,
            None => return Vec::new(),
        };
        let mut v = self
            .tracks
            .iter()
            .filter(|tr| tr.album_artist == artist_idx && tr.album == album_idx)
            .map(|tr| Track {
                path: tr.path.clone(),
                title: tr.title.clone(),
                track_no: tr.track_no,
                album: self.artist_albums[artist_idx]
                    .get_index(album_idx)
                    .unwrap()
                    .to_string(),
                album_artist: self.artists.get_index(artist_idx).unwrap().to_string(),
            })
            .collect::<Vec<Track>>();
        v.sort_by_key(|tr| tr.track_no);

        v
    }
}
