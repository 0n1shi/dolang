use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        if text.contains("foo") {
            self.client
                .publish_diagnostics(
                    params.text_document.uri,
                    vec![Diagnostic {
                        range: Range {
                            start: Position::new(0, 0),
                            end: Position::new(0, 3),
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: "\"foo\" を使わないでください".to_string(),
                        ..Default::default()
                    }],
                    None,
                )
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
