#![cfg(feature = "lsp")]

use anyhow::{Context, Result};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, InitializeResult, Position, Range, ServerCapabilities, ServerInfo,
    TextDocumentSyncKind,
};
use tower_lsp::{jsonrpc, Client, LanguageServer, LspService, Server};

use crate::diagnose::{Diagnoser, Diagnosis, DiagnosisPosition, DiagnosisRange, DiagnosisSeverity};

#[derive(Debug)]
struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncKind::FULL.into()),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let source = &params.text_document.text;
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);
        let diagnostics = Diagnoser::get_diagnostics(source);
        self.client
            .publish_diagnostics(
                uri,
                diagnostics.iter().map(|x| (*x).clone().into()).collect(),
                version,
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let source = &params.content_changes.first().unwrap().text;
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);
        let diagnostics = Diagnoser::get_diagnostics(source);
        self.client
            .publish_diagnostics(
                uri,
                diagnostics.iter().map(|x| (*x).clone().into()).collect(),
                version,
            )
            .await;
    }
}

impl From<Diagnosis> for Diagnostic {
    fn from(value: Diagnosis) -> Self {
        Diagnostic {
            range: value.range.into(),
            severity: value.severity.map(|x| x.into()),
            message: value.message,
            ..Default::default()
        }
    }
}

impl From<DiagnosisPosition> for Position {
    fn from(value: DiagnosisPosition) -> Self {
        Position { line: value.line, character: value.character }
    }
}

impl From<DiagnosisRange> for Range {
    fn from(value: DiagnosisRange) -> Self {
        Range { start: value.start.into(), end: value.end.into() }
    }
}

impl From<DiagnosisSeverity> for DiagnosticSeverity {
    fn from(value: DiagnosisSeverity) -> Self {
        match value {
            DiagnosisSeverity::ERROR => DiagnosticSeverity::ERROR,
            DiagnosisSeverity::HINT => DiagnosticSeverity::HINT,
            DiagnosisSeverity::INFORMATION => DiagnosticSeverity::INFORMATION,
            DiagnosisSeverity::WARNING => DiagnosticSeverity::WARNING,
            _ => panic!("Invalid diagnosis severity {:?}", value),
        }
    }
}

pub fn serve() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to start async runtime")?
        .block_on(serve_async());
    Ok(())
}

async fn serve_async() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
