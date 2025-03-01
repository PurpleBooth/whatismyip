# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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