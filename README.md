# Fuzz Rustc

This repo contains configuration for fuzz-testing the Rust compiler using [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer),
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

The run-fuzzer.sh script passes trailing arguments on to the underlying libfuzzer binary,
so you can pass any of these options: https://llvm.org/docs/LibFuzzer.html#options .

For example, this invocation will run 4 jobs in parallel and will only try ascii inputs:

```sh
./run_fuzzer.sh -jobs=4 -only_ascii=1
```

## Bugs found

[#62524](https://github.com/rust-lang/rust/issues/62524)
[#62546](https://github.com/rust-lang/rust/issues/62546)
[#62554](https://github.com/rust-lang/rust/issues/62554)
[#62863](https://github.com/rust-lang/rust/issues/62863)
[#62881](https://github.com/rust-lang/rust/issues/62881)
[#62894](https://github.com/rust-lang/rust/issues/62894)
[#62895](https://github.com/rust-lang/rust/issues/62895)
[#62913](https://github.com/rust-lang/rust/issues/62913)
[#62973](https://github.com/rust-lang/rust/issues/62973)
[#63116](https://github.com/rust-lang/rust/issues/63116)
[#63135](https://github.com/rust-lang/rust/issues/63135)
[#66473](https://github.com/rust-lang/rust/issues/66473)
[#68629](https://github.com/rust-lang/rust/issues/68629)
[#68730](https://github.com/rust-lang/rust/issues/68730)
[#68890](https://github.com/rust-lang/rust/issues/68890)
[#69130](https://github.com/rust-lang/rust/issues/69130)
[#69310](https://github.com/rust-lang/rust/issues/69310)
[#69378](https://github.com/rust-lang/rust/issues/69378)
[#69396](https://github.com/rust-lang/rust/issues/69396)
[#69401](https://github.com/rust-lang/rust/issues/69401)
[#69600](https://github.com/rust-lang/rust/issues/69600)
[#69602](https://github.com/rust-lang/rust/issues/69602)
[#70549](https://github.com/rust-lang/rust/issues/70549)
[#70552](https://github.com/rust-lang/rust/issues/70552)
[#70594](https://github.com/rust-lang/rust/issues/70594)
[#70608](https://github.com/rust-lang/rust/issues/70608)
[#70677](https://github.com/rust-lang/rust/issues/70677)
[#70724](https://github.com/rust-lang/rust/issues/70724)
[#70736](https://github.com/rust-lang/rust/issues/70736)
[#70763](https://github.com/rust-lang/rust/issues/70763)
[#70813](https://github.com/rust-lang/rust/issues/70813)
[#70942](https://github.com/rust-lang/rust/issues/70942)
[#71297](https://github.com/rust-lang/rust/issues/71297)
[#71471](https://github.com/rust-lang/rust/issues/71471)
[#71798](https://github.com/rust-lang/rust/issues/71798)
[#72410](https://github.com/rust-lang/rust/issues/72410)
[#84104](https://github.com/rust-lang/rust/issues/84104)
[#84117](https://github.com/rust-lang/rust/issues/84117)
[#84148](https://github.com/rust-lang/rust/issues/84148)
[#84149](https://github.com/rust-lang/rust/issues/84149)
[#86895](https://github.com/rust-lang/rust/issues/86895)
[#88770](https://github.com/rust-lang/rust/issues/88770)
[#92267](https://github.com/rust-lang/rust/issues/92267)
[#102114](https://github.com/rust-lang/rust/issues/102114)
[#102751](https://github.com/rust-lang/rust/issues/102751)

## TODO

Generalize this setup to also work other fuzzing engines, like AFL and Honggfuzz.

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
