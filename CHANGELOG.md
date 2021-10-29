# Changelog

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
