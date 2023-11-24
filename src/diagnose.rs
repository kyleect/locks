#![cfg(feature = "lsp")]

use crate::vm::{Compiler, Gc};

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::types::Span;

#[derive(Debug, Default)]
pub struct Diagnoser {}

impl Diagnoser {
    pub fn get_diagnostics(source: &str) -> Vec<Diagnostic> {
        let mut gc = Gc::default();

        let program = match crate::syntax::parse(source, 0) {
            Ok(program) => program,
            Err(errors) => {
                return errors
                    .iter()
                    .map(|(err, span)| Diagnostic {
                        range: Diagnoser::get_range(source, span),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: err.to_string(),
                        ..Default::default()
                    })
                    .collect()
            }
        };

        Compiler::compile(&program, &mut gc)
            .err()
            .unwrap_or_default()
            .iter()
            .map(|(err, span)| Diagnostic {
                range: Diagnoser::get_range(source, span),
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.to_string(),
                ..Default::default()
            })
            .collect()
    }

    fn get_range(source: &str, span: &Span) -> Range {
        Range {
            start: Diagnoser::get_position(source, span.start),
            end: Diagnoser::get_position(source, span.end),
        }
    }

    fn get_position(source: &str, idx: usize) -> Position {
        let before = &source[..idx];
        let line = before.lines().count() - 1;
        let character = before.lines().last().unwrap().len();
        Position { line: line as _, character: character as _ }
    }
}

#[cfg(test)]
#[cfg(feature = "lsp")]
mod tests {
    use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

    use super::Diagnoser;

    #[test]
    fn it_works() {
        let source = String::from("var a = 123;");

        let d = Diagnoser::get_diagnostics(&source);

        let mut e = Vec::<Diagnostic>::new();

        e.push(Diagnostic {
            range: Range {
                start: Position { line: 0, character: 4 },
                end: Position { line: 0, character: 5 },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            message: String::from("SyntaxError: unexpected \"a\""),
            ..Default::default()
        });

        assert_eq!(e, d);
    }

    #[test]
    fn it_works_2() {
        let source = String::from("let a = 123");

        let d = Diagnoser::get_diagnostics(&source);

        let mut e = Vec::<Diagnostic>::new();

        e.push(Diagnostic {
            range: Range {
                start: Position { line: 0, character: 11 },
                end: Position { line: 0, character: 11 },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            message: String::from("SyntaxError: unexpected end of file"),
            ..Default::default()
        });

        assert_eq!(e, d);
    }
}
