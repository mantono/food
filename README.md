# prune
Parse recipes

![Build & Test](https://github.com/mantono/food/workflows/Build%20&%20Test/badge.svg)

## Usage
```

```

#### Example
The following command will look for all files being 300 megabyte or larger (`-s 300m`), in the current directory and up to five directory levels
below (`-d 5`) stopping when ten files (`-l 10`) have been found.

`prn -s 300m -d 5 -l 10`

Symlinks will never be followed, as this could potentially result in infinite loops when traversing through directories.

## Building
The application is built with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Simply run the following command in the project directory.
```bash
cargo build --release
```
A binary will be created and put in directory `target/release`. 

## Install
Run `cargo install --path .`
