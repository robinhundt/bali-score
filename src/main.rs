use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Result};
use bali_score::{balibase, compute_scores, fasta};
use structopt::StructOpt;

/// This tool is inspired by the original implementation of the
/// `bali-score` tool which accompanies the
/// [BAliBASE 3](http://www.lbgi.fr/balibase/BalibaseDownload/BAliBASE_R1-5.tar.gz)
/// database.
/// Currently this tool only works for Balibase 3 reference alignments in `.xml`
/// format which have core blocks declared. It is not possible to declare
/// a threshold instead of core blocks.
#[derive(StructOpt, Debug)]
struct Opt {
    /// The alignment for which the SoP and CS score should be computed. Must be in `.fasta`
    /// format.
    #[structopt(short = "t", long, name = "TEST_FASTA", parse(from_os_str))]
    test_file: PathBuf,
    /// The Balibase version 3 reference align against which to compare the test alignment.
    /// Must be in `.xml` format containing core block data.
    #[structopt(short = "r", long, name = "REF_XML", parse(from_os_str))]
    ref_file: PathBuf,
    /// Where to store the computed score in `json` format (file extension will
    /// **not** be automatically appended). If no path is provided, the results
    /// will be written to stdout.
    #[structopt(short = "o", long, name = "OUT", parse(from_os_str))]
    out_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let test_alig = fasta::parse(&opt.test_file).with_context(|| {
        format!(
            "Unable to parse fasta test file at: {}",
            opt.test_file.to_string_lossy()
        )
    })?;
    let ref_alig = balibase::parse(&opt.ref_file).with_context(|| {
        format!(
            "Unable to parse xml reference file at: {}",
            opt.test_file.to_string_lossy()
        )
    })?;
    let scores = compute_scores(&ref_alig, &test_alig);

    match opt.out_file {
        Some(path) => {
            let file = File::create(&path).with_context(|| {
                format!(
                    "Could not create output file at: {}",
                    path.to_string_lossy()
                )
            })?;
            serde_json::to_writer_pretty(file, &scores)
                .context("Serialization of scores failed")?;
        }
        None => {
            println!("{:?}", scores);
        }
    }
    Ok(())
}
