use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use quick_xml::de::from_reader;
use serde::Deserialize;

use crate::{Alignment, Sequence};

pub fn parse(path: impl AsRef<Path>) -> Result<Alignment> {
    let bb_alignment = BBAlignment::from_xml_file(path)?;
    Ok(bb_alignment.into())
}

#[derive(Debug, Deserialize)]
struct XMLRoot {
    alignment: BBAlignment,
}

#[derive(Debug, Deserialize)]
struct BBAlignment {
    #[serde(rename = "aln-name")]
    name: String,
    #[serde(rename = "sequence")]
    sequences: Vec<BBSequence>,
    #[serde(rename = "column-score")]
    core_block: BBCoreBlock,
}

#[derive(Debug, Deserialize)]
struct BBSequence {
    #[serde(rename = "seq-name")]
    name: String,
    #[serde(rename = "seq-data")]
    data: String,
}

#[derive(Debug, Deserialize)]
struct BBCoreBlock {
    #[serde(rename = "colsco-data")]
    data: String,
}

impl From<BBAlignment> for Alignment {
    fn from(bb_alignment: BBAlignment) -> Self {
        let core_blocks = bb_alignment
            .core_block
            .data
            .split(' ')
            .map(|block| {
                let block_num: i32 = block.trim().parse().unwrap_or_else(|_| {
                    panic!("Encountered non i32 in core block data: {}", block)
                });
                block_num == 1
            })
            .collect();
        let seqs = bb_alignment
            .sequences
            .into_iter()
            .map(|seq| seq.into())
            .collect();

        Self::new(bb_alignment.name, seqs, core_blocks)
    }
}

impl From<BBSequence> for Sequence {
    fn from(bb_seq: BBSequence) -> Self {
        let data = bb_seq
            .data
            .bytes()
            .filter(|el| !el.is_ascii_whitespace())
            .collect();

        Self {
            name: bb_seq.name,
            data,
        }
    }
}

impl BBAlignment {
    fn from_xml_file(path: impl AsRef<Path>) -> Result<BBAlignment> {
        let file = BufReader::new(File::open(&path)?);
        let xml_root: XMLRoot = from_reader(file)?;
        Ok(xml_root.alignment)
    }
}
