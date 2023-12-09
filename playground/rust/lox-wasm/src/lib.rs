use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

use locks::diagnose::{Diagnoser, Diagnosis};
use locks::error::report_error;
use locks::syntax::parse;
use locks::vm::{Compiler, Disassembler, Gc, VM};
use serde::Serialize;
use termcolor::{Color, WriteColor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn loxDiagnose(source: &str) {
    console_error_panic_hook::set_once();

    let diagnostics = Diagnoser::get_diagnostics(source);

    postMessage(&Message::Diagnostics { diagnostics }.to_string());
    postMessage(&Message::ExitSuccess.to_string());
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn loxRun(source: &str) {
    console_error_panic_hook::set_once();

    let output = &mut Output::new();
    match VM::default().run(source, output) {
        Ok(()) => postMessage(&Message::ExitSuccess.to_string()),
        Err(errors) => {
            let mut writer = HtmlWriter::new(output);
            for e in errors.iter() {
                report_error(&mut writer, source, e);
            }
            postMessage(&Message::ExitFailure.to_string());
        }
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn locksDisassemble(source: &str) {
    console_error_panic_hook::set_once();

    let output = &mut Output::new();

    let mut gc = Gc::default();

    let program = match parse(source, source.len()) {
        Ok(program) => program,
        Err(error) => {
            panic!("There was a parsing error! {:?}", error);
        }
    };

    match Compiler::compile(&program, &mut gc) {
        Ok(function) => {
            let chunk = unsafe { &(*function).chunk };

            let d = Disassembler::new(chunk);

            let result = d.disassemble(None);
            let encoded_result = askama_escape::escape(&result, askama_escape::Html).to_string();

            let _ = output.write(encoded_result.as_bytes());

            postMessage(&Message::ExitSuccess.to_string());
        }
        Err(errors) => {
            let mut writer = HtmlWriter::new(output);
            for e in errors.iter() {
                report_error(&mut writer, source, e);
            }
            postMessage(&Message::ExitFailure.to_string());
        }
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn locksParse(source: &str) {
    console_error_panic_hook::set_once();

    let output = &mut Output::new();

    let program = match parse(source, source.len()) {
        Ok(program) => program,
        Err(errors) => {
            let mut writer = HtmlWriter::new(output);
            for e in errors.iter() {
                report_error(&mut writer, source, e);
            }
            postMessage(&Message::ExitFailure.to_string());

            return;
        }
    };

    let result = format!("{:#?}", program);

    let result = result.replace("    ", "  ");

    let encoded_result = askama_escape::escape(&result, askama_escape::Html).to_string();

    let _ = output.write(encoded_result.as_bytes());

    postMessage(&Message::ExitSuccess.to_string());
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum Message {
    ExitFailure,
    ExitSuccess,
    Output { text: String },
    Diagnostics { diagnostics: Vec<Diagnosis> },
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).expect("could not serialize message"))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self)]
    fn postMessage(s: &str);
}

#[derive(Debug)]
struct Output;

impl Output {
    fn new() -> Self {
        Self
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let text = String::from_utf8_lossy(buf).to_string();
        postMessage(&Message::Output { text }.to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Provides a [`WriteColor`] implementation for HTML, using Bootstrap 5.1
/// classes.
#[derive(Debug)]
struct HtmlWriter<W> {
    writer: W,
    span_count: usize,
}

impl<W> HtmlWriter<W> {
    fn new(writer: W) -> Self {
        HtmlWriter { writer, span_count: 0 }
    }
}

impl<W: Write> Write for HtmlWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let escaped = String::from_utf8_lossy(buf);
        let escaped = askama_escape::escape(&escaped, askama_escape::Html).to_string();
        write!(self.writer, "{escaped}")?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> WriteColor for HtmlWriter<W> {
    fn supports_color(&self) -> bool {
        true
    }

    fn set_color(&mut self, spec: &termcolor::ColorSpec) -> io::Result<()> {
        if spec.reset() {
            self.reset()?;
        }

        let mut classes = Vec::new();
        if let Some(fg) = spec.fg() {
            match fg {
                Color::Black => classes.push("text-black"),
                Color::Blue => classes.push("text-primary"),
                Color::Green => classes.push("text-success"),
                Color::Red => classes.push("text-danger"),
                Color::White => classes.push("text-white"),
                Color::Yellow => classes.push("text-warning"),
                _ => (),
            };
        }
        if let Some(bg) = spec.bg() {
            match bg {
                Color::Black => classes.push("bg-black"),
                Color::Blue => classes.push("bg-primary"),
                Color::Green => classes.push("bg-success"),
                Color::Red => classes.push("bg-danger"),
                Color::White => classes.push("bg-white"),
                Color::Yellow => classes.push("bg-warning"),
                _ => (),
            };
        }
        if spec.bold() {
            classes.push("fw-bold");
        }
        if spec.dimmed() {
            classes.push("opacity-75");
        }
        if spec.italic() {
            classes.push("fst-italic");
        }
        if spec.underline() {
            classes.push("text-decoration-underline");
        }

        if !classes.is_empty() {
            write!(self.writer, r#"<span class="{}">"#, classes.join(" "))?;
            self.span_count += 1;
        }
        Ok(())
    }

    fn reset(&mut self) -> io::Result<()> {
        for _ in 0..self.span_count {
            write!(self.writer, "</span>")?;
        }
        self.span_count = 0;
        Ok(())
    }
}
