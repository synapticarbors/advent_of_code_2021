#!/bin/sh

if [ $# != 1 ]; then
    echo "Usage: $(basename "$0") <day-number>" >&2
    exit 1
fi

name="$(printf "aoc%02d" "$1")"
cargo new --bin "$name" --vcs none

cp day_template/main.rs $name/src/
cat day_template/cargo_extras.txt >> $name/Cargo.toml

