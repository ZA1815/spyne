#[derive(Clone, PartialEq, Eq)]
pub enum TokenTree {
    Ident(String),
    Punct(char),
    Literal(String),
    Group(Delimiter, Vec<TokenTree>)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    Paren,
    Bracket,
    Brace
}

pub struct TokenIter {
    tokens: Vec<TokenTree>,
    pub pos: usize
}

impl TokenIter {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }
    pub fn next(&mut self) -> Option<&TokenTree> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let tok = Some(&self.tokens[self.pos]);
        self.pos += 1;
        
        tok
    }
    
    pub fn peek(&self) -> Option<&TokenTree> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.pos])
    }
    
    pub fn expect_ident(&mut self, s: Option<String>) -> Result<String, ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident) => {
                    match s {
                        Some(s) => {
                            if *ident == s {
                                Ok(s)
                            }
                            else {
                                Err(ParseError::IncorrectIdent(ident.to_owned()))
                            }
                        }
                        None => Ok(ident.to_owned())
                    }
                }
                TokenTree::Punct(c) => Err(ParseError::UnexpectedPunct(c.to_owned())),
                TokenTree::Literal(lit) => Err(ParseError::UnexpectedLiteral(lit.to_owned())),
                TokenTree::Group(del, _) => Err(ParseError::UnexpectedGroup(*del))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_punct(&mut self, c: Option<char>) -> Result<char, ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident) => Err(ParseError::UnexpectedIdent(ident.to_owned())),
                TokenTree::Punct(punct) => {
                    match c {
                        Some(c) => {
                            if *punct == c {
                                Ok(c)
                            }
                            else {
                                Err(ParseError::IncorrectPunct(punct.to_owned()))
                            }
                        }
                        None => Ok(punct.to_owned())
                    }
                }
                TokenTree::Literal(lit) => Err(ParseError::UnexpectedLiteral(lit.to_owned())),
                TokenTree::Group(del, _) => Err(ParseError::UnexpectedGroup(*del))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_literal(&mut self, s: Option<String>) -> Result<String, ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident) => Err(ParseError::UnexpectedIdent(ident.to_owned())),
                TokenTree::Punct(c) => Err(ParseError::UnexpectedPunct(c.to_owned())),
                TokenTree::Literal(lit) => {
                    match s {
                        Some(s) => {
                            if *lit == s {
                                Ok(s)
                            }
                            else {
                                Err(ParseError::IncorrectLiteral(lit.to_owned()))
                            }
                        }
                        None => Ok(lit.to_owned())
                    }
                }
                TokenTree::Group(del, _) => Err(ParseError::UnexpectedGroup(*del))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_group(&mut self, g: Option<(Delimiter, Vec<TokenTree>)>) -> Result<(Delimiter, Vec<TokenTree>), ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident) => Err(ParseError::UnexpectedIdent(ident.to_owned())),
                TokenTree::Punct(c) => Err(ParseError::UnexpectedPunct(c.to_owned())),
                TokenTree::Literal(lit) => Err(ParseError::UnexpectedLiteral(lit.to_owned())),
                TokenTree::Group(del, tree) => {
                    match g {
                        Some(g) => {
                            if *del == g.0 && tree.as_slice() == g.1.as_slice() {
                                Ok(g)
                            }
                            else {
                                Err(ParseError::IncorrectGroup(*del, tree.to_vec()))
                            }
                        }
                        None => Ok((*del, tree.to_vec()))
                    }
                }
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
}

pub enum ParseError {
    UnexpectedIdent(String),
    UnexpectedPunct(char),
    UnexpectedLiteral(String),
    UnexpectedGroup(Delimiter),
    UnexpectedEOT, // End of Tree
    IncorrectIdent(String),
    IncorrectPunct(char),
    IncorrectLiteral(String),
    IncorrectGroup(Delimiter, Vec<TokenTree>),
    IncorrectDelimiter(Delimiter)
}