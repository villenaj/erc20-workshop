#!/bin/bash

# Loop through each directory and run `cargo test`
for dir in */; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "Running tests in $dir"
    (cd "$dir" && cargo test)
  fi
done
