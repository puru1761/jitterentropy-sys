# Raw (low-level) Rust Bindings for `libjitterentropy`

![CI Badge](https://github.com/puru1761/jitterentropy-sys/actions/workflows/main.yml/badge.svg)
![License Badge](https://img.shields.io/github/license/puru1761/jitterentropy-sys)
![Crate Badge](https://img.shields.io/crates/v/jitterentropy-sys.svg)

This repository contains the raw low-level rust bindings for [libjitterentropy](https://github.com/smuellerDD/jitterentropy-library/tree/v3.3.1).
**DO NOT** use these bindings directly in your code. A new Rusty API will be
provided in the `jent` crate.

## Pre-requisites

Prior to building this project, clone this repository, and also checkout
all it's included sub-modules recursively.

```
git clone https://github.com/puru1761/jitterentropy-sys.git --recurse-submodules
```

## Author

* Purushottam A. Kulkarni <<puruk@protonmail.com>>
