# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.11...moverox-codegen-v0.0.12)

### ⛰️ Features

- *(moverox-codegen)* Filter out used phantom types - ([11b9802](https://github.com/0xangelo/moverox/commit/11b9802c50507be0f8eb36001a1d38e7746114ba))


## [0.0.11](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.10...moverox-codegen-v0.0.11)

### ⛰️ Features

- *(moverox-codegen)* `#[ext(moverox(type_({T} = OTW)))]` annotations - ([2081a2f](https://github.com/0xangelo/moverox/commit/2081a2f5f14ed4c5ee7c4e4728de0f3c89ed7ba2))


## [0.0.10](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.9...moverox-codegen-v0.0.10)

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: move-syn - ([0000000](https://github.com/0xangelo/moverox/commit/0000000))


## [0.0.9](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.8...moverox-codegen-v0.0.9)

### ⛰️ Features

- *(moverox-codegen)* [**breaking**] Avoid phantom field if phantom type is used - ([d47ce9e](https://github.com/0xangelo/moverox/commit/d47ce9e4965dc965ee625526c6816833a32378c5))

### 🚜 Refactor

- *(moverox-codegen)* Better check for enum phantoms - ([db6e305](https://github.com/0xangelo/moverox/commit/db6e305251cca8a924fcd5d4c86fa1b8461ce24d))

### ⚙️ Miscellaneous Tasks

- *(moverox-codegen)* Rename test case - ([2e02720](https://github.com/0xangelo/moverox/commit/2e0272071ed2f79d9826233b885e1f3b6094e0f0))


## [0.0.8](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.7...moverox-codegen-v0.0.8)

### 🚜 Refactor

- *(move-syn)* [**breaking**] Align attribute grammar with original impl - ([e4b190a](https://github.com/0xangelo/moverox/commit/e4b190a834450bdd70672610591c16fbeb3e1295))
- [**breaking**] R `move_syn::TypePath` -> `move_syn::ItemPath` - ([dc3056f](https://github.com/0xangelo/moverox/commit/dc3056f8f174ee3c8ed3d5c2444d7ce8c9e783fd))

### 🧪 Testing

- Include all features in `public-api.rs` - ([729a874](https://github.com/0xangelo/moverox/commit/729a87438b8efb5ec3f58e2b77f8a81e71e40e75))

### ⚙️ Miscellaneous Tasks

- *(move-syn)* Release v0.0.5-beta.6 ([#71](https://github.com/0xangelo/moverox/pull/71)) - ([3bbd99f](https://github.com/0xangelo/moverox/commit/3bbd99fa1587acbf37ecbff32c7d94d355069db2))
- *(move-syn)* Release v0.0.5-beta.5 ([#66](https://github.com/0xangelo/moverox/pull/66)) - ([03b3993](https://github.com/0xangelo/moverox/commit/03b399356b0f41c7813fadb1626fa8ea76f48a3d))
- *(move-syn)* Release v0.0.5-beta.4 ([#63](https://github.com/0xangelo/moverox/pull/63)) - ([f4a48bc](https://github.com/0xangelo/moverox/commit/f4a48bc6575e06d26f13020df6d375375840fae2))
- *(move-syn)* Release v0.0.5-beta.3 ([#62](https://github.com/0xangelo/moverox/pull/62)) - ([d6bd6f0](https://github.com/0xangelo/moverox/commit/d6bd6f0dd57e440c5aa76d795fb9049be6928628))
- *(move-syn)* Release v0.0.5-beta.2 ([#59](https://github.com/0xangelo/moverox/pull/59)) - ([b43747f](https://github.com/0xangelo/moverox/commit/b43747fec43a6fd6fd4703f18619a6623e872435))


## [0.0.7](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.6...moverox-codegen-v0.0.7)

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: move-syn - ([0000000](https://github.com/0xangelo/moverox/commit/0000000))


## [0.0.6](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.5...moverox-codegen-v0.0.6)

### ⛰️ Features

- *(move-syn)* [**breaking**] Granular function argument parsing ([#33](https://github.com/0xangelo/moverox/pull/33)) - ([3dd2ad6](https://github.com/0xangelo/moverox/commit/3dd2ad6f29b03b62f80e0b496afe63c5e9389959))

### 🚜 Refactor

- Enable `feature(doc_cfg)` only on nightly - ([bef4127](https://github.com/0xangelo/moverox/commit/bef4127d13442f4ad1d709a40d6bb91764976468))

### ⚙️ Miscellaneous Tasks

- *(move-syn)* Release v0.0.5-beta - ([7de04ab](https://github.com/0xangelo/moverox/commit/7de04abcd9875105130023e1cb0c6d60122dd02d))


## [0.0.5](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.4...moverox-codegen-v0.0.5)

### 🐛 Bug Fixes

- *(moverox-codegen)* Remove `#[cfg(not(doctest))]` from modules - ([9735dca](https://github.com/0xangelo/moverox/commit/9735dca7ed2d012a0c610ea01cfe20e563596530))


## [0.0.4](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.3...moverox-codegen-v0.0.4)

### ⛰️ Features

- *(moverox-codegen)* Hide generated doc attrs in doctests - ([fc6796c](https://github.com/0xangelo/moverox/commit/fc6796c438ef0e1b9cd350716aa44684fa571f4e))


## [0.0.2](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.1...moverox-codegen-v0.0.2)

### 📚 Documentation

- Tell docs.rs to build with all features and untable options - ([4e6596d](https://github.com/0xangelo/moverox/commit/4e6596d5e830a3d07fa0649b5da46726231718b1))


## [0.0.1](https://github.com/0xangelo/moverox/compare/moverox-codegen-v0.0.0...moverox-codegen-v0.0.1)

### ⛰️ Features

- *(moverox-codegen)* Move to Rust enums - ([4f64133](https://github.com/0xangelo/moverox/commit/4f64133067d39c21b3f4b65b9ba7b93f771ecf8b))

### 🧪 Testing

- *(crates)* Snapshot public API - ([45a3b8e](https://github.com/0xangelo/moverox/commit/45a3b8e11ce76e14498965af61e457a1b80663fb))

### ⚙️ Miscellaneous Tasks

- Init workspace - ([f5f4804](https://github.com/0xangelo/moverox/commit/f5f4804fe2dde0a7ab6e00fc3227d7fcd33a44e5))

