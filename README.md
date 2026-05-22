# Pipery Rust CI

CI pipeline for Rust: SAST, SCA, lint, build, test, versioning, packaging, release, reintegration

## Status

- Owner: `pipery-dev`
- Repository: `pipery-rust-ci`
- Marketplace category: `continuous-integration`
- Current version: `1.0.2`

## Usage

```yaml
name: Example
on: [push]

jobs:
  run-action:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pipery-dev/pipery-rust-ci@v1
        with:
          project_path: .
          config_file: .pipery/config.yaml
          rust_toolchain: stable
          tests_path: 
          features: 
          target: 
          github_token: 
          version_bump: patch
          log_file: pipery.jsonl
          target_branch: main
          crates_token: 
          skip_sast: false
          skip_sca: false
          skip_lint: false
          skip_build: false
          skip_test: false
          skip_versioning: false
          skip_packaging: false
          skip_release: false
          skip_reintegration: false
```

## Inputs

| Name | Required | Default | Description |
| --- | --- | --- | --- |
| `project_path` | no | `.` | Path to the project source tree the action should operate on. |
| `config_file` | no | `.pipery/config.yaml` | Path to Pipery config file. |
| `rust_toolchain` | no | `stable` | Rust toolchain to use (e.g. stable, nightly, 1.75). |
| `tests_path` | no | `` | Test filter pattern passed to cargo test. |
| `features` | no | `` | Cargo features to enable (comma-separated). |
| `target` | no | `` | Cargo target triple (e.g. x86_64-unknown-linux-musl). |
| `github_token` | no | `` | GitHub token for release and reintegration steps. |
| `version_bump` | no | `patch` | Version bump type: patch, minor, or major. |
| `log_file` | no | `pipery.jsonl` | Path to the JSONL log file written during the run. |
| `target_branch` | no | `main` | Target branch for reintegration. |
| `crates_token` | no | `` | crates.io API token for publishing. |
| `skip_sast` | no | `false` | Skip SAST step. |
| `skip_sca` | no | `false` | Skip SCA step. |
| `skip_lint` | no | `false` | Skip lint step. |
| `skip_build` | no | `false` | Skip build step. |
| `skip_test` | no | `false` | Skip test step. |
| `skip_versioning` | no | `false` | Skip versioning step. |
| `skip_packaging` | no | `false` | Skip packaging step. |
| `skip_release` | no | `false` | Skip release step. |
| `skip_reintegration` | no | `false` | Skip reintegration step. |

## Outputs

No outputs.

## Development

This repository is managed with `pipery-tooling`.

```bash
pipery-actions test --repo .
pipery-actions docs --repo .
pipery-actions release --repo . --dry-run
```

By default, `pipery-actions test --repo .` executes the action against `test-project` and validates `pipery.jsonl`.

## Marketplace Release Flow

1. Update the implementation and changelog.
2. Run `pipery-actions release --repo .`.
3. Push the created git tag and major tag alias.
4. Publish the GitHub release.
