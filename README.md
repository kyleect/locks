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
- [Interactive Documentation](https://kyleect.github.io/locks/#/docs)
- Language Server Protocol
- REPL
- [VS Code Extension](#vs-code-extension)
- [Docker](#docker)

Future [goals](https://github.com/kyleect/locks/issues/1)

## Getting Started

Check out the [documentation](https://kyleect.github.io/locks/#/docs) the start running code in the [playground](https://kyleect.github.io/locks/).

### Example

[Playground](https://kyleect.github.io/locks/#/?code=GYOwBMCWBe0EIFdYAoQEowG8CwAoMEA9gE5jIBuAhqZGALxgCMA3GLQDwMiu0O0DUTDDnwExkYGVoBSJgFZ6DAAzC8Y9WAAOxSCAAuYAEQAxGPCTRDzNRoC+N9QFMANgGdHbSchlgAzIrAVLAcNbV0DEzMrEIJ7USc3DwkpMFkFOmVVeNCdfSNEWGjs2JiwF3dg4rEwvMhrKriS3Di8KFgC6GRGJRV6vAB6frBCBD0ALiYBoZHxsAAmKeHRidNYRZmJgBZ15bAOndnV6AOJgHYTsAAOC6OL-dxBpdnGRhuzC8ZfD+2H6d2j+6PDZMABsH3OvyeK3ekOBjAAnHcLG81rDdnMFmjZnMvljoaigbtAX9sWC8WBbuS5tcqYjyQDkeTfK8mZjCYcYeyJr4fly9oy+ZS+b4IcKaYLOSSJsSoWBNiyJQSpXLcXzNrzlQylbLNmS1aLNZKdXS+TLgULlXI2ZbVYbtcCzbs5Hq7cdyXJxZaTa7HbMQQrlSDrbKLbKQRrZb78W6+SCDWHPSGjQ6BcrTgGk-bdqdbbLThHzWYo2BTi68-HC1nZqdvZHU5mY8rLsHgZdc5XG3WqxNLmWOxdLonW7WO8X4RngfCW-9k7t4QWifX++T4RW50OZ93+bAgA)

```
fn fizzBuzz(n) {
  for (var i = 1; i <= n; i = i + 1) {
      if (i % 15 == 0) {
        print "FizzBuzz";
      }
      else if (i % 3 == 0) {
        print "Fizz";
      }
      else if (i % 5 == 0) {
        print "Buzz";
      }
      else {
        print i;
      }
  }
}

fizzBuzz(100);
```

## Usage

### Runtime

Download the runtime from the [latest build](https://github.com/kyleect/locks/actions/workflows/ci.yml)

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

### Docker

- `$ just build-docker` Build docker image
- `$ just run-repl-docker` Run `locks repl` inside built docker image

### VS Code Extension

Download the VS Code extension from the [latest build](https://github.com/kyleect/locks/actions/workflows/ci.yml)

#### OR

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
