use lazy_static::lazy_static;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

lazy_static! {
    pub static ref BUILTIN_COMPLETIONS: [CompletionItem; 23] = [
        simple_completion_item("def", "`def` is a keyword used to define a function or method.", CompletionItemKind::KEYWORD),
        simple_completion_item("switch", "`switch` is a keyword used to create a switch statement for multi-branch conditionals.", CompletionItemKind::KEYWORD),
        simple_completion_item("case", "`case` is a keyword used to define a branch in a switch statement.", CompletionItemKind::KEYWORD),
        simple_completion_item("return", "`return` is a keyword used to exit a function and return a value.", CompletionItemKind::KEYWORD),
        simple_completion_item("if", "`if` is a keyword used for conditional branching.", CompletionItemKind::KEYWORD),
        simple_completion_item("else", "`else` is a keyword used to provide an alternative branch in a conditional.", CompletionItemKind::KEYWORD),
        simple_completion_item("when", "`when` is a keyword used to specify conditions in pattern matching.", CompletionItemKind::KEYWORD),
        simple_completion_item("match", "`match` is a keyword used for pattern matching against values.", CompletionItemKind::KEYWORD),
        simple_completion_item("λ", "`λ` is a literal used to define bend code.", CompletionItemKind::OPERATOR),
        simple_completion_item("Some", "`Some` is a keyword used to represent a value in an option type.", CompletionItemKind::KEYWORD),
        simple_completion_item("data", "`data` is a keyword used to define a data type.", CompletionItemKind::KEYWORD),
        simple_completion_item("let", "`let` is a keyword used to bind a value to a variable.", CompletionItemKind::KEYWORD),
        simple_completion_item("use", "`use` is a keyword used to bring modules or values into scope.", CompletionItemKind::KEYWORD),
        simple_completion_item("object", "`object` is a keyword used to define an object or an instance of a class.", CompletionItemKind::KEYWORD),
        simple_completion_item("fold", "`fold` is a keyword used to reduce a collection to a single value using a binary operation.", CompletionItemKind::KEYWORD),
        simple_completion_item("open", "`open` is a keyword used to open a file.", CompletionItemKind::KEYWORD),
        simple_completion_item("do", "`do` is a keyword used to start a block of expressions.", CompletionItemKind::KEYWORD),
        simple_completion_item("bind", "`bind` is a keyword used in monadic operations to chain computations.", CompletionItemKind::KEYWORD),
        simple_completion_item("Name", "`Name` is a keyword used to define a named entity.", CompletionItemKind::KEYWORD),
        simple_completion_item("identity", "`identity` is a keyword used to represent a function that returns its argument.", CompletionItemKind::KEYWORD),
        simple_completion_item("Bool", "`Bool` is a keyword used to define a boolean data type.", CompletionItemKind::KEYWORD),
        simple_completion_item("ask", "`ask` is a keyword used to retrieve a value from a context or environment.", CompletionItemKind::KEYWORD),
        simple_completion_item("with", "`with` is a keyword used to introduce a block where certain bindings are in scope.", CompletionItemKind::KEYWORD),
    ];
}

fn simple_completion_item(
    label: &str,
    detail: &str,
    comp_item_kind: CompletionItemKind,
) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        label_details: Some(CompletionItemLabelDetails {
            detail: None,
            description: Some(label.to_string()),
        }),
        kind: Some(comp_item_kind),
        detail: Some(detail.to_string()),
        documentation: None,
        deprecated: Some(false),
        preselect: Some(true),
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
