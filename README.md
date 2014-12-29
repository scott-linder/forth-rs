# forth #

A Forth interpreter in Rust. Inspired by
[monofuel's](https://github.com/monofuel/monoForth) interpreter in C++.

## lib ##

The core interpreter is defined as a library crate so that it can be used by
other Rust code. The current design is very limited, only supporting integral
values and a single stack (which is not visible to the calling Rust code) but
in the future it should be possible to use the lib as a means of embedding a
Forth interpreter into your Rust applications.

## bin ##

To guide development of the library crate (and perhaps for it's own utility)
there is also a binary crate which uses the library to implement a command-line
Forth interpreter.
