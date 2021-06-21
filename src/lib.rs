enum Token {
    Function,
    Var,
    Return,
    Equals,
    Literal,
    Identifier,
    Newline,
    Semicolon,
    Colon,
    Whitespace,
    Unknown,
    BOF,
    EOF,
}

struct Lexer {
}


fn lex(code: String) -> Lexer {
    Lexer {}

}

fn bind(tree: &Lexer) {

}

fn check(tree: &Lexer) {

}

fn emit(tree: &Lexer) {
}

fn transform(tree: &Lexer) -> &Lexer {
    tree
}

pub fn compile(code: String) {
    let tree = lex(code);
    bind(&tree);
    check(&tree);
    emit(transform(&tree))
}

#[cfg(test)]
mod tests {
    use super::compile;
    #[test]
    fn it_works() {
        compile(String::from("var num: number = 1"));
        assert_eq!(2 + 2, 4);
    }
}
