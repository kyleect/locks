<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# 🔓 locks

A toy language branched from [Lox](https://www.craftinginterpreters.com/) to learn language implementation. Forked from [loxcraft](https://github.com/ajeetdsouza/loxcraft).

[![ci](https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml/badge.svg)](https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml)

</div>

## Features

- Bytecode Compiler
- Stack Based Virtual Machine
- Garbage Collection
- [Online Playground](https://kyleect.github.io/locks/), via WebAssembly (/w shareable [links](https://kyleect.github.io/locks/#/code=GYOwBAFgpgNjD2AKEBDAtlAlGA3gWACgwwAnKAFwFcTwAiACVgTFrAGoxUMBuQgX0KEADiQCWIcpCZJaAdXgkYAE1qZeBIA))
- [Interactive Documentation](https://kyleect.github.io/locks/#/docs)
- Language Server
- REPL
- [VS Code Extension](#vs-code-extension)
- [Docker](#docker)

## Getting Started

Check out the [documentation](https://kyleect.github.io/locks/#/docs) the start running code in the [playground](https://kyleect.github.io/locks/).

### Example

[Playground](https://kyleect.github.io/locks/#/?code=GYOwBMCWBe0EIFdYAoQEowG8CwAoMEA9gE5jIBuAhqZGALxgCMA3GLQDwMiu0O0DUTDDnwExkYGVoBSJgFZ6DAAzC8Y9WAAOxSCAAuYAEQAxGPCTRDzNRoC+N9QFMANgGdHbSchlgAzIrAVLAcNbV0DEzMrEIJ7USc3DwkpMFkFOmVVeNCdfSNEWGjs2JiwF3dg4rEwvMhrKriS3Di8KFgC6GRGJRV6vAB6frBCBD0ALiYBoZHxsAAmKeHRidNYRZmJgBZ15bAOndnV6AOJgHYTsAAOC6OL-dxBpdnGRhuzC8ZfD+2H6d2j+6PDZMABsH3OvyeK3ekOBjAAnHcLG81rDdnMFmjZnMvljoaigbtAX9sWC8WBbuS5tcqYjyQDkeTfK8mZjCYcYeyJr4fly9oy+ZS+b4IcKaYLOSSJsSoWBNiyJQSpXLcXzNrzlQylbLNmS1aLNZKdXS+TLgULlXI2ZbVYbtcCzbs5Hq7cdyXJxZaTa7HbMQQrlSDrbKLbKQRrZb78W6+SCDWHPSGjQ6BcrTgGk-bdqdbbLThHzWYo2BTi68-HC1nZqdvZHU5mY8rLsHgZdc5XG3WqxNLmWOxdLonW7WO8X4RngfCW-9k7t4QWifX++T4RW50OZ93+bAgA)

```
fn fizzBuzz(n) {
  for (let i = 1; i <= n; i = i + 1) {
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

Download the runtime from the [latest build](https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml)

#### REPL

```shell
$ locks repl
```

#### Run files

```shell
$ locks run file.locks
```

#### Execute locks code as an argument

```shell
$ locks exec 'print "Hello";' # Hello
```

#### Execute locks code from stdin

```shell
$ cat res/benchmarks/fib.locks | locks exec
```

#### Print the Absract Syntax Tree (AST) from Locks code

```
// example.locks

let value;
print value; // out: nil
value = 42;
print value; // out: 42;
```

```shell
$ locks parse example.locks
```

```
Program {
  stmts: [
    (
      StmtAssign {
        identifier: Identifier {
          name: "value",
          depth: None,
        },
        value: None,
      },
      72..82,
    ),
    (
      StmtPrint {
        value: (
          ExprIdentifier {
            identifier: Identifier {
              name: "value",
              depth: None,
            },
          },
          89..94,
        ),
      },
      83..95,
    ),
    (
      StmtExpr {
        value: (
          ExprAssign {
            identifier: Identifier {
              name: "value",
              depth: None,
            },
            value: (
              Number(
                42.0,
              ),
              116..118,
            ),
          },
          108..118,
        ),
      },
      108..119,
    ),
    (
      StmtPrint {
        value: (
          ExprIdentifier {
            identifier: Identifier {
              name: "value",
              depth: None,
            },
          },
          126..131,
        ),
      },
      120..132,
    ),
  ],
}
```

#### Print the disassembled bytecode from Locks code

```
// ./res/examples/number/fizzbuzz.locks

fn fizzBuzz(n) {
  for (let i = 1; i <= n; i = i + 1) {
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

```shell
$ locks disassemble ./res/examples/number/fizzbuzz.locks
```

```
0000 OP_CLOSURE          0 == '<fn fizzBuzz arity=1>'
| 0000 OP_CONSTANT         0 == '1'
| 0002 OP_GET_LOCAL        2
| 0004 OP_GET_LOCAL        1
| 0006 OP_LESS_EQUAL
| 0007 OP_JUMP_IF_FALSE    7 -> 82
| 0010 OP_POP
| 0011 OP_GET_LOCAL        2
| 0013 OP_CONSTANT         1 == '15'
| 0015 OP_MODULUS
| 0016 OP_CONSTANT         2 == '0'
| 0018 OP_EQUAL
| 0019 OP_JUMP_IF_FALSE   19 -> 29
| 0022 OP_POP
| 0023 OP_CONSTANT         3 == 'FizzBuzz'
| 0025 OP_PRINT
| 0026 OP_JUMP            26 -> 71
| 0029 OP_POP
| 0030 OP_GET_LOCAL        2
| 0032 OP_CONSTANT         4 == '3'
| 0034 OP_MODULUS
| 0035 OP_CONSTANT         2 == '0'
| 0037 OP_EQUAL
| 0038 OP_JUMP_IF_FALSE   38 -> 48
| 0041 OP_POP
| 0042 OP_CONSTANT         5 == 'Fizz'
| 0044 OP_PRINT
| 0045 OP_JUMP            45 -> 71
| 0048 OP_POP
| 0049 OP_GET_LOCAL        2
| 0051 OP_CONSTANT         6 == '5'
| 0053 OP_MODULUS
| 0054 OP_CONSTANT         2 == '0'
| 0056 OP_EQUAL
| 0057 OP_JUMP_IF_FALSE   57 -> 67
| 0060 OP_POP
| 0061 OP_CONSTANT         7 == 'Buzz'
| 0063 OP_PRINT
| 0064 OP_JUMP            64 -> 71
| 0067 OP_POP
| 0068 OP_GET_LOCAL        2
| 0070 OP_PRINT
| 0071 OP_GET_LOCAL        2
| 0073 OP_CONSTANT         0 == '1'
| 0075 OP_ADD
| 0076 OP_SET_LOCAL        2
| 0078 OP_POP
| 0079 OP_LOOP            79 -> 2
| 0082 OP_POP
| 0083 OP_POP
| 0084 OP_NIL
| 0085 OP_RETURN
0002 OP_DEFINE_GLOBAL    1 == 'fizzBuzz'
0004 OP_GET_GLOBAL       1 == 'fizzBuzz'
0006 OP_CONSTANT         2 == '15'
0008 OP_CALL             1
0010 OP_POP
0011 OP_NIL
0012 OP_RETURN
```

#### Run the Locks Language Server

```shell
$ locks lsp
```

### Docker

- `$ just build-docker` Build docker image
- `$ just run-repl-docker` Run `locks repl` inside built docker image

### VS Code Extension

Download the VS Code extension from the [latest build](https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml)

#### OR

1. Run `just build-all`
2. Copy `./target/release/locks[.exe]` somewhere in your `PATH`
3. Install `./vsc/out/locks-language-1.0.0.vsix` in VS Code
4. Create a new file and save as `*.locks`

#### Features

- Language Server integration
- Syntax & error highlighting
- Commands
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

## Forked

This project was forked from [loxcraft](https://github.com/ajeetdsouza/loxcraft). The intent of this fork is to learn about bytecode compilers, stack-based virtual machines, programming language design, and the desire to implement new language features.

There were many potential open source projects that could have been the basis for this toy language but loxcraft had a solid base to start from especially when I was focused on implementing language tooling first. It already had a working language server so the Visual Studio Code extension was a natural place to start.

With the syntax and implementation changes so far the Locks language has divered from Lox and will continue to do so.

### Changes Made

- Comments added to code base as part of the learning process while implementing changes
- Rewrote & decoupled disassembler to build a string of the disassembled bytecode instead of printing it
- CLI enhancements
  - Add `parse` command to print the AST from a `*.locks` file.
  - Add `dissassemble` command to print disassembled bytecode from a `*.locks` file
  - Add `exec` command to execute Locks code from the arg or piped in from `stdin`
- Language changes
  - Functions declarations: `fun` -> `fn`
  - Variable declarations: `var` -> `let`
  - Class fields must now be declared using `let field;` or `let field = "defaultValue";`
  - Class method declarations require `fn` like functions
  - Setting undeclared fields on classes will generate an error
  - Class inheritence: `class Child : Parent {}` -> `class Child extends Parent {}`
  - Lists: `[1, 2, 3]`, `arr[0]`, `arr[0] = 123`
- [Dockerize](Dockerfile) the Locks binary executable
- Implemented a [VS Code Extension](vsc)
  - Integrates the existing [language server](src/lsp.rs) to display parsing/compiler errors
  - Syntax Highlighting, Auto Pair Complete
  - Snippets
  - Commands/tasks to run, parse, and disassemble Locks code
  - Debug config for running VS Code Extension in VS Code
- Add builds (Locks binary executable & VS Code extension) as artifacts to the Github workflow
- Revamped the [Online Playground](https://kyleect.github.io/locks/)
  - Added a [documentation](https://kyleect.github.io/locks/#/docs) page with runnable code examples. This reuses the same webassembly build of the Locks runtime that the playground uses.
  - Add a "Parse & "Disassemble" button to the playground page and to all code examples on the docs page
  - Restyled playground
  - Add live updated and shareable playground urls
