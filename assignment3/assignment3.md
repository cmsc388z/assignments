CMSC388Z Homework #3
===

## Read Before You Start
1. This assignment is due on **October 8th, 2021 at noon**.
2. Please submit your `src/main.rs` and `src/lib.rs` onto [**GradeScope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).
3. Please make sure you are using the latest version of Rust.
    ```bash
    $ rustup update
    $ rustc --version
    rustc 1.55.0 (c8dfcfe04 2021-09-06)
    ```
4. Please make sure your program doesn't contain any warning or error when submitting.
5. Please feel free to refer to any *appropriate* online resource. If you are not sure, you can email Dongze (dhe17 *at* umd *dot* edu) or Chase (ckanipe *at* terpmail *dot* umd *dot* edu) for clarification.
6. This is an individual project, please do not discuss any code-related questions with anyone.
7. [This feedback survey](https://forms.gle/kon3fKNB8qyXf2AB9) will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us *anonymously*. 
8. We highly recommend using VS code + [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension. But it is OK if you have your preferred code editor.

## Please use Piazza to ask questions!
## Introduction

In this assignment, you need to implement a command-line utility that is similar to the Linux command `find` **from scratch**. That is, your program should be able to recursively search an input directory `root_dir` and all its subdirectories to find files that match some given [regex patterns](https://en.wikipedia.org/wiki/Regular_expression). You will find [this chapter](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) in TRPL very useful.

Through this assignment, you will practice 
1. Basic interaction with the operating system
2. Error handling
3. struct/impl
4. Lifetime
5. Rust Build System

Note that the error handling strategy we used in this assignment is very basic. The purpose is to only help you understanding the concept. For more advanced error handling strategies, please refer to [this chapter](https://doc.rust-lang.org/rust-by-example/error.html) in Rust By Example.
## Overview

Your program will find the files in a given set of directories or their sub-directories when their name matches at least one of the given [regex patterns](https://en.wikipedia.org/wiki/Regular_expression) and (optionally) when their size is over a given size threshold. By default, the path of the matched file names will be printed via stdout. Optionally, if the user specifies an output file path, the path of the matched files will be written to that file. For example,

```bash
$ rust_find --patterns ".*\.rs" --dirs "./src" "./tests"
./src/lib.rs
./src/main.rs
./tests/integrated_tests.rs
```

This command will return all files that end with `.rs` in the directory `./src` and `./tests`, and all their sub-directories. We will use Rust's regular expression engine, so the syntax for regular expressions may be different from what you're used to in other languages or shells.

If you would like to run your program when implementing, you can do

```bash
$ cargo run -- --patterns ".*\.rs" --dirs "./src"
./src/lib.rs
./src/main.rs
```

Notice that the `--` between `cargo run` and the program's arguments is to help `cargo` to recognize the arguments passing to it and the program's arguments. You may specify only one value for each argument when doing `cargo run`, here I use `./src` and `*./.rs`.

## Implementation details

There are mainly 5 steps in the assignment:

0. Setting up the environment
1. Building command-line interface
2. Taking and parsing arguments
3. Getting matched files
4. Exporting results

The provided guideline is designed to be tested easily and smoothly. **I encourage you to write a unit testing function for every function you implemented.** 


### set-up
In this assignment, we will build a [binary + library program](https://www.reddit.com/r/rust/comments/cj8f14/understanding_rust_binaries_libraries/). So, you need to first create a *lib* program. In your terminal, do the following:
```bash
cargo new rust_find --lib && cd rust_find
touch src/main.rs
mkdir -p "tests" && touch "tests/integrated_tests.rs"
```

Then, you need to specify that your program contains a binary and a library in `Cargo.toml`. Meanwhile, you can add all the dependencies we will use in this assignment into `Cargo.toml` as well.

rust_find/Cargo.toml

```rust
[lib]
name = "lib"
path = "src/lib.rs"

[[bin]]
name = "rust_find"
path = "src/main.rs"


[dependencies]
regex = "1.5"
slog = "2.7.0"
slog-term = "2.8.0"

[dependencies.clap]
version = "3.0.0-beta.4"
features = ["wrap_help"]

```

In this assignment, you will work on 3 files, `src/main.rs`, `src/lib.rs`, and `tests/integrated_tests.rs`.
1. In `src/main.rs`, we parse command-line arguments and evaluate a `run()` function, which is defined in the `src/lib.rs`. 
2. In `src/lib.rs`, we define all the structs and functions.
3. In `tests/integrated_tests.rs`, we write testing functions.

### Taking command-line arguments

We will use `clap-rs` to build our command-line interface. You can find many quick examples in their [document](https://docs.rs/clap/3.0.0-beta.2/clap/index.html). Here I will show that how to set it up using [*Builder Pattern*](https://riptutorial.com/rust/example/23631/using-clap) from a string. In the following example, for the `--patterns` argument, 
- `-p` is its short version
- `<patterns>` is saying the value name is `patterns`. You will use this name to access it in your code.
- The short sentence, "List of file patterns to find.", is a description of this argument, which will be displayed if you do `rust_find --help` after compiling. 


**TODO**: You need to also define `--dirs` or `-d` for short, which specifies **a set of directories** (`multiple_values(true)`) that your program will find the matched filenames from, and `--size`, or `-s` for short, which specifies the minimum size, in bytes, a matched files need to have to be reported. The `--size` argument is optional (`required(false)`).

src/main.rs
```rust
use clap::{Arg, App}; // tell Rust you will use these two structs in clap
use lib::{run,Config}; // tell Rust you will use these two things from our "lib" module

fn main() {
    // Define command-line interface
    let matches = App::new("rust_find")
        .version("0.1.0")
        .author("Your Name <you.email@umd.edu>") 
        .about("Find files that match a regex pattern")
        .arg(
            Arg::from("-p , --patterns=<patterns> 'List of file patterns to find.'")
                .takes_value(true)
                .required(true)
                .multiple_values(true), // this argument can takes multiple values
        )
        .arg(
            Arg::from("-o, --output=<output> 'Write results to output file instead of stdout.'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )
        // TODO: define --dirs here
        // TODO: define --size here
        .get_matches();
        // .get_matches_from(vec!["rust-find", "--patterns=.*/.rs", "--output=./tests.out"]);


    let args = Config::from_args(&matches); // will be defined later

    if let Err(err) = run(&args) { //Error handling here!
        panic!("{}", err)
    }
}
```

When writing unit testing for your command-line interface, you need to copy and paste the whole statement of defining `matches` in your test function, and use `.get_matches_from()` function instead of `get_matches()` to take the arguments from the vector you provided. 

### The `run()` function

We will define the `run()` function in `src/lib.rs` and use it in `src/main.rs` by specifying `use lib::run;` in `src/main.rs`. 

```rust
use clap::ArgMatches;
use regex::Regex;
use std::fs::File;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub fn run(config: &Config) -> Result<(), &'static str> {
    // 1. parse patterns
    let v_pats: Vec<Regex> = config.parse_patterns()?;

    // 2. get directories
    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;

    // 3. parse optional arguments
    let mut output: Option<File> = config.parse_output();

    let size: Option<u64> = config.parse_size();

    // 4. get files and output
    let mut matched_files = Vec::with_capacity(v_dirs.len());
    for dir in v_dirs.iter() {
        get_matched_files(&mut matched_files, dir, &v_pats[..], size);

        // print or write
        if let Some(sv) = display(&matched_files, &mut output) {
            for s in sv {
                println!("{}", s);
            }
        };
        matched_files.clear();
    }

    Ok(())
}
```

I give you the code because this is the logic of this program. Please copy and paste it to the top lines of your `src/lib.rs`. You need to implement everything used in this function. Notice that here I defined a `matched_files` vector at step #4 and use this all the time. This is because allocating a new vector in the heap is time-consuming.

When implementing the functions, you need to bubble up the non-crucial errors you met in the process instead of letting your program panic directly. For example, when you are unable to access a file, you want to print a warning message instead of letting your program crash. This is how we do error handling in the real world. The `?` used after a function call is a simple way to [bubble up the errors](http://www.sheshbabu.com/posts/rust-error-handling/#:~:text=also%3A%20unwrap_or_else%20%2C%20unwrap_or_default-,Bubble%20up%20the%20error,error%20to%20the%20caller%20function.&text=There%20are%20two%20function%20calls,json%20\)%20that%20return%20Result%20values).

### `Config` strct

Next, we are going to parse command-line arguments into struct `Config`. This stuct should contain 4 fields, correspond to the four input arguments. The following example doesn't work because I want you to use `&str`, which means you need to specify an explicit lifetime `<'a>` for them. [Here](https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust) is a good post talking about how to specify lifetime in a struct. You need to implement a `from_args()` function to build a `Config` struct using the `ArgMatches` struct of the `clap` crate. Find more information on how to access data in `ArgMatches` [document](https://docs.rs/clap/2.33.3/clap/struct.ArgMatches.html). The function signature is given below. You need to use explicit lifetime for the `from_args()` function as well.

src/lib.rs

```rust
pub struct Config {
    pub dirs: Vec<&str>,
    pub patterns: Vec<&str>,
    pub output: Option<&str>,
    pub size: Option<&str>,
}
impl Config { // you need to use explit lifetime here as well
    pub fn from_args(args: &ArgMatches) -> Self {
        unimplemented!()
    }
}
```

When accessing data in `ArgMatches`, you can use `values_of()` for arguments with multiple values and `value_of()` for arguments with single value. Notice that `values_of()` returns a `Result` while `value_of()` returns an `Option`.

To use this struct in `src/main.rs`, you need to specify `use lib::Config` in the top lines of `src/main.rs`. For more useful methods of `ArgMatches`, please refer to its [document](https://docs.rs/clap/2.33.3/clap/struct.ArgMatches.html).

### Parsing `Config` fields

Next, to make the four arguments in the `Config` fields useful, we need to parse them into their desired types. For example, we want the patterns to be in `Regex` type, the paths in `PathBuf` type, and the size in `u64` type. So, we need to implement some methods for the `Config` struct to parse those arguments. You need to [handle the errors](http://www.sheshbabu.com/posts/rust-error-handling/#:~:text=also%3A%20unwrap_or_else%20%2C%20unwrap_or_default-,Bubble%20up%20the%20error,error%20to%20the%20caller%20function.&text=There%20are%20two%20function%20calls,json%20\)%20that%20return%20Result%20values) carefully in this step. ( Please use the `match` keyword or any appropriate ways to handle the error appropriately. Don't use `.unwrap()` all the time! ) For example, when one of the input patterns is invalid, instead of letting your program panic, you should print a [warning message](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html)  and move to the next pattern. You want to return `Err()` only if none of the patterns is valid. For optional arguments such as `output`, when the given output file path is invalid, you should just ignore the input argument and print a warning message.

src/lib.rs

```rust
impl Config {
    pub fn parse_patterns(&self) -> Result<Vec<Regex>, &'static str> {
        unimplemented!()
    }

    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        unimplemented!()
    }

    pub fn parse_output(&self) -> Option<File> {
        unimplemented!()
    }

    pub fn parse_size(&self) -> Option<u64> {
        unimplemented!()
    }
}
```

During the implementation, you may find the following functions and methods useful:

```rust
// To build regex pattern
use regex::Regex;
Regex::new();

// To check whether a vector is empty
let v = Vec::new();
v.is_empty();

// To build path in Rust
use std::path::PathBuf;
PathBuf::from();

// To create a file 
use std::fs::File;
File::create();

// To get the metadata of a &Path
path.metadata()


```

### Finding matched files

With the parsed patterns, we can now find files in the given directories that match the given patterns! We will achieve this goal by defining a `get_matched_files()` function. This function takes a directory and the rest three parsed arguments and finds the files that match at least one of the given patterns and build a `MyFile` struct for each of them. You will run this function for each given directory in the `dirs` argument.

```rust 
pub fn get_matched_files(
    files: &mut Vec<MyFile>,
    dir: &Path,
    pats: &[Regex],
    size: Option<u64>,
) {
    unimplemented!()
    // call get_matched_files() in itself if the given directory `dir` contains a sub-directory
}

pub struct MyFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}
impl MyFile {
    /// Instantiate a MyFile struct from the path of a file.
    pub fn from_path(path: &Path) -> Result<Self, &'static str> {
        unimplemented!()
    }
}
```

You would like to run `get_matched_files()` recursively because you need to find all sub-directories in a given directory. Notice that your code may meet some errors when accessing directories or files, you would like to print a warning if it happens and move to the next one. You may find the following functions and methods useful:

```rust
// To get an iterator over the entries within a directory.
let dir = std::env::current_dir().unwrap();
for rd in std::fs::read_dir(dir).unwrap() {
    // To get path from read_dir() result
    let path = rd.unwrap().path();
    // To check whether a path is a directory
    path.is_dir();
};

// To check whether a path is a directory or a file
path.is_file();

// To check whether a str matches a regex pattern
use regex::Regex;
let my_str = ".*/.rs" ;
let regex = Regex::new(my_str);
regex.is_match("main.rs");
```

## Exporting output

With the matched files in hand, we can now export the filenames via stdout or to the `output` file. We can define a `display()` function to do this. This function will take the matched files as the input, and write the `path` field of the matched files to the `output` file, or return a vector of strings if no `output` file is given. You will use `writeln!()` for writing to a file line by line.

```rust
use std::io::Write;
pub fn display(files: &[MyFile], output: &mut Option<File>)-> Option<Vec<String>> {
    unimplemented!()
}
```

## Notes

I want to emphasize that the error handling strategy used in this assignment is only for helping you understand the idea. For more advanced error handling strategies, please refer to [this chapter](https://doc.rust-lang.org/rust-by-example/error.html) in Rust By Example.

For command-line interface, you can find a series of examples and tutorials in `clap-rs`'s [GitHub repo](https://github.com/clap-rs/clap). There is another popular crate `StructOpt` built upon `clap-rs` for command-line argument parsing. They will be integrated in the near future.

## Submission
Please run the following cargo commands before submitting:
```bash
$ cargo clippy
$ cargo check
$ cargo fmt
```

Please submit you `src/main.rs` and `src/lib.rs` onto [GradeScope](https://www.gradescope.com/courses/291105). If you cannot access the GradeScope course page, please let us know.


