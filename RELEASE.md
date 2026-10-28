# Release guide

Run the following commands from the repository root.

## Publishing

Before each release, choose a [Semantic Versioning](https://semver.org/) version and update `Cargo.toml`. Edit `Cargo.toml` directly, or install `cargo-edit` and use `cargo set-version`:

```sh
cargo install cargo-edit --locked
cargo set-version 0.1.1
```

Run `cargo check` to update `Cargo.lock`, review the changes, and commit them.

Authenticate with a crates.io API token (only needed once):

```sh
cargo login
```

Install [VHS](https://github.com/charmbracelet/vhs) and ensure the working tree is clean. This command checks formatting, lint and tests, builds the release binary, regenerates the demo GIF, verifies the package and uploads it to crates.io:

```sh
make release
```

This publishes immediately after the dry run succeeds. The generated GIF is excluded from the crate to stay below crates.io's upload size limit. Review and commit the GIF afterward. Published versions cannot be reused, even if yanked.

## Demo GIF

The demo shows the startup banner and help, changes every on-screen setting,
then runs Random Soup with Auto and Shadow enabled for three seconds before
quitting.

Install [VHS](https://github.com/charmbracelet/vhs), then regenerate the GIF:

```sh
cargo build --release --locked
vhs examples/demo.tape
```

The recording is defined in [examples/demo.tape](examples/demo.tape) and writes
[examples/lifeterm.gif](examples/lifeterm.gif), which is displayed in the README.
