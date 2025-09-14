#!/usr/bin/env bash
set -o errexit
set -o pipefail
set -x

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

REPO=$(git rev-parse --show-toplevel)

cd "$REPO" || exit 1

go build .

if [[ -x $(which strip) ]]; then
    strip ./projector
fi

mv ./projector "$INSTALL_DIR/"
