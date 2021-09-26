# Discharged

Discharged is a new, lightweight-ish (11MB) Discord alternative/clone/competitor/aaaaaaaaaaa which is privacy-focused.

## Dependencies

**Total (direct)**: 9
**Total (indirect)**: 379

### Debug Mode

<<debug>>

### Release Mode

<<release>>

## Binary Size

### Android ARMv8l

| Compile Mode | Size          |
| ------------ | ------------- |
| debug        | <><debug><>   |
| release      | <><release><> |

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
