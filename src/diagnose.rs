use crate::vm::{Compiler, Gc};
use serde::{Deserialize, Serialize};

use crate::types::Span;

#[derive(Debug, Default)]
pub struct Diagnoser {}

impl Diagnoser {
    pub fn get_diagnostics(source: &str) -> Vec<Diagnosis> {
        let mut gc = Gc::default();

        let program = match crate::syntax::parse(source, 0) {
            Ok(program) => program,
            Err(errors) => {
                return errors
                    .iter()
                    .map(|(err, span)| Diagnosis {
                        range: Diagnoser::get_range(source, span),
                        severity: Some(DiagnosisSeverity::ERROR),
                        message: err.to_string(),
                        ..Default::default()
                    })
                    .collect();
            }
        };

        Compiler::compile(&program, &mut gc)
            .err()
            .unwrap_or_default()
            .iter()
            .map(|(err, span)| Diagnosis {
                range: Diagnoser::get_range(source, span),
                severity: Some(DiagnosisSeverity::ERROR),
                message: err.to_string(),
                ..Default::default()
            })
            .collect()
    }

    pub fn get_range(source: &str, span: &Span) -> DiagnosisRange {
        DiagnosisRange {
            start: Diagnoser::get_position(source, span.start),
            end: Diagnoser::get_position(source, span.end),
        }
    }

    pub fn get_position(source: &str, idx: usize) -> DiagnosisPosition {
        let before = &source[..idx];
        let line = before.lines().count() - 1;
        let character = before.lines().last().unwrap().len();
        DiagnosisPosition { line: line as _, character: character as _ }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnosis {
    pub range: DiagnosisRange,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosisSeverity>,

    pub message: String,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiagnosisSeverity(i32);
impl DiagnosisSeverity {
    pub const ERROR: DiagnosisSeverity = DiagnosisSeverity(1);
    pub const WARNING: DiagnosisSeverity = DiagnosisSeverity(2);
    pub const INFORMATION: DiagnosisSeverity = DiagnosisSeverity(3);
    pub const HINT: DiagnosisSeverity = DiagnosisSeverity(4);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default, Deserialize, Serialize)]
pub struct DiagnosisPosition {
    pub line: u32,
    pub character: u32,
}

impl DiagnosisPosition {
    pub fn new(line: u32, character: u32) -> DiagnosisPosition {
        DiagnosisPosition { line, character }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Deserialize, Serialize)]
pub struct DiagnosisRange {
    /// The range's start position (inclusive)
    pub start: DiagnosisPosition,
    /// The range's end position (exclusive)
    pub end: DiagnosisPosition,
}

impl DiagnosisRange {
    pub fn new(start: DiagnosisPosition, end: DiagnosisPosition) -> DiagnosisRange {
        DiagnosisRange { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::Diagnoser;
    use crate::diagnose::{Diagnosis, DiagnosisPosition, DiagnosisRange, DiagnosisSeverity};

    #[test]
    fn it_works() {
        let source = String::from("var a = 123;");

        let d = Diagnoser::get_diagnostics(&source);

        let mut e = Vec::<Diagnosis>::new();

        e.push(Diagnosis {
            range: DiagnosisRange {
                start: DiagnosisPosition { line: 0, character: 4 },
                end: DiagnosisPosition { line: 0, character: 5 },
            },
            severity: Some(DiagnosisSeverity::ERROR),
            message: String::from("SyntaxError: unexpected \"a\""),
            ..Default::default()
        });

        assert_eq!(e, d);
    }

    #[test]
    fn it_works_2() {
        let source = String::from("let a = 123");

        let d = Diagnoser::get_diagnostics(&source);

        let mut e = Vec::<Diagnosis>::new();

        e.push(Diagnosis {
            range: DiagnosisRange {
                start: DiagnosisPosition { line: 0, character: 11 },
                end: DiagnosisPosition { line: 0, character: 11 },
            },
            severity: Some(DiagnosisSeverity::ERROR),
            message: String::from("SyntaxError: unexpected end of file"),
            ..Default::default()
        });

        assert_eq!(e, d);
    }
}
