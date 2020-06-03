use std::convert::TryFrom;
use std::path::Path;
use std::str;

use anyhow::{Context, Result};
use needletail::{parse_sequence_path, SequenceRecord};

use crate::{Alignment, Sequence};

/// Reads an Alignment contained in a fasta file into the Alignment struct.
/// The name of the Alignment is the `file_stem` (the non-extension part)
/// of the provided path.
pub fn parse(path: impl AsRef<Path>) -> Result<Alignment> {
    let mut sequences = vec![];
    parse_sequence_path(&path, |_| {}, |seq| sequences.push(Sequence::try_from(seq)))?;
    let aligned_data: Vec<_> = sequences.into_iter().collect::<Result<_>>()?;

    let name = path
        .as_ref()
        .file_stem()
        .context("No filestem for fasta file")?
        .to_string_lossy()
        .into_owned();
    let alig_len = aligned_data.first().context("Empty alignment")?.data.len();
    let core_blocks = vec![true; alig_len];

    Ok(Alignment::new(name, aligned_data, core_blocks))
}

impl<'a> TryFrom<SequenceRecord<'a>> for Sequence {
    type Error = anyhow::Error;

    fn try_from(seq: SequenceRecord) -> Result<Self> {
        Ok(Sequence {
            name: str::from_utf8(&seq.id)?.to_string(),
            data: seq.seq.to_vec(),
        })
    }
}
