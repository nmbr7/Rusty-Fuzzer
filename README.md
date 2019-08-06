# **Rusty-Fuzzer**
__A Coverage Guided Evolutionary Fuzzer Written in Rust__


*_STATUS :  :heavy_check_mark:  stable_*

## # Installation

### Installing Rust Programming language
#### _In Unix-like OS:_ 
```
curl https://sh.rustup.rs -sSf | sh
```

**OR** [visit official _**Rust**_ website ](https://www.rust-lang.org/tools/install)

### Building and Running Rusty-Fuzzer
1) Building from git source
```
git clone https://github.com/nmbr7/Rusty-Fuzzer.git
cd Rusty-Fuzzer
cargo build --release
```
2) Running the fuzzer
```
cd target/release
./rusty-fuzzer  --help
rusty_fuzzer 0.1.0
nmbr_7
A grey box evolutionary fuzzer

USAGE:
    rusty-fuzzer [OPTIONS] --input <INPUT_COMMAND> --seed <SEED_FILE_DIR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT_COMMAND>     Input program and arguments where the argument to be fuzzed is specified by '@'
    -t, --inputtype <INPUT_TYPE>    Input type taken by the program (FileInput(f) or command line TextInput(c))
    -s, --seed <SEED_FILE_DIR>      Seed directory to use

```


## # Notes
This program has only been tested on a Linux (Arch linux) system.
It should build and run on all other linux and unix-like systems, though it is not guaranteed. 


_**This documentation is not complete, and other details related to binary instrumentations and fuzzing a binary will be updated shortly**_ 
