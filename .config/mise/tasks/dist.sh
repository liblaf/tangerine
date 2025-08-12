#!/bin/bash
set -o errexit
set -o nounset
set -o pipefail

cargo build --release

cargo read-manifest |
  yq '.targets | filter(.kind | contains(["bin"])) | .[].name' |
  while read -r name; do
    mkdir --parents --verbose "dist"
    cp --archive --no-target-directory "target/release/$name" "dist/$name-"
  done
