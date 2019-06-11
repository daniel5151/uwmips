Instead of rewriting a assembler in Rust, I just exposed a C interface on my C++ assembler, compiled it as a library, and use Rust's FFI to interact with it.

Now, I can't publish my C++ assembler source code, since writing it is one of the CS241 assignments, so I can only distribute a static library version of it.

If you want to convert your own assembler to run in `uwmips`, you can do the following:

1. Expose the following functions as `extern C`:

```
// Accepts mips asm file, returns "" if success, or error string on error.
const char* assemble(const char* raw_lines);
// Returns assembled word, or -1 when out of words.
// (is called in a loop to retrieve asm)
uint64_t get_word();
```

I basically just copied my main function, but instead of accepting input from stdin, I used a stringstream that was instantiated with raw_lines. Also, instead of pushing assembled words directly to stdout, I modified the function to push words into a global dequeue. get_word just pop_fronts from that deque.

2. Compile your assembler as a static library.

```bash
clang++ -c -fPIC -static *.cc
ar crs libmipsasm.a *.o
mv libmipsasm.a /path/to/uwmips/lib
```

**Caveats:** Some associated tooling won't work unless you match my error strings exactly! I was too lazy to implement a proper error struct FFI, so I just propagate up my errors to Rust, and use string parsing to extract useful data from them.
