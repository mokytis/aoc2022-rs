# Advent of Code 2022 in Rust

_Disclaimer: I have not written much rust so this code is by no means perfect_

## Running

Requires rust. Reccomended install using [rustup](https://rustup.rs/).
Challenge `day03` requires the nightly toolchain for `.array_chunks()`.

```bash
$ rustup toolchain install nightly
$ cargo +nightly build --release
$ ./targets/release/day03
```

## Inputs

Input files should be stored in `./inputs/`. If the day is the 3rd december the
test input should be at `./inputs/day03-test.txt` and the main challenge input
`./inputs/day03.txt`.

When running a binary, `--test-data` can be used to run the solution against
the test data, and `--input-file FILENAME` to specify an input file.
