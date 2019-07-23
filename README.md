# Fuzz Rustc

This repo contains configuration for fuzz-testing the Rust compiler using [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer-sys),
taking inspiration from [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [fuzz-targets](https://github.com/rust-fuzz/targets).

Because [rustc](https://github.com/rust-lang/rust) is a bootstrapping compiler, its build process has several stages
and involves juggling many flags, attributes, and environment variables. These complications create some difficulties for
cleanly setting up fuzz testing. We work around those difficulties with some
[light modifications to rustc](https://github.com/dwrensha/rust/tree/fuzz) and some additional configuration.


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
[#62863](https://github.com/rust-lang/rust/issues/62863)
[#62881](https://github.com/rust-lang/rust/issues/62881)
[#62894](https://github.com/rust-lang/rust/issues/62894)
[#62895](https://github.com/rust-lang/rust/issues/62895)
[#62913](https://github.com/rust-lang/rust/issues/62913)

## TODO

Generalize this setup to also work other fuzzing engines, like AFL and Honggfuzz.

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
