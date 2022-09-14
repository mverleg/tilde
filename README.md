# Tilde

A codegolf language - stack-based, small executable.

## Building locally

To build the executable:

    RUSTFLAGS="-C target-cpu=native" cargo +nightly build -Z build-std=std,panic_abort --target "$(rustc -vV | grep host | sed -E 's/.*: (.*)/\1/')" --no-default-features

The current goal is having only a single dependency (regex) and keep the binary size small, preferably under 0.5 MB.


