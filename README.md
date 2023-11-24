# Introduction

This project contains minimal implementations of various scripting engines in Rust (V8, JSC, Spidermonkey, Python etc.) 
for the purpose of benchmarking.

# Benchmark
Test machine was a 2023 16" MacBook Pro with M2 Max CPU and 32GB RAM.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release --bin lua-mlua` | 161.4 ± 6.6 | 149.7 | 208.6 | 1.00 |
| `cargo run --release --bin js-jsc` | 162.6 ± 4.1 | 154.6 | 179.4 | 1.01 ± 0.05 |
| `cargo run --release --bin py-pyo3-py37` | 179.9 ± 3.4 | 170.6 | 191.6 | 1.11 ± 0.05 |
| `cargo run --release --bin js-v8` | 255.2 ± 4.7 | 246.6 | 273.8 | 1.58 ± 0.07 |
| `cargo run --release --bin js-spidermonkey` | 415.5 ± 7.7 | 387.4 | 435.9 | 2.57 ± 0.12 |
| `cargo run --release --bin js-deno` | 448.5 ± 15.4 | 408.1 | 493.0 | 2.78 ± 0.15 |
