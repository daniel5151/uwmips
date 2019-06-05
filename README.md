# uwmips

`uwmips` is a reimplementation of the MIPS VM used by CS241 and CS230 at the University of Waterloo.

It's written in Rust, which means the code is clean and easy to read, while still being super performant!

## Building

`cargo build --release`, and it pops out in `./target/release/uwmips`.

You can also `cargo install` to install it globally.

## Usage

The single `uwmips` binary implements all the frontends.

```
Usage: uwmips [frontend] <filename> [...args] [load_address]
  frontend: twoints     - <no args>
            twointsargs - <int1> <int2>
            array       - <no args>
            noargs      - <no args>
```
