#!/bin/bash
watchexec --exts rs,toml --clear --restart 'cargo fmt; cargo clippy; cargo llvm-cov --lcov --output-path lcov.info; cargo test'