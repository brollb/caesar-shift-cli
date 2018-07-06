# Caesar Shift CLI
This is a simple command line tool for performing Caesar Shift encryption/decryption. The primary goal was to gain more practice with rust and with CLI applications written in rust. Hopefully this project can also show

## Installation
Building the project requires rust ([rustup](https://rustup.rs/)). First, clone the project
```
git clone https://github.com/brollb/caesar-shift-cli
cd caesar-shift-cli
cargo install
```

# Usage
Messages can be encrypted using
```
caesar-shift-cli "my message to encrypt"
```

The message can then be decrypted by adding the `-d` flag:
```
caesar-shift-cli -d "pb phvvJjh wr hqfubsw"
```
