//! The LSP implementation.

use builtin_completions::BUILTIN_COMPLETIONS;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, Hover,
        HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, MarkedString, MessageType, ServerCapabilities,
    },
    Client, LanguageServer,
};

pub mod builtin_completions;

use crate::introspection::WorkspaceKnowledge;
#[derive(Debug)]
pub struct Backend {
    client: Client,
    workspace_knowledge: WorkspaceKnowledge,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace_knowledge: WorkspaceKnowledge::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),

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

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut completions: Vec<CompletionItem> = vec![];
        completions.extend(BUILTIN_COMPLETIONS.clone().into_iter());

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        return Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }));
    }
}
