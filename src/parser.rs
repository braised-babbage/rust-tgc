use std::iter::Peekable;

use crate::lexer::{Lexer, Token, TokenKind, TokenPos};
use crate::ast::{Expr, Op};


/*
 * S -> E / 
 *
 *
 */

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    UnexpectedToken(&'static str, TokenKind<'a>),
}

type ParseResult<'a, T> = Result<T, ParseError<'a>>;

fn unexpected_token<'a>(expected: &'static str, actual: TokenKind<'a>) -> ParseResult<'a, Expr<'a>> {
    Err(ParseError::UnexpectedToken(expected, actual))
}


impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
	Parser {
	    tokens,
	    pos: 0,
	}
    }

    fn is_eof(&self) -> bool {
	self.pos >= self.tokens.len()
    }

    fn peek(&self) -> TokenKind<'a> {
	println!("{}", self.pos);
	self.tokens[self.pos].kind
    }

    fn tok_pos(&self) -> TokenPos {
	self.tokens[self.pos].pos
    }

    fn is_match(&self, kind: TokenKind) -> bool {
	!self.is_eof() && self.peek() == kind
    }

    fn advance(&mut self) {
	self.pos += 1;
    }
    
    pub fn parse(&mut self) -> ParseResult<'a, Expr> {
	self.t()
    }

    fn t(&mut self) -> ParseResult<'a, Expr<'a>> {
	let kind = self.peek();
	match kind {
	    TokenKind::Num(_) => {
		let left = self.f()?;
		self.t_rest(left)
	    },
	    TokenKind::LeftParen => {
		let left = self.f()?;
		self.t_rest(left)
	    },
	    _ => unexpected_token("an arithmetic expression", kind),
	}
    }

    fn t_rest(&mut self, left: Expr<'a>) -> ParseResult<'a, Expr<'a>> {
	if self.is_eof() {
	    return Ok(left)
	};
	
	let kind = self.peek();
	match kind {
	    TokenKind::Times => {
		let pos = self.tok_pos();
		self.advance();
		let right = self.f()?;
		let expr = Expr::BinOp {
		    left: Box::new(left),
		    oper: Op::Times,
		    right: Box::new(right),
		    pos,
		};
		self.t_rest(expr)
	    },
	    _ => Ok(left),
	}
    }

    fn f(&mut self) -> ParseResult<'a, Expr<'a>> {
	let kind = self.peek();
	let expr = match kind {
	    TokenKind::Num(x) => Ok(Expr::Int(x)),
	    _ => unexpected_token("a number", kind),
	}?;
	self.advance();
	Ok(expr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn products() {
	let toks = Lexer::new("3*4").collect();
	let mut parser = Parser::new(toks);
	let result = parser.parse();
	println!("{:?}", result);
	assert!(match result {
	    Ok(Expr::BinOp { left, oper, right, pos }) => true,
	    _ => false,
	});
    }
}
