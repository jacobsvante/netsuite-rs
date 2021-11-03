# Changelog

## [0.6.0](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.5.0...v0.6.0) (2021-11-03)


### ⚠ BREAKING CHANGES

* `suiteql` command now fetches all results, `suiteql-raw` retains previous behavior
* Fetch SuiteQL results more quickly using threads
* Read SuiteQL query from file path instead of from string
* Return CliError from CLI

### Features

* `suiteql` command now fetches all results, `suiteql-raw` retains previous behavior ([78690ca](https://www.github.com/jmagnusson/netsuite-rs/commit/78690ca398a7c08fe9384c27f5ea8fde1385a6b1))
* Fetch SuiteQL results more quickly using threads ([f4a2689](https://www.github.com/jmagnusson/netsuite-rs/commit/f4a2689baa3903c37d423e20014b05fbdb564571))
* Read SuiteQL query from file path instead of from string ([51d3445](https://www.github.com/jmagnusson/netsuite-rs/commit/51d34455d258a2fceb21d7df12ac3b4c50716ed8))


### Bug Fixes

* Include error message on IO error ([8cc4cef](https://www.github.com/jmagnusson/netsuite-rs/commit/8cc4ceff61fa3a0127642d2a7fc59bfd70a08902))
* Include serde error message ([0941a78](https://www.github.com/jmagnusson/netsuite-rs/commit/0941a7809de2d2b37234f4cbb2f95a765abc02ce))
* Make main structs Debug and Clone ([7fa92f4](https://www.github.com/jmagnusson/netsuite-rs/commit/7fa92f4e0ca3d145b8dbc4ea7f897f3bbf00a868))
* Return CliError from CLI ([5cbcbcb](https://www.github.com/jmagnusson/netsuite-rs/commit/5cbcbcb5efdb3de5519e4c90977e42b81b7c02ad))

## [0.5.0](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.4.0...v0.5.0) (2021-11-01)


### ⚠ BREAKING CHANGES

* oauth1 Token struct fields and authorize function were open to the outside world
* Harmonize `with_base_url` method with `with_algorithm`

### Features

* Allow overriding OAuth 1 algorithm ([7fa59b4](https://www.github.com/jmagnusson/netsuite-rs/commit/7fa59b4612824c0a706aff9722031e632e8250b3))


### Bug Fixes

* Harmonize `with_base_url` method with `with_algorithm` ([d95b76f](https://www.github.com/jmagnusson/netsuite-rs/commit/d95b76fcbfc959349da4fa0ab00688cbf8818bf7))
* oauth1 Token struct fields and authorize function were open to the outside world ([f0ec653](https://www.github.com/jmagnusson/netsuite-rs/commit/f0ec6532b56594c54939ae1cb0de6ef9832617c2))
* Only install `netsuite` binary if `cli` feature is enabled ([6e14ced](https://www.github.com/jmagnusson/netsuite-rs/commit/6e14ceda77a027958fa09377daad7d202c097790))

## [0.4.0](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.3.0...v0.4.0) (2021-10-30)


### ⚠ BREAKING CHANGES

* Allow Params.push to accept anything that can be turned into a String

### Features

* Add support for JSON Schema and OpenAPI + OpenAPI web schema browser ([8d97821](https://www.github.com/jmagnusson/netsuite-rs/commit/8d978213f045e2a6ad19f102489f52c9f89eda69))
* Don't include CLI dependencies by default ([6bac000](https://www.github.com/jmagnusson/netsuite-rs/commit/6bac0009a78775a141b0ece4f07ac2dede54a298))


### Bug Fixes

* Allow Params.push to accept anything that can be turned into a String ([2ebf3e0](https://www.github.com/jmagnusson/netsuite-rs/commit/2ebf3e02801bd868dc2da27d70b58d34bd74e665))

## [0.3.0](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.2.0...v0.3.0) (2021-10-29)


### ⚠ BREAKING CHANGES

* Simplify API usage by not using references for Config et al.

### Features

* Simplify API usage by not using references for Config et al. ([aa7f96d](https://www.github.com/jmagnusson/netsuite-rs/commit/aa7f96d079b65155becded72632da01b4b1d4dc3))

## [0.2.0](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.1.1...v0.2.0) (2021-10-28)


### ⚠ BREAKING CHANGES

* Move to SHA-256 algorithm for OAuth1 authentication header
* Support INI configs
* Rename CLI options to `account` and `token_id`

### Features

* Ability to set log level via CLI argument ([17a324e](https://www.github.com/jmagnusson/netsuite-rs/commit/17a324e6b2bc7b2e6bf370cb5369507ad37d8a58))
* Implement CRUD methods ([42d4038](https://www.github.com/jmagnusson/netsuite-rs/commit/42d40385077bd3c91208dbfd5acc9a3caaf6b250))
* Move to SHA-256 algorithm for OAuth1 authentication header ([5e01c54](https://www.github.com/jmagnusson/netsuite-rs/commit/5e01c549c7297018468ebc9970ae0b909a57e141))
* Support INI configs ([1ee0494](https://www.github.com/jmagnusson/netsuite-rs/commit/1ee049413f2ebc70e5f785e2fc8448a1bd73047d))


### Bug Fixes

* CLI didn't return proper library version ([10e79f4](https://www.github.com/jmagnusson/netsuite-rs/commit/10e79f49be7b2d9fb7dee1569a59a727881b56df))
* Couldn't see CLI --help because of how INI section was being checked ([ea7484f](https://www.github.com/jmagnusson/netsuite-rs/commit/ea7484f54b4ed18632325dad0a4bf2f29ffdbb95))
* INI containing unknown config key wasn't allowed ([073cf08](https://www.github.com/jmagnusson/netsuite-rs/commit/073cf0822055403be68da5f9bb2891f70343824b))
* Logging wasn't initialized before proper CLI args parse ([a3ca1f4](https://www.github.com/jmagnusson/netsuite-rs/commit/a3ca1f46f0b4ff9eb790f6c3cc0a927472afffdd))
* Rename CLI options to `account` and `token_id` ([aea4244](https://www.github.com/jmagnusson/netsuite-rs/commit/aea42448a39c37f87abe26a3892cb5edd0729f4d))
* Return error in case an unknown env var is being set ([db0d808](https://www.github.com/jmagnusson/netsuite-rs/commit/db0d80843516d80f9e8295db6ecb5ad5845d338b))

### [0.1.1](https://www.github.com/jmagnusson/netsuite-rs/compare/v0.1.0...v0.1.1) (2021-10-24)


### Bug Fixes

* Wrong repo/homepage links ([c12c7cc](https://www.github.com/jmagnusson/netsuite-rs/commit/c12c7ccd81202d055d0c2bbb9eac49b7841bd1b9))

## 0.1.0 (2021-10-24)


### Bug Fixes

* Implement clippy suggestions and format code ([4725b45](https://www.github.com/jmagnusson/netsuite-rs/commit/4725b45c56751756a3ac982bfe99be112dae6b6d))
