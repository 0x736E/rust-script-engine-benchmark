#!/bin/zsh
hyperfine --runs 100 --warmup 5 --export-markdown bench.md \
  'cargo run --release --bin js-spidermonkey' \
  'cargo run --release --bin js-deno' \
  'cargo run --release --bin js-jsc' \
  'cargo run --release --bin js-v8' \
  'cargo run --release --bin lua-mlua' \
  'cargo run --release --bin py-pyo3-py37' \
