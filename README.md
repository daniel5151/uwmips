# uwmips

`uwmips` is a reimplementation of the MIPS VM used by CS241 and CS230 at the University of Waterloo.

It has some basic debugging features. Passing the `--step` flag lets you step through the code one instruction at a time.
I might add more features, really depends on demand (from my buddies taking the course rn) and available time.

It's written in Rust, which means the code is pretty clean and easy to read, and is really performant :smile:

Bug reports appreciated, and feature requests welcome.

## Building

`cargo build --release`, and it pops out in `./target/release/uwmips`.

You can also `cargo install --path .` to install it globally (you might have to add the cargo bin directory to your path though).

It should build with any recent version of the Rust toolchain, including the one on the UW Student CS servers.

## Usage

The single `uwmips` binary implements all the frontends.

```
Usage: uwmips [OPTIONS] [frontend] <filename> [...args] [load_address]
   OPTIONS: --step      Dump state after every CPU instruction
                          and wait for 'enter' key to continue

  frontend: twoints     - <no args>
            twointsargs - <int1> <int2>
            array       - <no args>
            noargs      - <no args>
```

## Screenshots

Running with `--step` gives you a overview of the current state of the program:

```
  -------------==== Stack ====-------------
       ADDR    |     HEX     |     VAL
  -------------|-------------|-------------
   0x00ffffd4  | 0x00000000  | 0
   0x00ffffd8  | 0x00000000  | 0
   0x00ffffdc  | 0x00000000  | 0
   0x00ffffe0  | 0x00000000  | 0
   0x00ffffe4  | 0x00000000  | 0
   0x00ffffe8  | 0x00000000  | 0
>  0x00ffffec  | 0x0000001c  | 28
   0x00fffff0  | 0x00000030  | 48
   0x00fffff4  | 0x00000000  | 0
   0x00fffff8  | 0x00000005  | 5
   0x00fffffc  | 0x8123456c  | -2128394900
   0x01000000  | 0x00000000  | 0
   0x01000004  | 0x00000000  | 0

  ---------====== Program RAM ======--------
     ADDR    |   HEXVAL   :     MIPS ASM
  -----------|------------------------------
  0x00000034 | 0xafc2fff8 : sw $2, -8($30)
  0x00000038 | 0xafc4fff4 : sw $4, -12($30)
  0x0000003c | 0xafdffff0 : sw $31, -16($30)
  0x00000040 | 0x00002014 : lis $4
  0x00000044 | 0x00000010 : mfhi $0
  0x00000048 | 0x03c4f022 : sub $30, $30, $4
> 0x0000004c | 0x00001820 : add $3, $0, $0
  0x00000050 | 0x10200008 : beq $1, $0, 8
  0x00000054 | 0x00201020 : add $2, $1, $0
  0x00000058 | 0x00002014 : lis $4
  0x0000005c | 0x00000001 : .word 0x00000001 (1)
  0x00000060 | 0x00240822 : sub $1, $1, $4
  0x00000064 | 0x00002014 : lis $4

------------------------------------------------====== CPU State ======------------------------------------------------
$01 = 0x00000005 (5)          $02 = 0x00000000 (0)          $03 = 0x00000000 (0)          $04 = 0x00000010 (16)
$05 = 0x00000000 (0)          $06 = 0x00000000 (0)          $07 = 0x00000000 (0)          $08 = 0x00000000 (0)
$09 = 0x00000000 (0)          $10 = 0x00000000 (0)          $11 = 0x00000000 (0)          $12 = 0x00000000 (0)
$13 = 0x00000000 (0)          $14 = 0x00000000 (0)          $15 = 0x00000000 (0)          $16 = 0x00000000 (0)
$17 = 0x00000000 (0)          $18 = 0x00000000 (0)          $19 = 0x00000000 (0)          $20 = 0x00000000 (0)
$21 = 0x00000000 (0)          $22 = 0x00000000 (0)          $23 = 0x00000000 (0)          $24 = 0x00000000 (0)
$25 = 0x00000000 (0)          $26 = 0x00000000 (0)          $27 = 0x00000000 (0)          $28 = 0x00000000 (0)
$29 = 0x00000000 (0)          $30 = 0x00ffffec (16777196)   $31 = 0x0000001c (28)         $pc = 0x0000004c
```

Hit enter to advance to the next state.

## Wait, why build this thing if you're not even in CS 241 right now lol?

My buddy is taking 230, and I thought it would be a fun exercise to re-implement my old MIPS simulator ([mips241](https://github.com/daniel5151/mips241)) in Rust. V1 took literally 3 hours to rewrite (thanks Pattern Matching!) so it was a fun little diversion to help a friend :smile:
