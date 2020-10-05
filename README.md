# food
Application for generating shopping lists from recipes

![Build & Test](https://github.com/mantono/food/workflows/Build%20&%20Test/badge.svg)

## Usage
```
USAGE:
    food [FLAGS] [OPTIONS] [path]...

FLAGS:
    -D, --debug
            Print debug information about current build for binary, useful for when an issue is encountered and reported

    -h, --help
            Prints help information

    -s, --simple
            Only use simple recipes, with less ingredients, as far as possible

    -V, --version
            Prints version information


OPTIONS:
    -l, --limit <limit>
            Select how many different recipes to use when generating the shopping list [default: 7]

    -S, --seed <seed>
            Set the seed value which will be used to seed the random generator. Setting a different seed value will
            change which recipes are selected. The seed value is automatically updated on weekly basis.
    -z, --serving-size <serving_size>
            Set a custom serving size for each recipe

    -v, --verbosity <verbosity>
            Set the verbosity level, from 0 (least amount of output) to 5 (most verbose). Note that logging level
            configured via RUST_LOG overrides this setting. [default: 1]

ARGS:
    <path>...
            Select zero, one or several directories for which to look for files in. If no value is give, the application
            will default to current directory. [default: .]


```

#### Example
Select seven recipes from the current folder and subdirectories, and generate a shopping list

`food`

Select five recipes and generate a shopping list

`food -l 5 my_recipes/`

### Recipe Format
See [format](/format) for instructions on how to write recipes.

## Building
The application is built with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Simply run the following command in the project directory.
```bash
cargo build --release
```
A binary will be created and put in directory `target/release`.

## Install
Run `cargo install --path .`
