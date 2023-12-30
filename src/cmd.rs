use std::fs;
use std::io::{self, BufRead, Write};

use anyhow::{bail, Context, Result};
use clap::Parser;

use crate::error::ErrorS;
use crate::vm::{Compiler, Disassembler, Gc, VM};

#[derive(Debug, Parser)]
#[command(about, author, disable_help_subcommand = true, propagate_version = true, version)]
pub enum Cmd {
    Lsp,
    Repl,
    Run { path: String },
    Exec { source: Option<String> },
    Parse { path: String },
    Disassemble { path: String },
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        #[allow(unused_variables)]
        match self {
            #[cfg(feature = "lsp")]
            Cmd::Lsp => crate::lsp::serve(),
            #[cfg(not(feature = "lsp"))]
            Cmd::Lsp => bail!("locks was not compiled with the lsp feature"),

            #[cfg(feature = "repl")]
            Cmd::Repl => crate::repl::run(),
            #[cfg(not(feature = "repl"))]
            Cmd::Repl => bail!("locks was not compiled with the repl feature"),

            Cmd::Run { path } => {
                let source = fs::read_to_string(path)
                    .with_context(|| format!("could not read file: {path}"))?;
                let stdout = &mut io::stdout().lock();
                let mut vm = VM::new();
                if let Err(e) = vm.run(&source, stdout) {
                    report_err(&source, e);
                    bail!("program exited with errors");
                }
                Ok(())
            }

            Cmd::Exec { source } => match source {
                Some(source) => {
                    let stdout = &mut io::stdout().lock();
                    let mut vm = VM::new();

                    if let Err(e) = vm.run(source, stdout) {
                        report_err(source, e);
                        bail!("program exited with errors");
                    }
                    Ok(())
                }
                None => {
                    let source = io::stdin()
                        .lock()
                        .lines()
                        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

                    let stdout = &mut io::stdout().lock();
                    let mut vm = VM::new();

                    if let Err(e) = vm.run(&source, stdout) {
                        report_err(&source, e);
                        bail!("program exited with errors");
                    }

                    Ok(())
                }
            },

            Cmd::Disassemble { path } => {
                let source = fs::read_to_string(path)
                    .with_context(|| format!("could not read file: {path}"))?;

                let mut gc = Gc::default();

                let program = match crate::syntax::parse(&source, source.len()) {
                    Ok(program) => program,
                    Err(error) => {
                        panic!("There was a parsing error! {:?}", error);
                    }
                };

                let function = Compiler::compile(&program, &mut gc);

                if let Ok(f) = function {
                    unsafe {
                        let chunk = &(*f).chunk;
                        let disassembler = Disassembler::new(chunk);

                        println!("{}", disassembler.disassemble(None));
                    }
                }

                Ok(())
            }

            Cmd::Parse { path } => {
                let source = fs::read_to_string(path)
                    .with_context(|| format!("could not read file: {path}"))?;

                let program = match crate::syntax::parse(&source, source.len()) {
                    Ok(program) => program,
                    Err(error) => {
                        panic!("There was a parsing error! {:?}", error);
                    }
                };

                let result = format!("{:#?}", program);
                let result = result.replace("    ", "  ");

                println!("{}", result);

                Ok(())
            }
        }
    }
}

fn report_err(source: &str, errors: Vec<ErrorS>) {
    let mut buffer = termcolor::Buffer::ansi();
    for err in errors {
        crate::error::report_error(&mut buffer, source, &err);
    }
    io::stderr().write_all(buffer.as_slice()).expect("failed to write to stderr");
}
