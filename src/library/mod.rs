mod track;

use std::path::Path;
use track::Track;
use walkdir::WalkDir;

pub struct Library {
    tracks: Vec<Track>,
}

impl Library {
    pub fn index(path: impl AsRef<Path>) -> anyhow::Result<Library> {
        let mut tracks = Vec::new();

        let ents = WalkDir::new(path)
            .into_iter()
            .collect::<Result<Vec<walkdir::DirEntry>, walkdir::Error>>()?;

        let paths = ents
            .iter()
            .filter(|e| !e.file_type().is_dir())
            .map(|e| e.path());

        for path in paths {
            if let Some(tr) = Track::parse(path)? {
                tracks.push(tr);
            }
        }

        Ok(Library { tracks })
    }

    pub fn num_of_tracks(&self) -> usize {
        self.tracks.len()
    }
}
