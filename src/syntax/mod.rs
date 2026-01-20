use std::clone;

// #[cfg(feature = "syntax-macros")]
mod macros;

pub enum TokenTree {
    Ident(String),
    Punct(char),
    Literal(String),
    Group(Delimiter, Vec<TokenTree>)
}

#[derive(Clone)]
pub enum Delimiter {
    Paren,
    Bracket,
    Brace
}

pub struct TokenIter {
    tokens: Vec<TokenTree>,
    pos: usize
}

impl TokenIter {
    fn next(&mut self) -> Option<&TokenTree> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let tok = Some(&self.tokens[self.pos]);
        self.pos += 1;
        
        tok
    }
    
    fn peek(&self) -> Option<&TokenTree> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.pos])
    }
    
    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident) => Ok(ident.to_owned()),
                TokenTree::Punct(c) => Err(ParseError::UnexpectedPunct(c.to_owned())),
                TokenTree::Literal(lit) => Err(ParseError::UnexpectedLiteral(lit.to_owned())),
                TokenTree::Group(del, _) => Err(ParseError::UnexpectedGroup(del.clone()))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    fn expect_punct(&mut self, c: char) -> Result<char, ParseError> {
        
    }
    
    fn expect_literal(&mut self) -> Result<String, ParseError> {
        
    }
    
    fn expect_group(&mut self) -> Result<TokenTree, ParseError> {
        
    }
}

enum ParseError {
    UnexpectedIdent(String),
    UnexpectedPunct(char),
    UnexpectedLiteral(String),
    UnexpectedGroup(Delimiter),
    UnexpectedEOT // End of Tree
}