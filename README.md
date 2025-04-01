# Rust Starter Workspace
A simple reusable workspace configured how I like it.

All cargo crates are inside `/crates`.
- Output binary applications are in `/crates/bins`.
- Shared libraries are in `/crates/libs`.
- Dev tools are are in `/crates/dev`, primarily our [xtasks](https://github.com/matklad/cargo-xtask).

A "demo" binary and library are included in the crates,
partially for demonstration purposes,
but honestly it's mainly because of a weird edge case with the workspace `/Cargo.toml`.
If we don't have at least one crate in `/crates/bins`,
then `"crates/bins/*"` in the workspace config causes an error.
Same goes for `/crates/libs`.

# xtask
xtask is a design pattern I'm exploring at the moment.
It's pretty popular and is intended as a way to create scripts for working with your project, such as CICD.
It's essentially just another binary package that is intended to be used BY the developer,
and is wrapped by nicely into `cargo` with an alias in `/.cargo/config.toml`.

## Thoughts and Ramblings on my experience with xtasks so far
The primary benefit is to remove dependencies for the project itself.
i.e. you don't need to have a specific build system installed such as make or scons,
or even access to a particular scripting language such as Python or Bash or Powershell.
Only dependency is `cargo`, which is reasonable for a Rust project.
This in theory improves the cross platform developer experience as the project can bootstrap its dependencies.
Which also applies if the project is itself a dependency in a larger project,
it allows the larger project to not need to worry about installing dependencies for us.

Another benefit is that you get the usual benefits of Rust applied at meta level to your project.
"Correctness", predictability, error handling, etc.
It ALSO means that you get the usual drawbacks of Rust,
such as long compilation times and a less than ergonomic scripting experience.
Libraries like clap, xshell, and anyhow makes scripting easier, but it's still not perfect.

Generally, you want to keep your xtask small and with few dependencies to minimize compilation time.
It's a poor developer experience to wait on your scripting to compile,
and it's also something your CICD will burn cloud minutes on.
I, however, don't care about keeping it small for my personal projects.

## Usage
An alias to run it through `cargo` is provided, so all you need to type is `cargo x` to run your xtask.

See `cargo x --help` for more information.

### install-dev-tools
`cargo x install-dev-tools`

Installs necessary dev tools (of a specific, pinned version) locally into our `/.deps` folder.
You can add `/.deps/bin` to your path to use them directly.
Our other xtasks do that internally so you don't need to worry about modifying your path for the project.

Currently, the tools installed are:
- `llvm-cov` which is a `cargo` plugin for code coverage of unit tests
- `taplo` which provides autoformatting and checks for Toml files

### check
`cargo x check`

Probably the most important command here, as it runs the necessary checks for CICD.

### fmt
`cargo x fmt`

Runs our configured autoformatters.


# Explanation of lint levels
TODO: Write longer explanation about why certain lint levels were chosen.

# TODO
- Explore github actions for:
    - Testing
    - Release packages
- Explore additional cargo plugins for:
    - Unused dependencies (`cargo-udeps`)
    - Validating dependency licences (`cargo-deny`)

test: update something to re-run CI
