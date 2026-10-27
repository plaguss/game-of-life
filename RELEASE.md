# Release guide

Run the following commands from the repository root.

## Publishing

Before each release, update the version in `Cargo.toml`, run `cargo check` to
update `Cargo.lock`, and commit the release changes.

Authenticate with a crates.io API token (only needed once):

```sh
cargo login
```

Verify the package without uploading it, then publish:

```sh
cargo publish --dry-run --locked
cargo publish --locked
```

## Demo GIF

The demo shows the startup banner and help, changes every on-screen setting,
then runs Random Soup with Auto and Shadow enabled for three seconds before
quitting.

Install [VHS](https://github.com/charmbracelet/vhs), then regenerate the GIF:

```sh
cargo build --locked
vhs examples/demo.tape
```

The recording is defined in [examples/demo.tape](examples/demo.tape) and writes
[examples/lifeterm.gif](examples/lifeterm.gif), which is displayed in the README.
