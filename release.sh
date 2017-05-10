#!/usr/bin/env sh

echo "Have you bumped versions in README.md and Cargo.toml?"
help2man "cargo run --" > man/man1/skeleton.1
echo "Generated manpage"
cargo pkgbuild
echo "Generated PKGBUILD"

echo "Next steps: push release, publish crate and update AUR package"
