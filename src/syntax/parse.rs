use crate::syntax::{ast::{ParsedField, ParsedStruct}, token::{Delimiter, ParseError, TokenIter, TokenTree}};

impl ParsedStruct {
    fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let name = token_iter.expect_ident(None)?;
        let body = token_iter.expect_group(None)?;
        let fields: Vec<ParsedField> = match body.0 {
            Delimiter::Brace => {
                let mut inner_iter = TokenIter::new(body.1);
                let mut f = Vec::<ParsedField>::new();
                let mut ty = Vec::<TokenTree>::new();
                while inner_iter.peek().is_some() {
                    let name = inner_iter.expect_ident(None)?;
                    inner_iter.expect_punct(Some(':'))?;
                    
                    while inner_iter.peek().is_some() {
                        if inner_iter.peek().unwrap() == &TokenTree::Punct(',') {
                            f.push(ParsedField { name: Some(name.to_owned()), ty: ty.to_vec() });
                            ty.clear();
                            inner_iter.next();
                            break;
                        }
                        else {
                            ty.push(inner_iter.peek().unwrap().to_owned());
                            inner_iter.next();
                        }
                    }
                    
                    if !ty.is_empty() {
                        f.push(ParsedField { name: Some(name.to_owned()), ty: ty.to_vec() });
                    }
                }
                
                f
            }
            Delimiter::Paren => {
                let mut inner_iter = TokenIter::new(body.1);
                let mut f = Vec::<ParsedField>::new();
                let mut ty = Vec::<TokenTree>::new();
                while inner_iter.peek().is_some() {
                    if inner_iter.peek().unwrap() == &TokenTree::Punct(',') {
                        f.push(ParsedField { name: None, ty: ty.to_vec() });
                        ty.clear();
                    }
                    else {
                        ty.push(inner_iter.peek().unwrap().to_owned());
                    }
                    inner_iter.next();
                }
                if !ty.is_empty() {
                    f.push(ParsedField { name: None, ty });
                }
                
                token_iter.expect_punct(Some(';'))?;
                    
                f
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket))
        };
        
        Ok(ParsedStruct { name, fields })
    }
}