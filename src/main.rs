use tower_lsp::{LspService, Server};
pub mod backend;
pub mod introspection;

use backend::Backend;

#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn main() {
    compile_error!("Needs implementing for WASI.")
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
