#!/bin/bash
set -o errexit
set -o nounset
set -o pipefail

function gen_complete {
  local shell="$1"
  local filename="$2"
  local complete_dir="completions/$shell"
  mkdir --parents --verbose "$complete_dir"
  cargo run --all-features -- completion "$shell" > "$complete_dir/$filename"
}

gen_complete bash tangerine
gen_complete fish tangerine.fish
gen_complete zsh _tangerine
