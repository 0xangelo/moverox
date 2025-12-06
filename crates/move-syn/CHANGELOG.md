# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.5-beta.2](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.5-beta.1...move-syn-v0.0.5-beta.2)

### 🐛 Bug Fixes

- *(move-syn)* Parsing of `const` values - ([d46acc7](https://github.com/0xangelo/moverox/commit/d46acc7f2d8d355eccab2929752352e9cf9f3d03))

### 🧪 Testing

- *(move-syn)* Ensure we can parse DeepBookV3's code - ([1b943fe](https://github.com/0xangelo/moverox/commit/1b943fe514afd7ebab8e2ee663e27d1fbdb5b007))


## [0.0.5-beta.1](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.5-beta...move-syn-v0.0.5-beta.1)

### ⛰️ Features

- *(move-syn)* `Attribute::metas` - ([3afec25](https://github.com/0xangelo/moverox/commit/3afec25575cdb15127d210928d0536ab1661fd7b))

### 🐛 Bug Fixes

- *(move-syn)* `Attribute::external_attributes` - ([2be5954](https://github.com/0xangelo/moverox/commit/2be5954df32fd662001fe89e8cecd256a85da8ea))


## [0.0.5-beta](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.5-alpha.2...move-syn-v0.0.5-beta)

### ⛰️ Features

- *(move-syn)* `Attribute::external_attributes` - ([498f58e](https://github.com/0xangelo/moverox/commit/498f58e2e22d1a0d6cce998bfc2ddb6c41294da1))

### 🐛 Bug Fixes

- *(move-syn)* [**breaking**] Forbid some trailing delimiters ([#34](https://github.com/0xangelo/moverox/pull/34)) - ([f0f9f1d](https://github.com/0xangelo/moverox/commit/f0f9f1da3d4120a516258489225850c0e953b817))

### 🚜 Refactor

- *(move-syn)* Remove unnecessary `pub` - ([9efa704](https://github.com/0xangelo/moverox/commit/9efa704c15a77f3accf12d50b658e887dfe778d8))
- Enable `feature(doc_cfg)` only on nightly - ([bef4127](https://github.com/0xangelo/moverox/commit/bef4127d13442f4ad1d709a40d6bb91764976468))

### 🧪 Testing

- *(move-syn)* Function with compound attribute - ([5b84b62](https://github.com/0xangelo/moverox/commit/5b84b624674173a9dc47f1deab5d9fd948032029))

### ⚙️ Miscellaneous Tasks

- *(move-syn)* [**breaking**] Update attribute parsing - ([25f5b6a](https://github.com/0xangelo/moverox/commit/25f5b6a60282c968ae0dbd5af51a996b9ed10ded))


## [0.0.4](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.3...move-syn-v0.0.4)

### ⛰️ Features

- *(move-syn)* `Attribute::contents` - ([e3b78e0](https://github.com/0xangelo/moverox/commit/e3b78e03fb95b9396697f7ddeb0767fc74068153))


## [0.0.3](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.2...move-syn-v0.0.3)

### 🐛 Bug Fixes

- *(move-syn)* Handle `Self` imports ([#15](https://github.com/0xangelo/moverox/pull/15)) - ([6b10504](https://github.com/0xangelo/moverox/commit/6b10504008a7f5c430552665a347a43cb8b40e7d))

### 🚜 Refactor

- *(move-syn)* [**breaking**] Fix typo in field name ([#14](https://github.com/0xangelo/moverox/pull/14)) - ([404d385](https://github.com/0xangelo/moverox/commit/404d3852031df77601a2ac8a764bfd8e6374d790))
- *(move-syn)* Make `named_address::module as alias::...` unrepresentable - ([a3c945c](https://github.com/0xangelo/moverox/commit/a3c945c36941fa3d0cba9cd5f6709d594e4999e0))


## [0.0.2](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.1...move-syn-v0.0.2)

### 📚 Documentation

- Tell docs.rs to build with all features and untable options - ([4e6596d](https://github.com/0xangelo/moverox/commit/4e6596d5e830a3d07fa0649b5da46726231718b1))


## [0.0.1](https://github.com/0xangelo/moverox/compare/move-syn-v0.0.0...move-syn-v0.0.1)

### ⛰️ Features

- *(move-syn)* Fully-qualify all datatype field types - ([cb07aa0](https://github.com/0xangelo/moverox/commit/cb07aa094019912565f8935422168984c57026aa))
- *(move-syn)* Parse Move enums - ([5dd3266](https://github.com/0xangelo/moverox/commit/5dd3266a99de415f577b8b337e134e32a18d321b))

### 🐛 Bug Fixes

- *(move-syn)* Missing const for methods - ([fc86232](https://github.com/0xangelo/moverox/commit/fc862320dfaae2849f1136a38e7a46a81ee8da7b))
- *(move-syn)* Consider imports shadowed by generics - ([293876b](https://github.com/0xangelo/moverox/commit/293876bf002363e423f716702bfba55832bfe689))

### 🚜 Refactor

- *(move-syn)* Organize types - ([27021ab](https://github.com/0xangelo/moverox/commit/27021abf7b3160a8171b48ff96d45c119ea73edc))
- *(move-syn)* Re-use field parsing between structs and enums - ([2b88810](https://github.com/0xangelo/moverox/commit/2b8881059348959400cefac491639cd5ec8e82b0))

### 🎨 Styling

- Cargo fmt - ([f6098e8](https://github.com/0xangelo/moverox/commit/f6098e863b8068c1a9328c424c8f861ac121b926))

### 🧪 Testing

- *(crates)* Snapshot public API - ([45a3b8e](https://github.com/0xangelo/moverox/commit/45a3b8e11ce76e14498965af61e457a1b80663fb))

### ⚙️ Miscellaneous Tasks

- Init workspace - ([f5f4804](https://github.com/0xangelo/moverox/commit/f5f4804fe2dde0a7ab6e00fc3227d7fcd33a44e5))

