# bali-score

This tool is inspired by the original implementation of the `bali-score` tool which accompanies the [BAliBASE 3](http://www.lbgi.fr/balibase/BalibaseDownload/BAliBASE_R1-5.tar.gz) database.  
Currently this tool only works for BAliBASE 3 reference alignments in `.xml` format which have core blocks declared. It is not possible to declare a threshold instead of core blocks.

## Installing the tool

The tool can be used as a library for other projects or as a CLI tool. Installing it as a CLI tool depends on a stable Rust toolchain being available (recommended install via [rustup](https://rustup.rs)) and can be done by executing

```
cargo install --path .
```

at the top level of the project.

## Running the CLI tool

Execute 
```
bali-score -h
```
to see the available options. Scoring a test alignment in `fasta` format against a BAliBASE v3 reference alignment in `xml` format is as easy as running:
```
bali-score -t <test fasta file> -r <reference xml> [-o <output file>]
```

If no output file path is provided, the result is written to stdout, otherwise the scores are written as a `json` file at the provided path. 

## Why the reimplementation?
The tool has been reimplemented because I simply could not get the original one running and I had the necessary code mostly written already. I simply extracted it from my thesis to make it reusable.  
Most importantly it is not dependant upon the Expat xml parser or any other dynamic dependency (except libc).

## Feature requests
If you'd like a feature that is currently not available, e.g. using other file formats like `msf` as input, please open an issue describing the use case and if it sounds useful/interesting I might implement it :) No guarantees though.

## TODOs (mostly just ideas for which I'll probably have no time or motivation to implement)
- add license
    - is it problematic that original bali-score contains no license? This tool shares no code with it, but currently has the same name...
- accept other file formats as input
- document library
- publish as binary and clearly documented lib on crates.io
- provide c compatible methods
- allow evaluation against reference alignments without core block information (see original tool)
- write some tests
- write some benchmarks
- set up CI
- are there other scores/databases that could be supported? Maybe this tool could become more general than just BAliBASE
