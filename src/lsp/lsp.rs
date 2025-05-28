use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::eval::builtin::builtin::BUILTIN_FUNCTIONS;
use crate::lexer::Lexer;

pub struct Backend {
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<Url, String>>>,
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
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents
            .write()
            .await
            .insert(uri.clone(), text.clone());
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params
            .content_changes
            .first()
            .map_or(String::new(), |change| change.text.clone());

        self.documents.write().await.insert(uri.clone(), text);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
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

        let uri = params.text_document_position.text_document.uri;
        let documents = self.documents.read().await;
        let text = documents.get(&uri).cloned().unwrap_or_default();

        let mut lexer = Lexer::new(text.as_str());
        let mut tokens = vec![];
        loop {
            let token = lexer.next_token();
            if token.is_eof() || token.is_invalid() {
                break;
            }
            tokens.push(token);
        }

        Ok(Some(CompletionResponse::Array(items)))
    }
}
