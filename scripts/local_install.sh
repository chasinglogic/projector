#!/usr/bin/env bash
set -o errexit
set -o pipefail
set -x

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

REPO=$(git rev-parse --show-toplevel)

cd "$REPO" || exit 1

cargo build --release

if [[ -x $(which strip) ]]; then
    strip ./target/release/projector
fi

mv ./target/release/projector "$INSTALL_DIR/"
