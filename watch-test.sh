#!/bin/bash
watchexec --exts rs,toml --clear --restart 'cargo fmt; cargo test'