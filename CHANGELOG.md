# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.14.3 - 2025-06-13
#### Bug Fixes
- **(deps)** update rust crate tokio to v1.45.1 - (6ce91e3) - Solace System Renovate Fox
- **(deps)** update rust crate clap to v4.5.40 - (daadf19) - Solace System Renovate Fox
- **(deps)** update rust:alpine docker digest to 126df0f - (fbc2535) - Solace System Renovate Fox
- **(deps)** update goreleaser/nfpm docker digest to 929e105 - (f19e675) - Solace System Renovate Fox
#### Miscellaneous Chores
- **(deps)** update https://code.forgejo.org/docker/bake-action digest to 37816e7 - (23417ca) - Solace System Renovate Fox
- **(deps)** update rust crate criterion to v0.6.0 - (8ca8552) - Solace System Renovate Fox

- - -

## v0.14.2 - 2025-05-16
#### Bug Fixes
- add sudo to chmod command in GitHub CLI installation script - (970644e) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update https://code.forgejo.org/docker/bake-action digest to 212c367 - (7ce234a) - Solace System Renovate Fox
- update Renovate config to use specific config preset - (b51e0c0) - Billie Thompson

- - -

## v0.14.1 - 2025-05-15
#### Bug Fixes
- add sudo to GitHub CLI installation commands in pipeline workflow - (a877fa0) - Billie Thompson

- - -

## v0.14.0 - 2025-05-15
#### Bug Fixes
- **(deps)** update rust crate local-ip-address to v0.6.5 - (242da48) - Solace System Renovate Fox
- **(deps)** pin dependencies - (cc54bc4) - Solace System Renovate Fox
- **(deps)** update rust crate local-ip-address to v0.6.4 - (e705eb9) - Solace System Renovate Fox
- import miette macro in tests module - (bc7cb84) - Billie Thompson (aider)
- update integration tests to handle Result return type - (0baaf85) - Billie Thompson (aider)
- modify integration test to handle shared IPv6 addresses - (a842bdc) - Billie Thompson (aider)
- correct WAN IP lookup strategy selection in process_args - (b494365) - Billie Thompson (aider)
- remove invalid depends fields from nfpm.yaml configuration - (923f0f0) - Billie Thompson (aider)
- Mark unused error variable as intentionally unused - (a2dc076) - Billie Thompson (aider)
- Add retry logic for DNS, fix benchmark path, remove CLI conflicts - (bebef01) - Billie Thompson (aider)
- Hide env values in CLI help output - (12019fc) - Billie Thompson (aider)
- replace deprecated `ArgEnum` with `ValueEnum` in clap import - (7d9a0b9) - Billie Thompson (aider)
- update clap env attribute syntax in cli.rs - (c8b2ab7) - Billie Thompson (aider)
- update clap syntax for env vars and remove unused import - (18e6b4f) - Billie Thompson (aider)
- correct clap env attribute syntax in CLI args - (9f4988d) - Billie Thompson (aider)
- Add backticks to `ip_address` in doc comment - (1c5e8ce) - Billie Thompson (aider)
- enable tokio sync feature and update OnceCell usage - (f0f8111) - Billie Thompson (aider)
- correct hickory-resolver feature from tokio-runtime to tokio - (7b93e81) - Billie Thompson (aider)
- shave 10ms from the time - (36c97eb) - Billie Thompson
- add some more tests - (747875a) - Billie Thompson
#### Build system
- Add unicase and unicode-width dependencies to clap - (bfbb3b5) - Billie Thompson
- optimize Docker build context and Rust dependencies - (04642c6) - Billie Thompson (aider)
#### Continuous Integration
- add sudo to apt commands in pipeline - (2fb35ff) - Billie Thompson
#### Documentation
- **(readme)** enhance usage and installation instructions - (6e62326) - Billie Thompson
- add notes about CI and Docker build issues - (e4c3438) - Billie Thompson
#### Features
- **(cli)** expand IP handling and add tests - (12c39a1) - Billie Thompson
- replace `io::Error` with `miette` in integration tests - (8e87b79) - Billie Thompson (aider)
- add environment variable support for CLI arguments - (8d0c4ef) - Billie Thompson (aider)
#### Miscellaneous Chores
- **(deps)** pin dependencies - (9af70e2) - Solace System Renovate Fox
- **(deps)** update dependencies in Cargo.lock - (3e3ed3e) - Billie Thompson
- **(deps)** update https://code.forgejo.org/actions/cache digest to 5a3ec84 - (6bd91cc) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/docker/bake-action digest to 76f9fa3 - (86e5122) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/docker/login-action digest to 74a5d14 - (b7c43ec) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/actions/cache digest to d4323d4 - (8680cef) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/docker/setup-buildx-action digest to b5ca514 - (a0c4ec1) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/docker/metadata-action digest to 902fa8e - (2ed09cd) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/docker/bake-action digest to 4ba453f - (206c0ca) - Solace System Renovate Fox
- refactor tests to use Result and miette error handling - (3f947bb) - Billie Thompson (aider)
- Update dependencies and refactor mock_reverse_ip function - (edd36f8) - Billie Thompson
- update tokio and windows-sys dependencies - (0e104ba) - Billie Thompson
- Remove unused clap::ValueEnum import - (7c37d1a) - Billie Thompson (aider)
- Update .gitignore and remove unused dependencies in Cargo.lock - (db2bd36) - Billie Thompson
#### Refactoring
- simplify string comparison in test cases using dereference - (926c119) - Billie Thompson
- Replace len() != 0 checks with is_empty() and is_err() comparisons - (e13c3a8) - Billie Thompson
- replace panics with Result and miette error handling in tests - (003572c) - Billie Thompson (aider)
- replace panics with Result and miette error handling in tests - (ecdcf15) - Billie Thompson (aider)
- remove panics from integration tests, use Result return type - (567c8ac) - Billie Thompson (aider)
- replace ping commands with TCP connection checks in integration tests - (636d39b) - Billie Thompson (aider)
- Improve IP address strategy selection logic in CLI arguments - (94942b1) - Billie Thompson
- optimize Dockerfiles with cargo-chef for cross-compilation - (b8aa0a2) - Billie Thompson (aider)
- simplify CLI argument attribute definitions - (76429a7) - Billie Thompson
- optimize nfpm.yaml with better metadata and dependencies - (9c4d398) - Billie Thompson (aider)
#### Style
- format test code in myip.rs - (e867504) - Billie Thompson (aider)
- format error message with multi-line macro call - (5ffba30) - Billie Thompson (aider)
- format code to remove unnecessary line breaks - (e3abdec) - Billie Thompson (aider)
- remove extra blank lines in lib.rs - (738df3e) - Billie Thompson (aider)
- format integration test file with cargo fmt - (b0a6c0f) - Billie Thompson (aider)
- remove trailing whitespaces in integration tests - (2f4e3f2) - Billie Thompson
- format integration test file for consistent code style - (95a49e2) - Billie Thompson (aider)
- format println! macro in integration test - (7ffb55b) - Billie Thompson (aider)
- format intersection declarations in integration tests - (23b9f7e) - Billie Thompson (aider)
- remove extra whitespace in integration test file - (5bb7474) - Billie Thompson (aider)
- remove extra whitespace lines in integration tests - (aa02924) - Billie Thompson (aider)
- Remove trailing spaces in clap attributes - (c974924) - Billie Thompson (aider)
- Format cli.rs according to Rustfmt standards - (4b5b627) - Billie Thompson (aider)
- fix import ordering and formatting in src/lib.rs - (4a102e7) - Billie Thompson (aider)
#### Tests
- add connectivity checks for IPv4 and IPv6 tests - (c0ac291) - Billie Thompson (aider)
- remove test comparing local and WAN IP outputs - (1345682) - Billie Thompson
- check local and WAN outputs have empty IP intersection - (5e713c0) - Billie Thompson (aider)
- refactor local and wan ip comparison tests to handle ordering - (b27252d) - Billie Thompson (aider)

- - -

## v0.13.13 - 2025-02-25
#### Bug Fixes
- **(ci)** update workflow runners to runner-latest - (d23404a) - Billie Thompson
#### Continuous Integration
- add sudo to apt commands in pipeline - (96b0dfb) - Billie Thompson
- unify YAML indentation in pipelines - (dbb9142) - Billie Thompson
- remove crates publish - (bd7a5e0) - PurpleBooth
#### Miscellaneous Chores
- **(deps)** update https://code.forgejo.org/docker/bake-action digest to 4f08b22 - (9d24a12) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/actions/cache digest to 0c907a7 - (f5fc87e) - Solace System Renovate Fox

- - -

## v0.13.12 - 2025-02-11
#### Bug Fixes
- rename project to "whatismyip" in Dockerfile - (7a79532) - Billie Thompson
- Update dependencies to latest versions - (98e8750) - Billie Thompson
- migrate to codeberg - (4b1b612) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust:alpine docker digest to 9ab8f4e (#236) - (a74f819) - renovate[bot]
- remove yamlfmt command from Justfile - (74acb4e) - Billie Thompson
#### Refactoring
- simplify and adjust linting configuration - (0d099e3) - Billie Thompson

- - -

## v0.13.11 - 2024-12-06
#### Bug Fixes
- **(deps)** update rust crate clap to v4.5.23 (#235) - (7178d9f) - renovate[bot]

- - -

## v0.13.10 - 2024-12-04
#### Bug Fixes
- **(deps)** update rust crate miette to v7.4.0 - (a57e95f) - renovate[bot]
- **(deps)** update rust crate tokio to v1.42.0 - (fed28e9) - renovate[bot]
- **(deps)** update rust crate local-ip-address to v0.6.3 - (e9ad1cc) - renovate[bot]

- - -

## v0.13.9 - 2024-12-04
#### Bug Fixes
- **(deps)** update rust crate futures to v0.3.31 (#231) - (63408e5) - renovate[bot]

- - -

## v0.13.8 - 2024-12-04
#### Bug Fixes
- **(deps)** update rust crate clap to v4.5.22 (#230) - (1ecb127) - renovate[bot]
#### Miscellaneous Chores
- **(deps)** update rust:alpine docker digest to 838d384 - (74d8d68) - renovate[bot]
- **(deps)** update goreleaser/nfpm docker digest to ae35b40 - (85c6198) - renovate[bot]
- **(deps)** update rust:alpine docker digest to 2f42ce0 - (30595b0) - renovate[bot]
- **(deps)** update rust:alpine docker digest to 00c2107 - (d30a773) - renovate[bot]

- - -

## v0.13.7 - 2024-11-02
#### Bug Fixes
- Add secrets for packaging - (093a051) - Billie Thompson

- - -

## v0.13.6 - 2024-11-01
#### Bug Fixes
- release with packages - (d63ed9f) - Billie Thompson

- - -

## v0.13.5 - 2024-11-01
#### Bug Fixes
- Switch to full docker bake pipeline - (58bb492) - Billie Thompson
#### Continuous Integration
- Kake the directory correct for bins - (e5e1a41) - Billie Thompson

- - -

## v0.13.4 - 2024-10-31
#### Bug Fixes
- try a release with packages - (71f0320) - Billie Thompson
#### Continuous Integration
- try docker bake - (294f484) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust:alpine docker digest to 466dc99 - (6c604a8) - renovate[bot]
- **(deps)** update rust:alpine docker digest to d6e876c - (dd8fa5f) - renovate[bot]

- - -

## v0.13.3 - 2024-08-31
#### Bug Fixes
- **(deps)** update rust crate tokio to 1.40.0 - (d36ddbf) - renovate[bot]
#### Miscellaneous Chores
- **(deps)** pin dependencies - (b36f9db) - renovate[bot]
#### Refactoring
- Cross compile from docker - (5d3c855) - Billie Thompson

- - -

## v0.13.2 - 2024-08-29
#### Bug Fixes
- test deploy - (1607011) - Billie Thompson

- - -

## v0.13.1 - 2024-08-29
#### Bug Fixes
- test deploy - (19e55c7) - Billie Thompson

- - -

## v0.13.0 - 2024-08-29
#### Continuous Integration
- Add contents read permission to docker - (6e2d7f4) - Billie Thompson
#### Features
- Enable packages - (5f67cce) - Billie Thompson

- - -

## v0.12.2 - 2024-08-25
#### Bug Fixes
- **(deps)** update rust crate tokio to 1.39.3 - (aeb27d5) - renovate[bot]

- - -

## v0.12.1 - 2024-08-24
#### Bug Fixes
- **(deps)** update rust crate clap to 4.5.16 - (eb3ef8d) - renovate[bot]
#### Build system
- Ensure we are copying the correct binary for docker container - (1bf56e8) - Billie Thompson
- Ensure we don't cache the "Hello world" app - (d75d96a) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin rust docker tag to 29fe437 - (eb0f6cf) - renovate[bot]

- - -

## v0.12.0 - 2024-08-22
#### Build system
- Correctly set each target - (3c73202) - Billie Thompson
#### Features
- Add docker distribution - (0c4b8b2) - Billie Thompson
#### Miscellaneous Chores
- Unused file - (b0da6c0) - Billie Thompson
#### Refactoring
- Correct formatting - (37d3e7a) - Billie Thompson
- use shorter namespace - (db3e421) - Billie Thompson
- Deduplicate the strategy logic - (2916998) - Billie Thompson

- - -

## v0.11.7 - 2024-08-21
#### Bug Fixes
- Move to new name of trust - (c79c003) - Billie Thompson
#### Continuous Integration
- Remove unused file - (c987f87) - Billie Thompson
#### Miscellaneous Chores
- Formatting - (41ad185) - Billie Thompson

- - -

## v0.11.6 - 2024-08-19
#### Bug Fixes
- **(deps)** update rust crate tokio to v1.39.3 - (9822dc2) - renovate[bot]

- - -

## v0.11.5 - 2024-08-16
#### Bug Fixes
- **(deps)** bump clap from 4.5.15 to 4.5.16 - (de41a27) - dependabot[bot]
#### Build system
- Update the description in the homebrew repo - (4ddcaee) - Billie Thompson

- - -

## v0.11.4 - 2024-08-11
#### Bug Fixes
- **(deps)** update rust crate clap to v4.5.15 - (fbb093a) - renovate[bot]

- - -

## v0.11.3 - 2024-08-08
#### Bug Fixes
- **(deps)** bump clap from 4.5.13 to 4.5.14 - (c57968d) - dependabot[bot]

- - -

## v0.11.2 - 2024-08-03
#### Bug Fixes
- bump deps - (42701b7) - Billie Thompson
#### Continuous Integration
- format - (48094e1) - Billie Thompson

- - -

## v0.11.1 - 2024-08-03
#### Bug Fixes
- **(deps)** update rust crate clap to v4.5.13 - (e4d63fe) - renovate[bot]

- - -

## v0.11.0 - 2024-08-03
#### Bug Fixes
- Add reasons to why we allowing some lints - (3af16ed) - Billie Thompson
- Bump versions - (49e072d) - Billie Thompson
#### Build system
- Disable warning that is only in stable - (4082a20) - Billie Thompson
#### Continuous Integration
- use binstall for faster releases - (0d719d4) - Billie Thompson
- do if in other steps - (ccfdea8) - Billie Thompson
- tidy lines - (3252feb) - Billie Thompson
- Add renovate.json - (6820efe) - renovate[bot]
- Switch to cog - (3fdb2b6) - Billie Thompson
#### Documentation
- format readme - (bca5d12) - Billie Thompson
#### Features
- Also list local IP addresses - (69f3d8a) - Billie Thompson
#### Refactoring
- Tidy wan ip switch - (d5b1fc7) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).