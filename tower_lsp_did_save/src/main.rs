use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, String>,
}

fn update_text(text: &str) -> Option<String> {
    let lines: Vec<_> = text.lines().map(|l| format!(".{}", l)).collect();
    Some(lines.join("\n"))
}

impl Backend {
    async fn on_change(&self, params: &DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.content_changes[0].text.clone();
        self.document_map.insert(uri, text);
    }

    async fn get_update(&self, params: &DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = params.text_document.uri.to_string();
        let initial_text = self.document_map.get(uri.as_str())?;
        let text = update_text(&initial_text)?;
        let (line, character) = last_position(&text)?;
        let start = Position {
            line: 0,
            character: 0,
        };
        // TODO: figure out where/how using
        // `as u32` can got sideways
        let end = Position {
            line: line as u32,
            character: character as u32,
        };
        let range = Range { start, end };
        let text_edit = TextEdit {
            range,
            new_text: text.to_string(),
        };
        Some(vec![text_edit])
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
            .log_message(MessageType::INFO, "Tower LSP Example Server Initialized!")
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

fn last_position(text: &str) -> Option<(usize, usize)> {
    // Add a newline because .lines removes it.
    let check_text = format!("{}\n", text);
    let lines: Vec<&str> = check_text.lines().collect();
    let line_count = lines.len() - 1;
    let last_char = lines
        .iter()
        .last()?
        .graphemes(true)
        .collect::<Vec<&str>>()
        .len();
    Some((line_count, last_char))
}
