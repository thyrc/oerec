#!/bin/bash

set -eu
set -o pipefail

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/rust -w /usr/src/rust rust:latest bash -c "cargo build --target x86_64-unknown-linux-musl --target x86_64-unknown-linux-gnu --release && strip --strip-unneeded -R .comment target/x86_64-unknown-linux-musl/release/oerec; strip --strip-unneeded -R .comment target/x86_64-unknown-linux-gnu/release/oerec" &>/dev/null
ls -lapv target/x86_64-unknown-linux-*/release/oerec;
