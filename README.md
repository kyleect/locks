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

## Syntax Examples

[Variables](https://kyleect.github.io/locks/#/code=G4QwTgBKA2CuCmEC8ECMAmAzAbgLACgAHMASwDsAXKEOebCAegYgHtYKAuNLIA)

```
var value = 123;
print value; // out: 123
```

[Functions](https://kyleect.github.io/locks/#/code=GYOwBAzgrgtmAUBDANGARgSjAbwLACgwwAnAUwBcpjxEwBqdAbgIF8CCAHYgSxHMljwAbAAZUAFhEZGYAPSywAeyjkAXGACMIkUA)

```
fn sum (a, b) {
  return a + b;
}

print sum(60, 40); // out: 100
```

[String Concatenation](https://kyleect.github.io/locks/#/code=GYOwBAFgpgNjD2AKEBDAtlAlGA3gWACgwwAnKAFwFcTwAiACVgTFrAGoxUMBuQgX0KEADiQCWIcpCZJaAdXgkYAE1qZuQA)

```
fn hello(name) {
  return "Hello " + name;
}

print hello("World"); // out: Hello
```

[Closures](https://kyleect.github.io/locks/#/code=GYOwBA5gTgpjAuAKad4EsQQJRgN4FgAoMMUMABxigGcB7EREAQwFsYcDiSxZ4BXKOBQIMEMAGowAImkSwzNgG4iJAL4qwG3gPCUa9ZYXWEi5KBniRYCRFIASMADaPaUrLYDqtKI4AmbxTAAeiCwWj54AC4wB2daMC8fXyA)

```
fn greet(greeting) {
  fn person(name) {
    return greeting + " " + name;
  }

  return person;
}

print greet("Hello")("World"); // out: Hello World
```

[For Loops](https://kyleect.github.io/locks/#/code=GYewTgBAFAbghpAlhAvBADAbgsgPBARix1RIGpCBKCAbwFgAoCCABzEQDsAXHTRgX0ZA)

```
for (var i = 0; i < 10; i = i + 1) {
  print i;
}

// out: 0
// out: 1
// out: 2
// out: 3
// out: 4
// out: 5
// out: 6
// out: 7
// out: 8
// out: 9
```

[If/Else](https://kyleect.github.io/locks/#/code=G4QwTgBAlgzgKmArgUwgXggFycg3AWACgioAzCAClgRQEoIBvIiCABzCgDtMIAiAdRAwsOAIS8ChAL4RkAGxiomhFuy48BQiKRALk4yVKJEA9CYgB7RJgBcEQcOwpRQA)

```
var isTrue = true;

if (isTrue) {
  print "Was true!";
} else {
  print "Was false!";
}

// out: Was true!
```

[Classes](https://kyleect.github.io/locks/#/code=MYGwhgzhAEDiBOBTRAXR9oG8CwAoa0AlgHaEoAUA5kqiZQJRZ4EEoAWhEAdNcindAC80XrWKUA3M2gBfPNNEViYALaJGOfCyQoArvGLR2nHjX7joAamgAiW1ejK1UrXNxu8ANzAZF6IXBm6OQ2ABKIICAA9jb0LngADvAkKCJB8KZ8IQDqUfAgACaxEtAA9KXQUbooAFzQ4ZFR0Ln5BXhAA)

```
class Greeter {
  init(greeting) {
    this.greeting = greeting;
  }

  greet(name) {
    return this.greeting + " " + name;
  }
}

var greeter = Greeter("Hello");

print greeter.greet("World"); // out: Hello World
```

[Inheritance](https://kyleect.github.io/locks/#/code=MYGwhgzhAEDiBOBTRAXR9oG8CwAoa0AlgHaEoAUA5kqiZQJRZ4EEoAWhEAdNcindAC80XrWKUA3M2gBfPNNEViYALaJGOfCyQoArvGLR2nHjX7joAamgAiW1ejK1UrXNxu8oSDAASiECAA9gh86NAAPHBmYZoEJGTkGtIEELoADuhc8RQ2fgGBNvQuBG4euABuYBiKYcJ5QSGo6IkueGnwJCgi0fCmfOQ2AOqB8CAAJoUS0AD009CBuigAXND1gdDDo2N4QA)

```
class Greeter {
  init(greeting) {
    this.greeting = greeting;
  }

  greet(name) {
    return this.greeting + " " + name;
  }
}

class HelloGreeter < Greeter {
  init() {
    super.init("Hello");
  }
}

var greeter = HelloGreeter();

print greeter.greet("World"); // out: Hello World
```

## Usage

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

## VS Code Extension

- Syntax highlighting
- Error reporting
- Snippets
- Command: Run current file

### Configuration

Set `locks.binPath` to the command or absolute path to `locks`.

### Usage

1. Run `just build-all`
2. Run `cargo install`
3. Install `./vsc/out/locks-language-1.0.0.vsix` in VS Code
4. Create `*.locks` file
