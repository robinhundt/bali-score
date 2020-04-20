# bali-score

This tool is inspired by the original implementation of the `bali-score` tool which accompanies the [BAliBASE 3](http://www.lbgi.fr/balibase/BalibaseDownload/BAliBASE_R1-5.tar.gz) database.

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
The tool has been reimplemented because I simply could not get it running and I had necessary code written already. I simply extracted it from my thesis to make it reusable.

