# discharged

A small, lightweight Discord alternative with absolutely no telemetry or personal data collection.

Recommended for selfhosting, but an online instance will be available soon(tm).

## Installation

```sh
# Download the source code
git clone https://github.com/nearlySplat/discharged
cd discharged

# Build the project (will take a while)
cargo build --release

# Install
mv target/release/discharged $PREFIX/bin
```

## Running Discharged

```sh
source .env

discharged
```
