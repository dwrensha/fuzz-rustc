# Fuzz Rustc

Configuration for fuzz testing the Rust compiler using [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer-sys),
taking inspiration from [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [fuzz-targets](https://github.com/rust-fuzz/targets).

Builds a [lightly modified rustc](https://github.com/dwrensha/rust/tree/fuzz) with fuzzing instrumentation and runs
a coverage-guided fuzzer on it.

## Running


```sh
./run-fuzzer.sh
```

You may add some example inputs in the `./seeds/` directory.

New interesting test cases are automatically written to the `./corpus/` directory as they are found.

## Bugs found

[#62524](https://github.com/rust-lang/rust/issues/62524)
[#62646](https://github.com/rust-lang/rust/issues/62546)
[#62554](https://github.com/rust-lang/rust/issues/62554)

## TODO

Generalize this setup to also work other fuzzing engines, like AFL and Honggfuzz.

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
