# Discharged

Discharged is a new, lightweight-ish (11MB) Discord alternative/clone/competitor/aaaaaaaaaaa which is privacy-focused.

## Dependencies

**Total (direct)**: 9
**Total (indirect)**: 379

### Debug Mode

| Name        | Size      |
| ----------- | --------- |
| colorful    | 909.99 KB |
| ctrlc       | 174.47 KB |
| lazy_static | 23.58 KB  |
| mongodb     | 57.43 MB  |
| rand        | 2.13 MB   |
| rocket      | 31.51 MB  |
| serde       | 5.44 MB   |
| serde_json  | 3.61 MB   |
| tokio       | 21.87 MB  |
| tungstenite | 3.17 MB   |

### Release Mode

| Name        | Size     |
| ----------- | -------- |
| colorful    | 417 KB   |
| ctrlc       | 60.22 KB |
| lazy_static | 22.92 KB |
| mongodb     | 27.19 MB |
| rand        | 1.49 MB  |
| rocket      | 10.3 MB  |
| serde       | 5.67 MB  |
| serde_json  | 1.76 MB  |
| tokio       | 13.44 MB |
| tungstenite | 1.49 MB  |

## Binary Size

### Android ARMv8l

| Compile Mode | Size      |
| ------------ | --------- |
| debug        | 122.03 MB |
| release      | 11.12 MB  |

## Installation

Discharged currently can only be downloaded as a source tree. It is planned
that when most core features are finished, a [Release] will be provided.
[(roadmap)]

### From source

This method takes a while and also bullies your RAM.

```sh
git clone https://github.com/nearlySplat/discharged
cd discharged

# to run:
cargo run --release

# to compile for later use:
cargo build --release && mv target/release/discharged "$PREFIX/bin"
```

### Magic

Snap your fingers, say _"Abra cadabra"_ and the Discharged binary for your system will arrive on your computer!

_Warning: does not work most of the time_

[release]: ../../releases
[(roadmap)]: ROADMAP.md
