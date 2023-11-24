# Introduction

This project contains minimal implementations of various JavaScript engines in Rust (V8, JSC, Spidermonkey) for the 
purpose of benchmarking.

# Benchmark
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --bin js-jsc` | 164.9 | 164.9 | 164.9 | 1.00 |
| `cargo run --release --bin lua-mlua` | 165.8 | 165.8 | 165.8 | 1.01 |
| `cargo run --release --bin py-pyo3-py37` | 170.7 | 170.7 | 170.7 | 1.04 |
| `cargo run --release --bin js-v8` | 250.9 | 250.9 | 250.9 | 1.52 |
| `cargo run --release --bin js-spidermonkey` | 417.1 | 417.1 | 417.1 | 2.53 |
| `cargo run --release --bin js-deno` | 424.2 | 424.2 | 424.2 | 2.57 |
