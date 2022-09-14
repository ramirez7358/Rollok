mod lexer;

use lexer::tokenize_all_collect;

fn main() {
    let test = "object Singleton {}";

    let toks = tokenize_all_collect(test);

    println!("#toks = {:?}", toks);
}
