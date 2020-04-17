use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use bali_score::{balibase, compute_scores, fasta};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "t", long, name = "TEST_FASTA", parse(from_os_str))]
    test_file: PathBuf,
    #[structopt(short = "r", long, name = "REF_XML", parse(from_os_str))]
    ref_file: PathBuf,
    #[structopt(short = "o", long, name = "OUT", parse(from_os_str))]
    out_file: Option<PathBuf>,
    #[structopt(long)]
    ignore_symbol_case: bool,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let test_alig = fasta::parse(opt.test_file)?;
    let ref_alig = balibase::parse(opt.ref_file)?;
    let scores = compute_scores(&ref_alig, &test_alig, opt.ignore_symbol_case);

    match opt.out_file {
        Some(path) => {
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &scores)?;
        }
        None => {
            println!("{:?}", scores);
        }
    }
    Ok(())
}
