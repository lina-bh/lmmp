use super::{Parser, Track};
use anyhow::Result;
use std::path::Path;

pub struct OpusParser;

impl Parser for OpusParser {
    fn parse(p: impl AsRef<Path>) -> Result<Option<Track>> {
        let p = p.as_ref();
        let op = opus_headers::parse_from_path(p)?;
        Ok(super::parse_vorbis_comments(&op.comments.user_comments, p))
    }
}
