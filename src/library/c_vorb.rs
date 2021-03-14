use super::{Parser, Track};
use crate::vorbfile::VorbisFile;
use anyhow::Result;
use std::path::Path;

pub struct VorbParser;

impl Parser for VorbParser {
    fn parse(path: impl AsRef<Path>) -> Result<Option<Track>> {
        let mut vf = VorbisFile::open(&path)?;
        let met = vf.comments();

        Ok(super::parse_vorbis_comments(&met, path.as_ref()))
    }
}
