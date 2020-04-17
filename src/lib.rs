use std::fmt;
use std::ops::Add;
use std::str;

use itertools::Itertools;
use num_integer::binomial;
use serde::{Deserialize, Serialize};

pub mod balibase;
pub mod fasta;

#[derive(Clone, Debug)]
pub struct Alignment {
    pub name: String,
    pub aligned_data: Vec<Sequence>,
    pub unaligned_data: Vec<Sequence>,
    unaligned_to_aligned_pos: Vec<Vec<usize>>,
    pub core_blocks: Vec<bool>,
}
#[derive(Clone, PartialEq)]
pub struct Sequence {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Site {
    pub seq: usize,
    pub pos: usize,
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum PositionAlignment {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(Default, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AlignmentScores {
    pub sum_of_pairs: f64,
    pub column_score: f64,
}

pub fn compute_scores(
    ref_alignment: &Alignment,
    test_alignment: &Alignment,
    ignore_symbol_case: bool,
) -> AlignmentScores {
    let seq_cnt = test_alignment.aligned_data.len();
    let mut correctly_aligned = 0;
    let mut correct_column_count = 0;
    let test_column_cnt = test_alignment
        .aligned_data
        .first()
        .expect("Empty alignment")
        .data
        .len();

    let mut gaps_per_seq = vec![0; seq_cnt];
    let mut gaps_per_col = vec![0; test_column_cnt];
    let mut correct_in_column = vec![false; test_column_cnt];

    for column in 0..test_column_cnt {
        gaps_per_col.iter_mut().for_each(|el| *el = 0);
        correct_in_column.iter_mut().for_each(|el| *el = false);
        for (seq_id, seq) in test_alignment.aligned_data.iter().enumerate() {
            if seq.data[column] == Alignment::GAP_CHARACTER {
                gaps_per_seq[seq_id] += 1;
                gaps_per_col[seq_id] += 1;
            }
        }
        for ((a_idx, seq_a), (b_idx, seq_b)) in test_alignment
            .aligned_data
            .iter()
            .enumerate()
            .tuple_combinations()
        {
            let a = seq_a.data[column];
            let b = seq_b.data[column];

            if a == Alignment::GAP_CHARACTER || b == Alignment::GAP_CHARACTER {
                continue;
            }

            let site_a = Site {
                seq: a_idx,
                pos: column - gaps_per_seq[a_idx],
            };
            let site_b = Site {
                seq: b_idx,
                pos: column - gaps_per_seq[b_idx],
            };

            let symbols_aligned =
                ignore_symbol_case || a.is_ascii_uppercase() && b.is_ascii_uppercase();

            if symbols_aligned
                && ref_alignment.pos_aligned(site_a, site_b) == PositionAlignment::Correct
            {
                correct_in_column[a_idx] = true;
                correct_in_column[b_idx] = true;
                correctly_aligned += 1;
            }
        }
        let correct_in_column_sum: usize = correct_in_column.iter().copied().map(usize::from).sum();
        // TODO Note that this only works for ref alignments with core blocks where every seq
        // is aligned in this position
        if correct_in_column_sum == seq_cnt {
            correct_column_count += 1;
        }
    }

    let sum_of_pairs = correctly_aligned as f64 / true_site_pair_count(ref_alignment) as f64;
    let column_score = correct_column_count as f64 / ref_alignment.core_block_columns() as f64;
    AlignmentScores {
        sum_of_pairs,
        column_score,
    }
}

fn true_site_pair_count(alignment: &Alignment) -> usize {
    let core_block_data = alignment.core_block_data();
    let aligned_seq_len = core_block_data[0].data.len();
    let aligned_pos_per_col = (0..aligned_seq_len).map(|pos| {
        core_block_data.iter().fold(
            0,
            |acc, seq| {
                if seq.data[pos] == b'-' {
                    acc
                } else {
                    acc + 1
                }
            },
        )
    });

    aligned_pos_per_col.fold(0, |acc, aligned_count| acc + binomial(aligned_count, 2))
}

impl Add for AlignmentScores {
    type Output = AlignmentScores;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.sum_of_pairs += rhs.sum_of_pairs;
        self.column_score += rhs.column_score;
        self
    }
}

impl Alignment {
    pub const GAP_CHARACTER: u8 = b'-';
    pub fn new(name: String, aligned_data: Vec<Sequence>, core_blocks: Vec<bool>) -> Self {
        let (unaligned_data, pos_mapping) = aligned_data
            .iter()
            .map(|seq| seq.clone().into_unaligned())
            .unzip();

        Self {
            name,
            aligned_data,
            unaligned_data,
            unaligned_to_aligned_pos: pos_mapping,
            core_blocks,
        }
    }

    pub fn pos_aligned(&self, pos1: Site, pos2: Site) -> PositionAlignment {
        let pos_mapping = &self.unaligned_to_aligned_pos;
        let mapped_1 = pos_mapping[pos1.seq][pos1.pos];
        let mapped_2 = pos_mapping[pos2.seq][pos2.pos];
        if mapped_1 == mapped_2 && self.core_blocks[mapped_1] {
            PositionAlignment::Correct
        } else if mapped_1 != mapped_2 && (self.core_blocks[mapped_1] || self.core_blocks[mapped_2])
        {
            PositionAlignment::Incorrect
        } else {
            // mapped to position is not part of core block region
            PositionAlignment::Unknown
        }
    }

    pub fn core_block_data(&self) -> Vec<Sequence> {
        self.aligned_data
            .iter()
            .map(|seq| seq.select_core_blocks(&self.core_blocks))
            .collect()
    }

    pub fn core_block_columns(&self) -> usize {
        self.core_blocks.iter().copied().map(usize::from).sum()
    }
}

impl Sequence {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns new Sequence with stripped gaps und Vec that matches each unaligned position to
    /// it's original aligned position
    fn into_unaligned(self) -> (Self, Vec<usize>) {
        let (positions, no_gap_data) = self
            .data
            .into_iter()
            .enumerate()
            .filter(|(_, el)| *el != Alignment::GAP_CHARACTER)
            .unzip();
        let unaligned_sequence = Self {
            name: self.name,
            data: no_gap_data,
        };
        (unaligned_sequence, positions)
    }

    fn select_core_blocks(&self, core_blocks: &[bool]) -> Self {
        if self.data.len() != core_blocks.len() {
            panic!("Sequence len must be equal to core blocks len")
        }
        let data = self
            .data
            .iter()
            .zip(core_blocks)
            .filter_map(|(el, core_block)| if *core_block { Some(*el) } else { None })
            .collect();
        Self {
            name: self.name.clone(),
            data,
        }
    }
}

impl fmt::Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sequence {{ name: {}, data: {} }}",
            self.name,
            str::from_utf8(&self.data).unwrap()
        )
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ">{}\n{}",
            self.name,
            str::from_utf8(&self.data).map_err(|_| fmt::Error)?
        )
    }
}
