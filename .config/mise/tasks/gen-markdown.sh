#!/bin/bash
set -o errexit
set -o nounset
set -o pipefail

mkdir --parents --verbose 'docs'
cargo run --all-features -- gen-markdown > 'docs/help.md'
prettier --write 'docs/help.md'
