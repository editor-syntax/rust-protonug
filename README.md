# rust-protonug

Rust-Protonug is a command-line tool for installing and updating the GloriousEggroll (GE) custom Steam Proton builds on Linux. It simplifies the process of downloading, extracting, and managing GE Proton versions.

## Features

- Download the latest GE Proton release from GitHub
- Extract the downloaded release to the appropriate Steam directory
- Clean up older GE Proton versions

## Prerequisites

- Rust ofc
- Git

## Installation

1. Clone the repository:

```bash
git clone https://github.com/pneb/rust-protonug.git
```

2. Change to the cloned directory:

```bash
cd rust-protonug
```

3. Build and install the binary:

```bash
cargo build --release
sudo cp target/release/protonug /usr/local/bin/rust-protonug
```


## Usage

To use Rust-Protonug, run the following command:

```bash
rust-protonug update [--clean]
```


- `update`: Update the GE Proton build.
- `--clean` or `-c`: Remove older existing GE Proton builds before updating to the latest version.

## Example

To update the GE Proton build and clean up older versions, run:

```bash
rust-protonug update --clean
```

## Why are u here

- clout has been given to: [@VoltrexKeyva](https://github.com/VoltrexKeyva) as he made the original one
