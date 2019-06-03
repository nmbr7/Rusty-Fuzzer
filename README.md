# **Rusty-Fuzzer**
__A Coverage Guided Evolutionary Fuzzer Written in Rust__


*_STATUS :  :negative_squared_cross_mark:  Unstable_*

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
./rusty-fuzzer
```
<img src="/Images/help.png" width=70% height=70% >

## # Notes
This program has only been tested on a Linux (Arch linux) system.
It should build and run on all other linux and unix-like systems, though it is not guaranteed. 

:bangbang: The Fuzzer is currently experimental.

**Other details related to binary instrumentations will be updated shortly** 
