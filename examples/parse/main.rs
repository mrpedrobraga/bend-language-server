use bend::fun::load_book::do_parse_book;
use std::fs;
use std::path::PathBuf;

fn main() {
    let source = "def main():\n  return \"Hello, World!\"";

    let book_res = do_parse_book(
        source,
        PathBuf::from("./example.bend").as_path(),
        bend::fun::Book::builtins(),
    );

    match book_res {
        Ok(v) => {
            fs::write("./examples/parse/book.txt", format!("{:#?}", v)).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:#?}", e);
        }
    }
}
