# Garden Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/garden)](https://pkg.fluentci.io/garden)
[![ci](https://github.com/fluentci-io/garden-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/garden-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [garden](https://garden.io/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm garden setup
```

## Functions

| Name     | Description                                |
| -------- | ------------------------------------------ |
| setup    | Installs a specific version of garden.     |
| build    | Perform you Builds.                        |
| cleanup  | Cleanup resources.                         |
| deploy   | Deploy actions to your environment.        |
| get      | Retrieve and output data and objects, e.g. secrets, status info etc. |
| publish  | Build and publish artifacts (e.g. container images) to a remote registry.  |
| run      | Perform one or more Run actions            |
| test     | Run all or specified Test actions in the project. |
| validate | Check your garden configuration for errors. |
| workflow | Run a Workflow.                            |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/garden@v0.1.0?wasm=1", "setup", vec!["0.13.35"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: garden
    args: |
      setup
- name: Show garden version
  run: |
    type garden
    garden version
```
