# lifeterm

A terminal renderer for [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), where cells evolve over successive generations according to simple rules.

![lifeterm.gif](./examples/lifeterm.gif)

## Installation

Once published on crates.io, install it with:

```sh
cargo install lifeterm
```

Then start it from your terminal:

```sh
lifeterm
```

## Usage

Press `Enter` on the startup panel to begin. On the board, press `h` for help, `s` for settings, or `q` / `Esc` to quit. The simulation keeps running while menus are open. Use the arrow keys to navigate or adjust values, `Enter` to select, and `q`, `Esc`, or `Backspace` to go back.

The settings menu offers:

- **FPS**: adjust the simulation speed.
- **Theme**: change the colors.
- **Pattern**: restart the board with a different seed.
- **Rule**: select birth and survival rules.
- **Auto**: add random patterns when the board becomes sparse or stagnant, helping keep the simulation alive.
- **Shadow**: show an afterglow of previously live cells.
- TODO: **min-density**: minimum density after which a new pattern can be added (in auto mode).

Patterns include selections from the [Game of Life Lab](https://www.gameoflifelab.com/#lab), where you can find more information and examples.

## Development

Install [prek](https://prek.j178.dev/) and the Rust `rustfmt` and `clippy` components, then enable the pre-commit hooks:

```sh
prek install
```

When Rust files in `src/` are staged, the hooks run `make fmt-check` and `make lint`. These commands check the project as a whole; formatting is not changed automatically. Run `make fmt` to fix formatting, or run both hooks manually with:

```sh
prek run --all-files
```

GitHub Actions runs `make fmt-check`, `make lint`, and `make test` as independent
checks on every push and pull request. Formatting and linting do not modify files;
formatting differences, lint warnings, and test failures cause their checks to fail.
Run the same commands locally before pushing.

### Documentation

`lifeterm` is a terminal application, not a public Rust library. Generate its
internal documentation with:

```sh
cargo doc --no-deps --document-private-items --open
```

For publishing and demo GIF generation, see the [release guide](RELEASE.md).
