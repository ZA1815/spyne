
use crate::{ast::{ParsedEnum, ParsedField, ParsedStruct, ParsedVariant, VariantData}, token::{Delimiter, ParseError, Spacing, TokenIter, TokenTree}};

impl ParsedStruct {
    fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let name = token_iter.expect_ident(None)?;
        let body = token_iter.expect_group(None)?;
        let fields: Vec<ParsedField> = match body.0 {
            Delimiter::Brace => {
                let mut inner_iter = TokenIter::new(body.1);
                let mut f = Vec::<ParsedField>::new();
                while inner_iter.peek().is_some() {
                   let field = ParsedField::parse(&mut inner_iter, Delimiter::Brace)?;
                   f.push(field);
                }
                
                f
            }
            Delimiter::Parenthesis => {
                let mut inner_iter = TokenIter::new(body.1);
                let mut f = Vec::<ParsedField>::new();
                while inner_iter.peek().is_some() {
                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Parenthesis)?;
                    f.push(field);
                }
                
                token_iter.expect_punct(Some(';'))?;
                    
                f
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket)),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None))
        };
        
        Ok(ParsedStruct { name, fields })
    }
}

impl ParsedEnum {
    fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let name = token_iter.expect_ident(None)?;
        let body = token_iter.expect_group(None)?;
        let variants: Vec<ParsedVariant> = match body.0 {
            Delimiter::Brace => {
                let mut outer_iter = TokenIter::new(body.1);
                let mut v = Vec::<ParsedVariant>::new();
                let mut index: u32 = 0;
                while outer_iter.peek().is_some() {
                    let name = outer_iter.expect_ident(None)?;
                    if outer_iter.peek().is_none() {
                        v.push(ParsedVariant { name: name.to_owned(), index, data: VariantData::Unit });
                    }
                    else if outer_iter.peek().unwrap() == &TokenTree::Punct(',', Spacing::Alone) {
                        v.push(ParsedVariant { name, index, data: VariantData::Unit });
                    }
                    else {
                        let var_body = outer_iter.expect_group(None)?;
                        let data: VariantData = match var_body.0 {
                            Delimiter::Parenthesis => {
                                let mut inner_iter = TokenIter::new(var_body.1);
                                let mut f = Vec::<ParsedField>::new();
                                while inner_iter.peek().is_some() {
                                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Parenthesis)?;
                                    f.push(field);
                                }
                                
                                VariantData::Tuple(f)
                            }
                            Delimiter::Brace => {
                                let mut inner_iter = TokenIter::new(var_body.1);
                                let mut f = Vec::<ParsedField>::new();
                                while inner_iter.peek().is_some() {
                                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Brace)?;
                                    f.push(field);
                                }
                                
                                VariantData::Struct(f)
                            }
                            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket)),
                            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None))
                        };
                        
                        v.push(ParsedVariant { name, index, data });
                    }
                    
                    outer_iter.expect_punct(Some(','))?;
                    index += 1;
                }
                
                v
            }
            Delimiter::Parenthesis => return Err(ParseError::IncorrectDelimiter(Delimiter::Parenthesis)),
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket)),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None))
        };
        
        Ok(ParsedEnum { name, variants })
    }
}

impl ParsedField {
    fn parse(token_iter: &mut TokenIter, delimiter: Delimiter) -> Result<Self, ParseError> {
        match delimiter {
            Delimiter::Brace => {
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                let name = token_iter.expect_ident(None)?;
                token_iter.expect_punct(Some(':'))?;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone) => { depth += 1; ty.push(token_iter.peek().unwrap().to_owned()); },
                        &TokenTree::Punct('>', Spacing::Alone) => {
                            depth.checked_sub(1).map(|_| ()).ok_or(ParseError::UnmatchedAngleBracket)?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        },
                        &TokenTree::Punct(',', Spacing::Alone) => {
                            if depth == 0 {
                                token_iter.next();
                                break;
                            }
                            else {
                                ty.push(token_iter.peek().unwrap().to_owned());
                            }
                        },
                        _ => ty.push(token_iter.peek().unwrap().to_owned())
                    }
                    
                    token_iter.next();
                }
                
                Ok(ParsedField { name: Some(name.to_owned()), ty })
            }
            Delimiter::Parenthesis => {
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone) => { depth += 1; ty.push(token_iter.peek().unwrap().to_owned()); }
                        &TokenTree::Punct('>', Spacing::Alone) => {
                            depth.checked_sub(1).map(|_| ()).ok_or(ParseError::UnmatchedAngleBracket)?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        }
                        &TokenTree::Punct(',', Spacing::Alone) => {
                            if depth == 0 {
                                token_iter.next();
                                break;
                            }
                            else {
                                ty.push(token_iter.peek().unwrap().to_owned());
                            }
                        },
                        _ => ty.push(token_iter.peek().unwrap().to_owned()),
                    }
                    
                    token_iter.next();
                }
                
                Ok(ParsedField { name: None, ty })
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket)),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None))
        }
    }
}