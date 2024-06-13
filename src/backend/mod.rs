//! The LSP implementation.

use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, Hover,
        HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, MarkedString, MessageType, ServerCapabilities,
    },
    Client, LanguageServer,
};

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
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("def".to_string(), "`def` is a keyword used to define a function or method.".to_string()),
            CompletionItem::new_simple("switch".to_string(), "`switch` is a keyword used to create a switch statement for multi-branch conditionals.".to_string()),
            CompletionItem::new_simple("case".to_string(), "`case` is a keyword used to define a branch in a switch statement.".to_string()),
            CompletionItem::new_simple("return".to_string(), "`return` is a keyword used to exit a function and return a value.".to_string()),
            CompletionItem::new_simple("if".to_string(), "`if` is a keyword used for conditional branching.".to_string()),
            CompletionItem::new_simple("else".to_string(), "`else` is a keyword used to provide an alternative branch in a conditional.".to_string()),
            CompletionItem::new_simple("when".to_string(), "`when` is a keyword used to specify conditions in pattern matching.".to_string()),
            CompletionItem::new_simple("match".to_string(), "`match` is a keyword used for pattern matching against values.".to_string()),
            CompletionItem::new_simple("λ".to_string(), "`λ` is a literal used to define bend code.".to_string()),
            CompletionItem::new_simple("Some".to_string(), "`Some` is a keyword used to represent a value in an option type.".to_string()),
            CompletionItem::new_simple("data".to_string(), "`data` is a keyword used to define a data type.".to_string()),
            CompletionItem::new_simple("let".to_string(), "`let` is a keyword used to bind a value to a variable.".to_string()),
            CompletionItem::new_simple("use".to_string(), "`use` is a keyword used to bring modules or values into scope.".to_string()),
            CompletionItem::new_simple("object".to_string(), "`object` is a keyword used to define an object or an instance of a class.".to_string()),
            CompletionItem::new_simple("fold".to_string(), "`fold` is a keyword used to reduce a collection to a single value using a binary operation.".to_string()),
            CompletionItem::new_simple("open".to_string(), "`open` is a keyword used to open a file.".to_string()),
            CompletionItem::new_simple("do".to_string(), "`do` is a keyword used to start a block of expressions.".to_string()),
            CompletionItem::new_simple("bind".to_string(), "`bind` is a keyword used in monadic operations to chain computations.".to_string()),
            CompletionItem::new_simple("Name".to_string(), "`Name` is a keyword used to define a named entity.".to_string()),
            CompletionItem::new_simple("identity".to_string(), "`identity` is a keyword used to represent a function that returns its argument.".to_string()),
            CompletionItem::new_simple("Bool".to_string(), "`Bool` is a keyword used to define a boolean data type.".to_string()),
            CompletionItem::new_simple("ask".to_string(), "`ask` is a keyword used to retrieve a value from a context or environment.".to_string()),
            CompletionItem::new_simple("with".to_string(), "`with` is a keyword used to introduce a block where certain bindings are in scope.".to_string()),
        ])))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        return Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }));
    }
}
