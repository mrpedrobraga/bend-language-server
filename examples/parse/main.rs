use bend::diagnostics::DiagnosticsConfig;
use bend::fun::parser::TermParser;
use bend::{check_book, CompileOpts};
use std::fs;
use std::path::PathBuf;

fn main() {
    let path_buf = PathBuf::from("./examples/hello_world.bend");
    let path = path_buf.as_path();
    let source = std::fs::read_to_string(path).unwrap();

    let book_res = TermParser::new(source.as_str()).parse_book(bend::fun::Book::builtins(), false);

    match book_res {
        Ok(mut book) => {
            let book_check = check_book(
                &mut book,
                DiagnosticsConfig::default(),
                CompileOpts::default(),
            );
            print!("{:#?}", book_check);
            fs::write("./examples/parse/book.txt", format!("{:#?}", book)).unwrap();
        }
        Err(e) => {
            print!("{}", e);
        }
    }
}
