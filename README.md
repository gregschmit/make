# Oxidized Make (omake)

This is a Rust implementation of `make`, striving to be simple, portable, and fast.

To avoid clashing with your system's `make`, this project is built as `omake` by default, but if
this project is ever used as the default implementation of `make` in a system, then it should be
named `make` (to follow the same convention as `bmake` and `gmake`).

I decided to try to re-write `make` in Rust both as a way to learn Rust and also because I found the
existing `make` implementations' source code very convoluted.

https://xkcd.com/2314/

## Installation

You can install with Cargo: `cargo install omake`. In the future, I may consider packaging this
project for other repos such as Homebrew or the AUR.

## Project Goals

This project is in its infancy, so I may find out later that some or all of the project goals are
impossible to achieve. Regardless, in order of importance, here are the project goals:

1. Portable makefiles and makefiles generated by tools like CMake should behave correctly.
2. Support as many commonly-used BSD and GNU `make` extensions as possible.
3. Be capable of building the Linux kernel.
4. Be really fast.
5. If we decide to implement new extensions, they should be opt-in to retain backwards
   compatibility. We should avoid this unless there are serious performance improvements.
6. Possibly the hardest: don't turn into a backwards-incompatible competing standard
   (https://xkcd.com/927/). As uninspired as it may seem, I just want an implementation of `make`
   that works on Linux and FreeBSD (and macOS); that's it.

1.0 release will probably happen when this project can build the Linux kernel.

Note that due to implementation details (and especially during the initial development phase this
project is in), it's possible certain features are inadvertently added. Users should probably not
rely on those and they may even qualify as bugs. I hope to get everything ironed out before the 1.0
release to avoid (as stated in Goal #6) building an incompatible competing standard. There are
already other build systems, I don't actually want to make another one.

Working list of things that I plan on leaving out of this implementation intentionally:
1. Remaking makefiles from RCS/SCCS. I see no need to support this.

## Testing Methodology

We have unit tests where they are feasible.

There is also a system test suite in the `tests` directory. In this context, "system" tests are
directories which contain a makefile, a `mod.rs` file, and any other files needed by the makefile.
The `mod.rs` invokes a `system_test_cases!` macro, which executes this project's resulting binary
against that directory's makefile given the arguments provided and checks STDOUT/STDERR against the
expected STDOUT/STDERR provided to the macro, and also checks the directory against the expected
files and content provided to the macro.

At some point, I should probably also copy over the GNU make test suite and try to get this project
to pass the entire test suite.
