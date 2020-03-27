extern crate jehanni;

use jehanni::input_file::open_file;
use jehanni::tokenizer::TokenList;
use jehanni::parser::AST;

fn main() {
    let code = open_file();
    // println!("{}", code);
    let tokens = TokenList::new(&code);
    // println!("{}", tokens);
    let ast = AST::new(tokens);
    println!("{}", ast);
}
