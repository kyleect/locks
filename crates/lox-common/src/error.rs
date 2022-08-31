use crate::types::{Span, Spanned};

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use termcolor::WriteColor;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, (E, Span)>;
pub type ErrorS = Spanned<Error>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("AttributeError: {0}")]
    AttributeError(AttributeError),
    #[error("IOError: {0}")]
    IoError(IoError),
    #[error("NameError: {0}")]
    NameError(NameError),
    #[error("SyntaxError: {0}")]
    SyntaxError(SyntaxError),
    #[error("TypeError: {0}")]
    TypeError(TypeError),
}

impl AsDiagnostic for Error {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            Error::AttributeError(e) => e.as_diagnostic(span),
            Error::IoError(e) => e.as_diagnostic(span),
            Error::NameError(e) => e.as_diagnostic(span),
            Error::SyntaxError(e) => e.as_diagnostic(span),
            Error::TypeError(e) => e.as_diagnostic(span),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AttributeError {
    #[error("{type_:?} object has no attribute {name:?}")]
    NoSuchAttribute { type_: String, name: String },
}

impl AsDiagnostic for AttributeError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            AttributeError::NoSuchAttribute { .. } => Diagnostic::error()
                .with_code("AttributeError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())]),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IoError {
    #[error("unable to write to file: {file:?}")]
    WriteError { file: String },
}

impl AsDiagnostic for IoError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            IoError::WriteError { .. } => Diagnostic::error()
                .with_code("IOError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())]),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum NameError {
    #[error("cannot access variable {name:?} in its own initializer")]
    AccessInsideInitializer { name: String },
    #[error("name {name:?} is already defined")]
    AlreadyDefined { name: String },
    #[error("name {name:?} is not defined")]
    NotDefined { name: String },
}

impl AsDiagnostic for NameError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            NameError::AccessInsideInitializer { .. }
            | NameError::AlreadyDefined { .. }
            | NameError::NotDefined { .. } => Diagnostic::error()
                .with_code("NameError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())]),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SyntaxError {
    #[error("extraneous input: {token:?}")]
    ExtraToken { token: String },
    #[error("invalid input")]
    InvalidToken,
    #[error("unexpected input")]
    UnexpectedInput { token: String },
    #[error("unexpected end of file")]
    UnrecognizedEOF { expected: Vec<String> },
    #[error("unexpected {token:?}")]
    UnrecognizedToken { token: String, expected: Vec<String> },
    #[error("unterminated string")]
    UnterminatedString,
    #[error(r#""return" outside function"#)]
    ReturnOutsideFunction,
}

impl AsDiagnostic for SyntaxError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            SyntaxError::ExtraToken { .. }
            | SyntaxError::InvalidToken
            | SyntaxError::ReturnOutsideFunction
            | SyntaxError::UnexpectedInput { .. }
            | SyntaxError::UnterminatedString => Diagnostic::error()
                .with_code("SyntaxError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())]),
            SyntaxError::UnrecognizedEOF { expected }
            | SyntaxError::UnrecognizedToken { expected, .. } => Diagnostic::error()
                .with_code("SyntaxError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())])
                .with_notes(vec![format!("expected: {}", one_of(expected))]),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TypeError {
    #[error("{name}() takes {exp_args} arguments but {got_args} were given")]
    ArityMismatch { name: String, exp_args: usize, got_args: usize },
    #[error("init() should use an empty return, not {type_:?}")]
    InitInvalidReturnType { type_: String },
    #[error("{type_:?} object is not callable")]
    NotCallable { type_: String },
    #[error(r#"superclass should be of type "class", not {type_:?}"#)]
    SuperclassInvalidType { type_: String },
    #[error("unsupported operand type for {op}: {rt_type:?}")]
    UnsupportedOperandPrefix { op: String, rt_type: String },
    #[error("unsupported operand type(s) for {op}: {lt_type:?} and {rt_type:?}")]
    UnsupportedOperandInfix { op: String, lt_type: String, rt_type: String },
}

impl AsDiagnostic for TypeError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            TypeError::ArityMismatch { .. }
            | TypeError::InitInvalidReturnType { .. }
            | TypeError::NotCallable { .. }
            | TypeError::SuperclassInvalidType { .. }
            | TypeError::UnsupportedOperandPrefix { .. }
            | TypeError::UnsupportedOperandInfix { .. } => Diagnostic::error()
                .with_code("TypeError")
                .with_message(self.to_string())
                .with_labels(vec![Label::primary((), span.clone())]),
        }
    }
}

trait AsDiagnostic {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()>;
}

fn one_of(tokens: &[String]) -> String {
    let (token_last, tokens) = match tokens.split_last() {
        Some((token_last, &[])) => return token_last.to_string(),
        Some((token_last, tokens)) => (token_last, tokens),
        None => return "nothing".to_string(),
    };

    let mut output = String::new();
    for token in tokens {
        output.push_str(token);
        output.push_str(", ");
    }
    output.push_str("or ");
    output.push_str(token_last);
    output
}

pub fn report_err(writer: &mut dyn WriteColor, source: &str, (err, span): &ErrorS) {
    let file = SimpleFile::new("<script>", source);
    let config = term::Config::default();
    let diagnostic = err.as_diagnostic(span);
    term::emit(writer, &config, &file, &diagnostic).expect("failed to write to output");
}