use regex::Regex;

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
    pos: usize,
    token: Token,
    text: String,
    original_text: String
}
impl Lexer {
    fn new(str: String) -> Lexer {
        Lexer {
            pos: 0,
            token: Token::BOF,
            text: String::from(""),
            original_text: str
        }
    }
    fn scan(&mut self) {
        self.scan_forward(Regex::new(r"[ \t\n]").unwrap())
    }
    fn scan_forward(&mut self, regex: Regex) {
        while self.pos < self.original_text.len() {
            if let Some(c) = &self.original_text.chars().nth(self.pos) {
                if regex.is_match(&c.to_string()) {
                    self.pos += 1;
                    continue;
                }
            }
            break;
        }
    }
}

fn lex(code: String) -> Lexer {
    Lexer::new(code)
}

fn bind(tree: &Lexer) {}

fn check(tree: &Lexer) {}

fn emit(tree: &Lexer) {}

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
    use super::*;
    #[test]
    fn it_works() {
        compile(String::from("var num: number = 1"));
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn scan() {
        let mut lex = Lexer::new(String::from("var a = 1"));
        lex.scan();
        assert_eq!(lex.pos, 0);

        let mut lex = Lexer::new(String::from("  var a = 1"));
        lex.scan();
        assert_eq!(lex.pos, 2);
    }
}
