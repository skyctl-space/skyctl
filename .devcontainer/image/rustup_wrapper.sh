#!/bin/sh

# When invoking rustup we need to override the cargo home, otherwise packets will end up in the wrong place
export CARGO_HOME=/opt/rust
/opt/rust/bin/${0##*/} "$@"
