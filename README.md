# uda-connector
A Rust lib to access UDA (unicycling registration software)

## How to deploy a new version of the lib to crates.io?

This repository includes [a GitHub action](.github/workflows/publish.yml) that makes deployment easy.
1. Update the lib version in [Cargo.toml](Cargo.toml)
2. Merge your pull request
3. Tag with the correct version

The last step should automatically trigger a job. Once it is done, the new version should be available on [crates.io](https://crates.io/crates/uda-connector) ✨