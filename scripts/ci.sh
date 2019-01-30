#!/usr/bin/env bash
rustup component add clippy
rustup target add x86_64-unknown-linux-musl
#RUSTFLAGS="-D warnings" cargo check --all
cargo clippy -- -D clippy::all -D warnings \
    -A renamed-and-removed-lints #this option is workaround (see https://issues.apache.org/jira/browse/THRIFT-4764)