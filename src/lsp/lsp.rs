use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::ast::{Expr, Stmt};
use crate::eval::builtin::builtin::BUILTIN_FUNCTIONS;
use crate::lexer::Lexer;
use crate::parser::Parser;

use super::keyword::KEYWORDS;

pub struct Backend {
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<Url, String>>>,
    pub builtin_items: Arc<RwLock<Vec<CompletionItem>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        let mut items = vec![];
        for func in BUILTIN_FUNCTIONS.iter() {
            items.push(CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(func.description.to_string()),
                documentation: None,
                ..Default::default()
            });
        }
        for value in ["true", "false"] {
            items.push(CompletionItem {
                label: value.to_string(),
                kind: Some(CompletionItemKind::VALUE),
                detail: Some(value.to_string()),
                documentation: None,
                ..Default::default()
            });
        }
        for keyword in KEYWORDS.iter() {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Keyword".to_string()),
                documentation: None,
                ..Default::default()
            });
        }

        Backend {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            builtin_items: Arc::new(RwLock::new(items)),
        }
    }
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
        let mut items: Vec<CompletionItem> = self.builtin_items.read().await.clone();

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

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Parse error: {}", err))
                    .await;
                return Ok(None);
            }
        };

        for stmt in ast.stmts.iter() {
            match stmt {
                Stmt::Let { name, val } => match val {
                    Expr::Func { params, body: _ } => {
                        items.push(CompletionItem {
                            label: name.clone(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some(format!("Function with params: {:?}", params)),
                            documentation: None,
                            ..Default::default()
                        });
                    }
                    _ => {
                        let var = CompletionItem {
                            label: name.clone(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some("Variable".to_string()),
                            documentation: None,
                            ..Default::default()
                        };
                        items.push(var);
                    }
                },
                _ => {
                    // do nothing for other statements
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }
}
