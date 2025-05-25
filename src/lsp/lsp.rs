use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::eval::builtin::builtin::BUILTIN_FUNCTIONS;

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
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(
                        "abcdefghijklmnopqrstuvwxyz"
                            .chars()
                            .map(|c| c.to_string())
                            .collect(),
                    ),

                    ..Default::default()
                }),
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
        self.client
            .log_message(MessageType::INFO, &format!("Opened document: {}", text))
            .await;
    }
    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items = BUILTIN_FUNCTIONS
            .iter()
            .map(|func| CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(func.description.to_string()),
                documentation: None,
                ..Default::default()
            })
            .collect();
        Ok(Some(CompletionResponse::Array(items)))
    }
}
