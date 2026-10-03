# ferris_wheels

Shared programming project for members of the Leeds Rust meetup. A place to build something, learn from other's code, and have fun in a small Rust codebase built on the [Bevy](https://bevyengine.org/) games engine.

## How it works

- **Pick an issue and assign yourself.** Find an issue you want to work on and attach your name to it.
- **Work on `main`.** You don't need to open a pull request to get your work in. Commit and push straight to `main`. You can work on a feature branch if you really want but we want to avoid long lived feature branches and merge conflicts.
- **Keep `main` building.** CI runs `cargo fmt --check` and `cargo clippy -- -D warnings` on every push and pull request. If you do break the build because you have not finished please leave a message in your commit.

If you're new to Rust and or Bevy thats fine. This project is for learning together.

## Getting started

You need a recent stable Rust toolchain

Build:

```sh
cargo build
```

Run:

```sh
cargo run
```

Before you push, check formatting and lints:

```sh
cargo fmt
cargo clippy -- -D warnings
```

### Linux

Bevy needs some system libraries on Linux. The CI workflow installs them in [`.github/actions/install-linux-deps`](.github/actions/install-linux-deps/action.yml). That will have the package list for your distro.

## License

Released into the public domain under the [Unlicense](LICENSE).
