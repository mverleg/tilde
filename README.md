# Tilde

A codegolf language - stack-based, small executable.

## Status

Tilde is in early development, and breaking changes in the language or code are likely. That is, if the project doesn't just die altogether.

## Semantics

Tilde is stack-based. The initial stack contains one vector, with a string value for each line of stdin in reverse.

To encode instructions as compactly as possible, each 'letter' is a half byte, so there are 16 only. But they can be combined in different ways, so in practice many instructions will be 1 byte, 1.5 byte or even longer

## Building locally

To build the executable:

    RUSTFLAGS="-C target-cpu=native" cargo +nightly build -Z build-std=std,panic_abort --target "$(rustc -vV | grep host | sed -E 's/.*: (.*)/\1/')" --no-default-features

The current goal is having only a single dependency (regex) and keep the binary size small, preferably under 0.5 MB.


