#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTree {
    Ident(String, Span),
    Punct(char, Spacing, Span),
    Literal(String, Span),
    Group(Delimiter, Vec<TokenTree>, Span)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    Alone,
    Joint
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub pos: usize,
    pub id: usize
}

pub struct TokenIter {
    tokens: Vec<TokenTree>,
    pos: usize
}

impl TokenIter {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }
    
    pub fn pos(&self) -> usize {
        self.pos
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
    
    pub fn expect_ident(&mut self, s: Option<String>) -> Result<(String, Span), ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident, span) => {
                    match s {
                        Some(s) => {
                            if *ident == s {
                                Ok((s, *span))
                            }
                            else {
                                Err(ParseError::IncorrectIdent(ident.to_owned(), *span))
                            }
                        }
                        None => Ok((ident.to_owned(), *span))
                    }
                }
                TokenTree::Punct(c, _, span) => Err(ParseError::UnexpectedPunct(c.to_owned(), *span)),
                TokenTree::Literal(lit, span) => Err(ParseError::UnexpectedLiteral(lit.to_owned(), *span)),
                TokenTree::Group(del, _, span) => Err(ParseError::UnexpectedGroup(*del, *span))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_punct(&mut self, c: Option<char>) -> Result<(char, Span), ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident, span) => Err(ParseError::UnexpectedIdent(ident.to_owned(), *span)),
                TokenTree::Punct(punct, _, span) => {
                    match c {
                        Some(c) => {
                            if *punct == c {
                                Ok((c, *span))
                            }
                            else {
                                Err(ParseError::IncorrectPunct(punct.to_owned(), *span))
                            }
                        }
                        None => Ok((punct.to_owned(), *span))
                    }
                }
                TokenTree::Literal(lit, span) => Err(ParseError::UnexpectedLiteral(lit.to_owned(), *span)),
                TokenTree::Group(del, _, span) => Err(ParseError::UnexpectedGroup(*del, *span))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_literal(&mut self, s: Option<String>) -> Result<(String, Span), ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident, span) => Err(ParseError::UnexpectedIdent(ident.to_owned(), *span)),
                TokenTree::Punct(c, _, span) => Err(ParseError::UnexpectedPunct(c.to_owned(), *span)),
                TokenTree::Literal(lit, span) => {
                    match s {
                        Some(s) => {
                            if *lit == s {
                                Ok((s, *span))
                            }
                            else {
                                Err(ParseError::IncorrectLiteral(lit.to_owned(), *span))
                            }
                        }
                        None => Ok((lit.to_owned(), *span))
                    }
                }
                TokenTree::Group(del, _, span) => Err(ParseError::UnexpectedGroup(*del, *span))
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
    
    pub fn expect_group(&mut self, g: Option<(Delimiter, Vec<TokenTree>, Span)>) -> Result<(Delimiter, Vec<TokenTree>, Span), ParseError> {
        match self.next() {
            Some(tok) => match tok {
                TokenTree::Ident(ident, span) => Err(ParseError::UnexpectedIdent(ident.to_owned(), *span)),
                TokenTree::Punct(c, _, span) => Err(ParseError::UnexpectedPunct(c.to_owned(), *span)),
                TokenTree::Literal(lit, span) => Err(ParseError::UnexpectedLiteral(lit.to_owned(), *span)),
                TokenTree::Group(del, tree, span) => {
                    match g {
                        Some(g) => {
                            if *del == g.0 && tree.as_slice() == g.1.as_slice() {
                                Ok(g)
                            }
                            else {
                                Err(ParseError::IncorrectGroup(*del, tree.to_vec(), *span))
                            }
                        }
                        None => Ok((*del, tree.to_vec(), *span))
                    }
                }
            }
            None => Err(ParseError::UnexpectedEOT)
        }
    }
}

pub enum ParseError {
    UnexpectedIdent(String, Span),
    UnexpectedPunct(char, Span),
    UnexpectedLiteral(String, Span),
    UnexpectedGroup(Delimiter, Span),
    UnexpectedEOT, // End of Tree
    IncorrectIdent(String, Span),
    IncorrectPunct(char, Span),
    IncorrectLiteral(String, Span),
    IncorrectGroup(Delimiter, Vec<TokenTree>, Span),
    IncorrectDelimiter(Delimiter, Span),
    UnmatchedAngleBracket(Span)
}