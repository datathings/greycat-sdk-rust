# csv2

An example of native library to parse csv

> This is a work in progress

## Install
Make sure you have `cargo-greycat` installed. In this repository you can run:
```sh
cargo install --path ../../cargo-greycat
```
Or from the root:
```sh
cargo install --path cargo-greycat
```
Or from Github:
```sh
cargo install --git https://github.com/datathings/greycat-sdk-rust.git --branch dev
```

## Build (debug)
> Make sure you are in `examples/csv-native-library` before running this command
```sh
cargo greycat build
```

## Build (release)
> Make sure you are in `examples/csv-native-library` before running this command
```sh 
cargo greycat build --release
```

> `cargo-greycat build` does some GreyCat-specific work before calling `cargo build` propagating the arguments you've passed
> If the build is successful, it copies the built artifacts to `lib/csv2/csv2.gclib`

## Run
> Make sure you are in `examples/csv-native-library` before running this command
```sh
greycat run
```