# csv2

An example of native library to parse csv

> This is a work in progress

## Build (debug)
```sh
cargo greycat build
```

## Build (release)
```sh 
cargo greycat build --release
```

> `cargo-greycat build` does some GreyCat-specific work before calling `cargo build` propagating the arguments you've passed
> If the build is successful, it copies the built artifacts to `lib/csv2/csv2.gclib`

## Run
```sh
greycat run
```