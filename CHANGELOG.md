# Changelog

## [0.4.0](https://github.com/liblaf/tangerine/compare/v0.3.3..v0.4.0) - 2025-09-11

### 💥 BREAKING CHANGES

- rewrite project from Rust to Python implementation - ([6b26dce](https://github.com/liblaf/tangerine/commit/6b26dce458a9cc9356d33948388308e462f6b72b))

### ✨ Features

- **templates:** add Python badges and projects templates - ([0d97858](https://github.com/liblaf/tangerine/commit/0d97858467a605d24fd772bd1069c5c0713b4a92))

### ⬆️ Dependencies

- **deps:** update rust crate grapes to v0.2.3 (#27) - ([2247f95](https://github.com/liblaf/tangerine/commit/2247f958afba3eca6bfea2bd6f6c4e9a68415679))

### 📝 Documentation

- **templates:** reorder copier projects list - ([9042d75](https://github.com/liblaf/tangerine/commit/9042d750321d09f0ca4159bb95f86c9927b82584))

### ❤️ New Contributors

- [@liblaf](https://github.com/liblaf) made their first contribution
- [@github-actions[bot]](https://github.com/apps/github-actions) made their first contribution in [#30](https://github.com/liblaf/tangerine/pull/30)
- [@renovate[bot]](https://github.com/apps/renovate) made their first contribution in [#27](https://github.com/liblaf/tangerine/pull/27)

## [0.3.3](https://github.com/liblaf/tangerine/compare/v0.3.2..v0.3.3) - 2025-09-07

### ⬆️ Dependencies

- **deps:** update rust crate clap to v4.5.47 (#26) - ([26a8a87](https://github.com/liblaf/tangerine/commit/26a8a87575d52280bec61f2d1466ddd3c27e7a94))
- **deps:** update rust crate grapes to v0.2.2 (#25) - ([9cc148a](https://github.com/liblaf/tangerine/commit/9cc148a29ba1e0d693075f687b83f919642e8f15))
- **deps:** update rust crate tracing-subscriber to v0.3.20 [security] (#23) - ([5ab440d](https://github.com/liblaf/tangerine/commit/5ab440de1a3ee19bca6678c83b1a74e5f273f4f0))

## [0.3.2](https://github.com/liblaf/tangerine/compare/v0.3.1..v0.3.2) - 2025-08-31

### ⬆️ Dependencies

- **deps:** update rust crate clap to v4.5.46 (#20) - ([a864461](https://github.com/liblaf/tangerine/commit/a86446129b3536bde6f5509315bd069154903244))
- **deps:** update rust crate minijinja to v2.12.0 (#18) - ([2d71143](https://github.com/liblaf/tangerine/commit/2d71143ed31c48476e13780a2f41b7dccacb1795))
- **deps:** update rust crate regex to v1.11.2 (#16) - ([88c2fe8](https://github.com/liblaf/tangerine/commit/88c2fe83d577a07ffe7d2a087d411128d859f210))
- **deps:** update rust crate grapes to v0.2.1 (#14) - ([295d8ae](https://github.com/liblaf/tangerine/commit/295d8ae42f3f756bccdb3eb54d28854b09de15cd))

## [0.3.1](https://github.com/liblaf/tangerine/compare/v0.3.0..v0.3.1) - 2025-08-24

### ⬆️ Dependencies

- **deps:** update grapes to v0.2 - ([00d19f5](https://github.com/liblaf/tangerine/commit/00d19f549cc861d6441c0a9e014f8572b1104e6b))

## [0.3.0](https://github.com/liblaf/tangerine/compare/v0.2.0..v0.3.0) - 2025-08-14

### 💥 BREAKING CHANGES

- **cli:** add shell completions and docs generation - ([fded12a](https://github.com/liblaf/tangerine/commit/fded12a6598c5b55533ff6aba0122ab8ae2a4424))

### ⬆️ Dependencies

- **deps:** update rust crate clap to v4.5.45 (#10) - ([211b542](https://github.com/liblaf/tangerine/commit/211b5429c2f0cff104626fb3f0a6acaa0bbc070a))

### ♻ Code Refactoring

- **core:** simplify execution and improve tracing - ([ebba0b3](https://github.com/liblaf/tangerine/commit/ebba0b31a8a52a3ea6aece490c81544064fa9d06))

### 🔧 Continuous Integration

- downgrade Python version for distribution - ([b07b380](https://github.com/liblaf/tangerine/commit/b07b3807319f0dac5565a261efb3babc53fca794))
- specify Python version for distribution - ([d7c85f7](https://github.com/liblaf/tangerine/commit/d7c85f767ad4461e764832a66e7c943ee63afb16))
- improve build workflow clarity - ([8641b8c](https://github.com/liblaf/tangerine/commit/8641b8c3836dd3385ea73d5b99f2cf57fba8ee8c))
- use centralized distribution script from copier-rust - ([be940bc](https://github.com/liblaf/tangerine/commit/be940bc8e47bf045cbcee8c321abae6af6ebc647))

### ❤️ New Contributors

- [@renovate[bot]](https://github.com/apps/renovate) made their first contribution in [#10](https://github.com/liblaf/tangerine/pull/10)

## [0.2.0] - 2025-08-12

### 💥 BREAKING CHANGES

- **cli:** enhance I/O handling and add build automation - ([51afe2d](https://github.com/liblaf/tangerine/commit/51afe2decbf56878ee34af69ac601df77691f41c))

### ✨ Features

- initialize tangerine project - ([c22eeb5](https://github.com/liblaf/tangerine/commit/c22eeb5325a840d46c48127c33d2e494e298261c))

### ♻ Code Refactoring

- **build:** migrate distribution script to standalone - ([9aafa43](https://github.com/liblaf/tangerine/commit/9aafa4396ecadf858232892d5a67a445343cd757))

### 👷 Build System

- replace shell dist script with Python implementation - ([df61ee5](https://github.com/liblaf/tangerine/commit/df61ee5c51bc843963c6da3c29dc1e9d6b486274))

### 🔧 Continuous Integration

- reorganize and enhance GitHub workflows - ([ab3a286](https://github.com/liblaf/tangerine/commit/ab3a286ed794f3d74c602b5073c5dac529576136))
- use vendored OpenSSL in builds - ([44ec973](https://github.com/liblaf/tangerine/commit/44ec973fbdd17a1c26e1937c8058e4300d6344d8))
- add openssl dependency and disable fail-fast - ([b5d8e85](https://github.com/liblaf/tangerine/commit/b5d8e85b9a8f16b5c527c5468b92d239d77f2f27))

### ❤️ New Contributors

- [@liblaf-bot[bot]](https://github.com/apps/liblaf-bot) made their first contribution in [#9](https://github.com/liblaf/tangerine/pull/9)
- [@liblaf](https://github.com/liblaf) made their first contribution
