# Changelog

## [0.3.0](https://github.com/liblaf/github-bot/compare/v0.2.0..v0.3.0) - 2025-08-13

### 💥 BREAKING CHANGES

- **cli:** add shell completions and docs generation - ([fded12a](https://github.com/liblaf/github-bot/commit/fded12a6598c5b55533ff6aba0122ab8ae2a4424))

### ⬆️ Dependencies

- **deps:** update rust crate clap to v4.5.45 (#10) - ([211b542](https://github.com/liblaf/github-bot/commit/211b5429c2f0cff104626fb3f0a6acaa0bbc070a))

### ♻ Code Refactoring

- **core:** simplify execution and improve tracing - ([ebba0b3](https://github.com/liblaf/github-bot/commit/ebba0b31a8a52a3ea6aece490c81544064fa9d06))

### 🔧 Continuous Integration

- downgrade Python version for distribution - ([b07b380](https://github.com/liblaf/github-bot/commit/b07b3807319f0dac5565a261efb3babc53fca794))
- specify Python version for distribution - ([d7c85f7](https://github.com/liblaf/github-bot/commit/d7c85f767ad4461e764832a66e7c943ee63afb16))
- improve build workflow clarity - ([8641b8c](https://github.com/liblaf/github-bot/commit/8641b8c3836dd3385ea73d5b99f2cf57fba8ee8c))
- use centralized distribution script from copier-rust - ([be940bc](https://github.com/liblaf/github-bot/commit/be940bc8e47bf045cbcee8c321abae6af6ebc647))

## [0.2.0] - 2025-08-12

### 💥 BREAKING CHANGES

- **cli:** enhance I/O handling and add build automation - ([51afe2d](https://github.com/liblaf/github-bot/commit/51afe2decbf56878ee34af69ac601df77691f41c))

### ✨ Features

- initialize tangerine project - ([c22eeb5](https://github.com/liblaf/github-bot/commit/c22eeb5325a840d46c48127c33d2e494e298261c))

### ♻ Code Refactoring

- **build:** migrate distribution script to standalone - ([9aafa43](https://github.com/liblaf/github-bot/commit/9aafa4396ecadf858232892d5a67a445343cd757))

### 👷 Build System

- replace shell dist script with Python implementation - ([df61ee5](https://github.com/liblaf/github-bot/commit/df61ee5c51bc843963c6da3c29dc1e9d6b486274))

### 🔧 Continuous Integration

- reorganize and enhance GitHub workflows - ([ab3a286](https://github.com/liblaf/github-bot/commit/ab3a286ed794f3d74c602b5073c5dac529576136))
- use vendored OpenSSL in builds - ([44ec973](https://github.com/liblaf/github-bot/commit/44ec973fbdd17a1c26e1937c8058e4300d6344d8))
- add openssl dependency and disable fail-fast - ([b5d8e85](https://github.com/liblaf/github-bot/commit/b5d8e85b9a8f16b5c527c5468b92d239d77f2f27))
