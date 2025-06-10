#![allow(unused)]
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, String>,
}

impl Backend {
    async fn on_change(&self, params: &DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.content_changes[0].text.clone();
        self.document_map.insert(uri, text);
    }

    async fn get_update(&self, params: &DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = params.text_document.uri.to_string();
        match self.document_map.get(uri.as_str()) {
            Some(new_text) => {
                let start = Position {
                    line: 0,
                    character: 0,
                };
                let end = Position {
                    line: 0,
                    character: 0,
                };
                let range = Range { start, end };
                let text_edit = TextEdit {
                    range,
                    new_text: new_text.to_string(),
                };
                Some(vec![text_edit])
            }
            None => None,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Right(DocumentFormattingOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                completion_provider: Some(CompletionOptions::default()),
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
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Ok(self.get_update(&params).await)
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(&params).await;
        self.client
            .log_message(
                MessageType::WARNING,
                format!("{:?}", "GOT CHANGE".to_string()),
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
