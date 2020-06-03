# bali-score

This tool is inspired by the original implementation of the `bali-score` tool which accompanies the [BAliBASE 3](http://www.lbgi.fr/balibase/BalibaseDownload/BAliBASE_R1-5.tar.gz) database.  
Currently this tool only works for Balibase 3 reference alignments in `.xml` format which have core blocks declared. It is not possible to declare a threshold instead of core blocks.

## Installing the tool

The tool can be used as a library for other projects or as a CLI tool. Installing it as a CLI tool depends on a stable Rust toolchain being available (recommended install via [rustup](https://rustup.rs)) and can be done by executing

```
cargo install --path .
```

at the top level of the project.

## Running the CLI tool

Execute 
```
spam-align -h
```
to see the available options.

## Why the reimplementation?
The tool has been reimplemented because I simply could not get the original one running and I had the necessary code mostly written already. I simply extracted it from my thesis to make it reusable.

