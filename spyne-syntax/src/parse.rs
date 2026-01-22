use crate::{ast::{ParsedEnum, ParsedField, ParsedStruct, ParsedVariant, VariantData}, token::{Delimiter, ParseError, Spacing, Span, TokenIter, TokenTree}};

impl ParsedStruct {
    pub fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let (name, name_span) = token_iter.expect_ident(None)?;
        let (delimiter, fields, body_span) = token_iter.expect_group(None)?;
        let fields: Vec<ParsedField> = match delimiter {
            Delimiter::Brace => {
                let mut inner_iter = TokenIter::new(fields);
                let mut f = Vec::<ParsedField>::new();
                while inner_iter.peek().is_some() {
                   let field = ParsedField::parse(&mut inner_iter, Delimiter::Brace)?;
                   f.push(field);
                }
                
                f
            }
            Delimiter::Parenthesis => {
                let mut inner_iter = TokenIter::new(fields);
                let mut f = Vec::<ParsedField>::new();
                while inner_iter.peek().is_some() {
                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Parenthesis)?;
                    f.push(field);
                }
                
                token_iter.expect_punct(Some(';'))?;
                    
                f
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket, body_span)),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None, body_span))
        };
        
        Ok(ParsedStruct { name, fields, span: name_span })
    }
}

impl ParsedEnum {
    pub fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let (name, name_span) = token_iter.expect_ident(None)?;
        let (delimiter, variants, body_span)= token_iter.expect_group(None)?;
        let variants: Vec<ParsedVariant> = match delimiter {
            Delimiter::Brace => {
                let mut outer_iter = TokenIter::new(variants);
                let mut v = Vec::<ParsedVariant>::new();
                let mut index: u32 = 0;
                while outer_iter.peek().is_some() {
                    let (name, span) = outer_iter.expect_ident(None)?;
                    if outer_iter.peek().is_none() {
                        v.push(ParsedVariant { name: name.to_owned(), index, data: VariantData::Unit(span), span });
                    }
                    else if outer_iter.peek().unwrap() == &TokenTree::Punct(',', Spacing::Alone, span) {
                        v.push(ParsedVariant { name, index, data: VariantData::Unit(span), span });
                    }
                    else {
                        let (delimiter, fields, span) = outer_iter.expect_group(None)?;
                        let data: VariantData = match delimiter {
                            Delimiter::Parenthesis => {
                                let mut inner_iter = TokenIter::new(fields);
                                let mut f = Vec::<ParsedField>::new();
                                while inner_iter.peek().is_some() {
                                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Parenthesis)?;
                                    f.push(field);
                                }
                                
                                VariantData::Tuple(f, span)
                            }
                            Delimiter::Brace => {
                                let mut inner_iter = TokenIter::new(fields);
                                let mut f = Vec::<ParsedField>::new();
                                while inner_iter.peek().is_some() {
                                    let field = ParsedField::parse(&mut inner_iter, Delimiter::Brace)?;
                                    f.push(field);
                                }
                                
                                VariantData::Struct(f, span)
                            }
                            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket, span)),
                            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None, span))
                        };
                        
                        v.push(ParsedVariant { name, index, data, span });
                    }
                    
                    outer_iter.expect_punct(Some(','))?;
                    index += 1;
                }
                
                v
            }
            Delimiter::Parenthesis => return Err(ParseError::IncorrectDelimiter(Delimiter::Parenthesis, body_span)),
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket, body_span)),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None, body_span))
        };
        
        Ok(ParsedEnum { name, variants, span: name_span })
    }
}

impl ParsedField {
    pub fn parse(token_iter: &mut TokenIter, delimiter: Delimiter) -> Result<Self, ParseError> {
        match delimiter {
            Delimiter::Brace => {
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                let (name, span) = token_iter.expect_ident(None)?;
                token_iter.expect_punct(Some(':'))?;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone, _) => { depth += 1; ty.push(token_iter.peek().unwrap().to_owned()); },
                        &TokenTree::Punct('>', Spacing::Alone, span) => {
                            depth.checked_sub(1).map(|_| ()).ok_or(ParseError::UnmatchedAngleBracket(span))?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        },
                        &TokenTree::Punct(',', Spacing::Alone, _) => {
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
                
                Ok(ParsedField { name: Some(name.to_owned()), ty, span })
            }
            Delimiter::Parenthesis => {
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone, _) => { depth += 1; ty.push(token_iter.peek().unwrap().to_owned()); }
                        &TokenTree::Punct('>', Spacing::Alone, span) => {
                            depth.checked_sub(1).map(|_| ()).ok_or(ParseError::UnmatchedAngleBracket(span))?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        }
                        &TokenTree::Punct(',', Spacing::Alone, _) => {
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
                
                Ok(ParsedField { name: None, ty, span: Span::default()})
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket, Span::default())),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None, Span::default()))
        }
    }
}