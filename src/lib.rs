use std::collections::HashMap;
use regex::Regex;

#[derive(Copy, Clone)]
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
    original_text: String,
    keyword_map: HashMap<String, Token>,
}

impl Lexer {
    fn new(str: String) -> Lexer {
        let mut keyword_map = HashMap::new();
        keyword_map.insert(String::from("function"), Token::Function);
        keyword_map.insert(String::from("var"), Token::Var);
        keyword_map.insert(String::from("return"), Token::Return);

        Lexer {
            pos: 0,
            token: Token::BOF,
            text: String::from(""),
            original_text: str,
            keyword_map,
        }
    }
    fn scan(&mut self) {
        self.scan_forward(Regex::new(r"[ \t\n]").unwrap());
        let start = self.pos;
        if self.pos == self.original_text.len() {
            self.token = Token::EOF;
        } else if Regex::new(r"[0-9]").unwrap().is_match(&self.original_text.chars().nth(self.pos).unwrap().to_string()) {
            self.scan_forward(Regex::new(r"[0-9]").unwrap());
            self.text = self.original_text[start..self.pos].to_string();
            if let Some(k) = self.keyword_map.get(&self.text) {
                self.token = *k;
            } else {
                self.token = Token::Identifier;
            }
        } else {
            self.pos += 1;
            self.token = match self.original_text.chars().nth(self.pos - 1) {
                Some('=') => Token::Equals,
                Some(';') => Token::Semicolon,
                Some(':') => Token::Colon,
                _   => Token::Unknown
            }
        }

    }
    fn scan_forward(&mut self, regex: Regex) {
        while self.pos < self.original_text.len() {
            if let Some(c) = &self.original_text.chars().nth(self.pos) {
                if regex.is_match(&c.to_string()) {
                    println!("match, {}", c);
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
    }
}

fn lex(code: String) -> Lexer {
    Lexer::new(code)
}

struct Symbol {
}

struct Module {
}

fn parse(lex: Lexer) -> Module {
    Module {}
}

fn bind(tree: &Module) {}

fn check(tree: &Module) {}

fn emit(tree: &Module) {}

fn transform(tree: &Module) -> &Module {
    tree
}

pub fn compile(code: String) {
    let tree = parse(lex(code));
    bind(&tree);
    check(&tree);
    emit(transform(&tree))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_forward() {
        let mut lex = Lexer::new(String::from("var a = 1"));
        lex.scan();
        assert_eq!(lex.pos, 1);

        let mut lex = Lexer::new(String::from("  var a = 1"));
        lex.scan();
        assert_eq!(lex.pos, 3);
    }
}
