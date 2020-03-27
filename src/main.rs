extern crate jehanni;

use jehanni::input_file::open_file;
use jehanni::tokenizer::TokenList;

fn main() {
    let code = open_file();
    // println!("{}", code);
    let tokens = TokenList::new(&code);
    println!("{}", tokens);
}
