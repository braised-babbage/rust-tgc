use std::{iter::Peekable, str::CharIndices};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind<'source> {
    // symbols
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    Period,
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    NotEquals,   // <>
    LT,          // <
    LE,          // <=
    GT,          // >
    GE,          // >=
    Ampsersand,  // &
    Pipe,        // |
    ColonEquals, // :=
    // keywords
    Array,
    If,
    Then,
    Else,
    While,
    For,
    To,
    Do,
    Let,
    In,
    End,
    Of,
    Break,
    Nil,
    Function,
    Var,
    Type,
    Import,
    Primitive,
    // other
    Id(&'source str),
    Num(i32),
    String(&'source str),
    // exceptional
    EOF,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TokenPos {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    pub kind: TokenKind<'source>,
    pub pos: TokenPos,
}

pub struct Lexer<'source> {
    input: &'source str,
    iter: Peekable<CharIndices<'source>>,
    c: char,   // current char
    ci: usize, // current offset
    error: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        let mut lex = Lexer {
            input,
            iter: input.char_indices().peekable(),
            c: '\x00',
            ci: 0,
            error: false,
        };
        lex.scan_char();
        lex
    }

    pub fn next_token(&mut self) -> Token<'source> {
        self.scan_whitespace();
        if self.is_at_end() {
            return Token {
                kind: TokenKind::EOF,
                pos: TokenPos {
                    start: self.ci,
                    end: self.ci,
                },
            };
        }

        let start = self.ci;

        let kind = match self.c {
            ',' => TokenKind::Comma,
            ':' => match self.iter.peek() {
                Some((_, '=')) => {
                    self.scan_char();
                    TokenKind::ColonEquals
                }
                _ => TokenKind::Colon,
            },
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftSquare,
            ']' => TokenKind::RightSquare,
            '{' => TokenKind::LeftCurly,
            '}' => TokenKind::RightCurly,
            '.' => TokenKind::Period,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Times,
            '/' => TokenKind::Divide,
            '=' => TokenKind::Equals,
            '<' => match self.iter.peek() {
                Some((_, '=')) => {
                    self.scan_char();
                    TokenKind::LE
                }
                Some((_, '>')) => {
                    self.scan_char();
                    TokenKind::NotEquals
                }
                _ => TokenKind::LT,
            },
            '>' => match self.iter.peek() {
                Some((_, '=')) => {
                    self.scan_char();
                    TokenKind::GE
                }
                _ => TokenKind::GT,
            },
            '&' => TokenKind::Ampsersand,
            '|' => TokenKind::Pipe,
            '"' => {
                return self.scan_quote();
            }
            // this is a hack...
            _ => TokenKind::Error,
        };

        if kind != TokenKind::Error {
            self.scan_char();
            Token {
                kind,
                pos: TokenPos {
                    start,
                    end: self.ci,
                },
            }
        } else if self.c.is_alphabetic() {
            self.scan_identifier_or_keyword()
        } else if self.c.is_digit(10) {
            self.scan_number()
        } else {
            self.error_token()
        }
    }

    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }

    fn scan_char(&mut self) {
        if let Some((idx, c)) = self.iter.next() {
            self.ci = idx;
            self.c = c;
        } else {
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }

    fn scan_whitespace(&mut self) {
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char();
        }
    }

    fn scan_quote(&mut self) -> Token<'source> {
        let start = self.ci;
        let mut prev = self.c;
        self.scan_char(); // eat first quote
        loop {
            if self.is_at_end() || (self.c == '"' && prev != '\\') {
                break;
            }
            prev = self.c;
            self.scan_char();
        }
        if self.c != '"' {
            self.error_token()
        } else {
            self.scan_char();
            let end = self.ci;
            Token {
                kind: TokenKind::String(&self.input[(start + 1)..(end - 1)]),
                pos: TokenPos { start, end },
            }
        }
    }

    // todo: comments

    fn scan_identifier_or_keyword(&mut self) -> Token<'source> {
        let start = self.ci;
        while self.c.is_alphanumeric() || self.c == '_' {
            self.scan_char();
        }
        let end = self.ci;
        let text = &self.input[start..end];

        let kind = match text {
            "array" => TokenKind::Array,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "to" => TokenKind::To,
            "do" => TokenKind::Do,
            "let" => TokenKind::Let,
            "in" => TokenKind::In,
            "end" => TokenKind::End,
            "of" => TokenKind::Of,
            "break" => TokenKind::Break,
            "nil" => TokenKind::Nil,
            "function" => TokenKind::Function,
            "var" => TokenKind::Var,
            "type" => TokenKind::Type,
            "import" => TokenKind::Import,
            "primitive" => TokenKind::Primitive,
            _ => TokenKind::Id(text),
        };

        Token {
            kind,
            pos: TokenPos { start, end },
        }
    }

    fn scan_number(&mut self) -> Token<'source> {
        let start = self.ci;
        while self.c.is_numeric() {
            self.scan_char();
        }
        let end = self.ci;
        let num = self.input[start..end].parse::<i32>().unwrap();
        Token {
            kind: TokenKind::Num(num),
            pos: TokenPos { start, end },
        }
    }

    fn error_token(&mut self) -> Token<'source> {
        self.error = true;
        Token {
            kind: TokenKind::Error,
            pos: TokenPos {
                start: self.ci,
                end: self.ci + 1,
            },
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }
        let tok = self.next_token();
        if tok.kind == TokenKind::EOF {
            None
        } else {
            Some(tok)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tok {
        ($tok:expr, $kind:expr) => {
            let tok = $tok;
            assert_eq!(
                tok,
                Token {
                    kind: $kind,
                    pos: tok.pos,
                }
            );
        };
        ($tok:expr, $kind:expr, $start:expr) => {
            assert_eq!(
                $tok,
                Token {
                    kind: $kind,
                    pos: TokenPos {
                        start: $start,
                        end: ($start + 1)
                    },
                }
            );
        };
        ($tok:expr, $kind:expr, $start:expr, $end:expr) => {
            assert_eq!(
                $tok,
                Token {
                    kind: $kind,
                    pos: TokenPos {
                        start: $start,
                        end: $end
                    },
                }
            );
        };
    }

    #[test]
    fn next_token_with_error() {
        let mut lex = Lexer::new("foo ~ bar");
        assert_tok!(lex.next_token(), TokenKind::Id("foo"), 0, 3);
        assert_tok!(lex.next_token(), TokenKind::Error, 4, 5);
        assert_tok!(lex.next_token(), TokenKind::Error, 4, 5);
    }

    #[test]
    fn next_token_with_eof() {
        let mut lex = Lexer::new("foo");
        assert_tok!(lex.next_token(), TokenKind::Id("foo"), 0, 3);
        assert_tok!(lex.next_token(), TokenKind::EOF, 3, 3);
        assert_tok!(lex.next_token(), TokenKind::EOF, 3, 3);
    }

    #[test]
    fn iterator() {
        let lex = Lexer::new("hello[]()there");

        let toks: Vec<Token> = lex.collect();
        assert_eq!(toks.len(), 6);
        assert_tok!(toks[0], TokenKind::Id("hello"), 0, 5);
        assert_tok!(toks[1], TokenKind::LeftSquare, 5);
        assert_tok!(toks[2], TokenKind::RightSquare, 6);
        assert_tok!(toks[3], TokenKind::LeftParen, 7);
        assert_tok!(toks[4], TokenKind::RightParen, 8);
        assert_tok!(toks[5], TokenKind::Id("there"), 9, 14);
    }

    #[test]
    fn identifiers() {
        let mut lex = Lexer::new("foo_bar frob");
        assert_tok!(lex.next_token(), TokenKind::Id("foo_bar"));
        assert_tok!(lex.next_token(), TokenKind::Id("frob"));
    }

    #[test]
    fn colon_equals() {
        let mut lex = Lexer::new(": = :=:");
        assert_tok!(lex.next_token(), TokenKind::Colon, 0);
        assert_tok!(lex.next_token(), TokenKind::Equals, 2);
        assert_tok!(lex.next_token(), TokenKind::ColonEquals, 4, 6);
        // todo
    }

    #[test]
    fn comparisons() {
        let mut lex = Lexer::new("< <= > >= = <>");
        assert_tok!(lex.next_token(), TokenKind::LT);
        assert_tok!(lex.next_token(), TokenKind::LE);
        assert_tok!(lex.next_token(), TokenKind::GT);
        assert_tok!(lex.next_token(), TokenKind::GE);
        assert_tok!(lex.next_token(), TokenKind::Equals);
        assert_tok!(lex.next_token(), TokenKind::NotEquals);
    }
}
