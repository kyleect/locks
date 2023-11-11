<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# 🔒🔓 locks

A toy language branched from [Lox](https://www.craftinginterpreters.com/) to learn language implementation. Forked from [loxcraft](https://github.com/ajeetdsouza/loxcraft)

[![ci](https://github.com/kyleect/locks/actions/workflows/ci.yml/badge.svg)](https://github.com/kyleect/locks/actions/workflows/ci.yml)

</div>

## Features

- Bytecode Compiler
- Stack Based Virtual Machine
- Garbage Collection
- [Online Playground](https://kyleect.github.io/locks/), via WebAssembly (/w shareable [links](https://kyleect.github.io/locks/#/code=GYOwBAFgpgNjD2AKEBDAtlAlGA3gWACgwwAnKAFwFcTwAiACVgTFrAGoxUMBuQgX0KEADiQCWIcpCZJaAdXgkYAE1qZeBIA))
- Language Server Protocol
- REPL
- [VS Code Extension](#vs-code-extension)

Future [goals](https://github.com/kyleect/locks/issues/1)

## Getting Started

Check out [online playground](https://kyleect.github.io/locks/)) & [interactive Documentation](https://kyleect.github.io/locks/#/docs)

## Installing

### Runtime

Download the binary and vs code extension from the [latest build](https://github.com/kyleect/locks/actions/workflows/ci.yml)

```shell
$ locks repl
```

You can also run files

```shell
$ locks run file.locks
```

Execute from a string argument

```shell
$ locks exec 'print "Hello";' # Hello
```

Or from stdin

```shell
$ cat res/benchmarks/fib.locks | locks exec
```

Or run the language server

```shell
$ locks lsp
```

### VS Code Extension

1. Run `just build-all`
2. Copy `./target/release/locks[.exe]` somewhere in your `PATH`
3. Install `./vsc/out/locks-language-1.0.0.vsix` in VS Code
4. Create a new file and save as `*.locks`

#### Features

- Language Server support
- Syntax highlighting
- Snippets

## Development

### Setup

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://www.docker.com/)
- `$ cargo install just` Required for running development scripts
- `$ cargo install wasm-pack` Required to compile wasm package for playground

### Scripts

- `$ just build-all` Build all packages (locks, playground, & vs code extension)
- `$ just lint-all` Run linting on packages
- `$ just clean-all` Clean build artifacts in all packages
- `$ just run-playground` Build and run playground
- `$ just build-docker` Build docker image
- `$ just run-repl-docker` Run `locks repl` inside built docker image
