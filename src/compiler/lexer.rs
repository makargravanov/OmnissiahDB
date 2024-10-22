/*#[derive(Debug, PartialEq)]
pub enum TokenType {
    Select,
    Insert,
    Update,
    Delete,

    Create,
    Drop,

    Identifier,
    Symbol,
    Number,
    StringLiteral,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}*/

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<String> {
        let mut tokens = Vec::new();
        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.input.next();
                }
                '(' | ')' | ',' | ';' => {
                    tokens.push(ch.to_string());
                    self.input.next();
                }
                '\'' | '"' => {
                    tokens.push(self.collect_string_literal(ch));
                }
                '!' | '=' | '<' | '>' => {
                    tokens.push(self.collect_operator());
                }
                _ => {
                    if ch.is_alphanumeric() || ch == '_' {
                        tokens.push(self.collect_identifier());
                    } else {
                        self.input.next();
                    }
                }
            }
        }
        tokens
    }

    fn collect_string_literal(&mut self, quote: char) -> String {
        let mut literal = String::new();
        literal.push(self.input.next().unwrap());
        while let Some(&ch) = self.input.peek() {
            literal.push(ch);
            self.input.next();
            if ch == quote {
                break;
            }
        }
        literal
    }

    fn collect_operator(&mut self) -> String {
        let mut operator = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch == '!' || ch == '=' || ch == '<' || ch == '>' {
                operator.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        operator
    }

    fn collect_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        identifier
    }
}

