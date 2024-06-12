use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc::Result, Client, LanguageServer, LspService, Server};

macro_rules! code_action {
    ($title:expr) => {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: ($title).into(),
            kind: Some(CodeActionKind::REFACTOR),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        })
    };
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),

                ..Default::default()
            },
            server_info: Default::default(),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::LOG, "o(≧∇≦)o Booting up!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::LOG, "_(ᴗ˳ᴗ)_ Shutting down...")
            .await;
        Ok(())
    }

    async fn code_action(&self, _: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let ca: CodeActionResponse = vec![
            code_action!("Do a barrel roll"),
            code_action!("Re-align stray AI"),
            code_action!("Generate AGI"),
            code_action!("Solve Alignment Problem"),
            code_action!("DM Sam Altman"),
            code_action!("Discover meaning of life"),
        ];

        Ok(Some(ca))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        return Ok(None);

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: include_str!("./example_hover.md").into(),
            }),
            range: Some(Range {
                start: Position::new(5, 0),
                end: Position::new(5, 27),
            }),
        }))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
