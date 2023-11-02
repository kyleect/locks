<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# 🔒🔓 locks

A toy language branched from [Lox](https://www.craftinginterpreters.com/) to learn language implementation. Forked from [loxcraft](https://github.com/ajeetdsouza/loxcraft)

[![ci](https://github.com/kyleect/locks/actions/workflows/ci.yml/badge.svg)](https://github.com/kyleect/locks/actions/workflows/ci.yml)

</div>

## Docker

### Build

```shell
$ docker build -t kyleect/locks:1.0.0 .
```

### Repl

```shell
$ docker run --rm -it kyleect/locks:1.0.0 locks repl
```

## VS Code Extension

- Syntax highlighting
- Error reporting
- Snippets

### Configuration

Set `locks.binPath` to the absolute path of the `locks` binary. Defaults to `target/release/locks.exe`.

## Development

### Setup

- `$ cargo install just` Required for running development scripts
- `$ cargo install wasm-pack` Required to compile wasm package for playground

### Scripts

- `$ just build-all` Build all packages (locks, playground, & vs code extension)
- `$ just lint-all` Run linting on packages
- `$ just clean-all` Clean build artifacts in all packages
- `$ just run-playground` Build and run playground
