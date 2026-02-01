use std::collections::HashMap;

use crate::{ast::{ParsedAttribute, ParsedEnum, ParsedField, ParsedStruct, ParsedVariant, VariantData}, token::{Delimiter, ParseError, Spacing, Span, TokenIter, TokenTree}};

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
                    else if matches!(outer_iter.peek().unwrap(), &TokenTree::Punct(',', Spacing::Alone, _)) {
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
                    
                    if let Some(_) = outer_iter.peek() {
                        outer_iter.expect_punct(Some(','))?;
                    }
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
                let mut attrs: Vec<ParsedAttribute> = Vec::new();
                while matches!(token_iter.peek(), Some(TokenTree::Punct('#', _, _))) {
                    token_iter.next();
                    attrs.push(ParsedAttribute::parse(token_iter)?)
                }
                if let Some(tok) = token_iter.peek() {
                    match tok {
                        TokenTree::Ident(s, _) if s == "pub" => {
                            let _ = token_iter.next();
                        }
                        _ => ()
                    }
                }
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                let (name, span) = token_iter.expect_ident(None)?;
                token_iter.expect_punct(Some(':'))?;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone, _) => {
                            depth += 1;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        },
                        &TokenTree::Punct('>', _, span) => {
                            depth = depth.checked_sub(1).ok_or(ParseError::UnmatchedAngleBracket(span))?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        },
                        &TokenTree::Punct(',', _, _) => {
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
                
                Ok(ParsedField { name: Some(name.to_owned()), attrs, ty, span })
            }
            Delimiter::Parenthesis => {
                let mut attrs: Vec<ParsedAttribute> = Vec::new();
                while matches!(token_iter.peek(), Some(TokenTree::Punct('#', _, _))) {
                    token_iter.next();
                    attrs.push(ParsedAttribute::parse(token_iter)?)
                }
                let mut ty = Vec::<TokenTree>::new();
                let mut depth: usize = 0;
                while token_iter.peek().is_some() {
                    match token_iter.peek().unwrap() {
                        &TokenTree::Punct('<', Spacing::Alone, _) => {
                            depth += 1;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        }
                        &TokenTree::Punct('>', _, span) => {
                            depth = depth.checked_sub(1).ok_or(ParseError::UnmatchedAngleBracket(span))?;
                            ty.push(token_iter.peek().unwrap().to_owned());
                        }
                        &TokenTree::Punct(',', _, _) => {
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
                
                Ok(ParsedField { name: None, attrs, ty, span: Span::default()})
            }
            Delimiter::Bracket => return Err(ParseError::IncorrectDelimiter(Delimiter::Bracket, Span::default())),
            Delimiter::None => return Err(ParseError::IncorrectDelimiter(Delimiter::None, Span::default()))
        }
    }
}

impl ParsedAttribute {
    pub fn parse(token_iter: &mut TokenIter) -> Result<Self, ParseError> {
        let (delimiter, body, span) = token_iter.expect_group(None)?;
        if delimiter != Delimiter::Bracket {
            return Err(ParseError::IncorrectDelimiter(delimiter, span));
        }
        
        let mut attr = TokenIter::new(body);
        let (name, span) = attr.expect_ident(None)?;
        let (delimiter, args_tokens, _) = attr.expect_group(None)?;
        if delimiter != Delimiter::Parenthesis {
            return Err(ParseError::IncorrectDelimiter(delimiter, span));
        }
        
        let mut args = HashMap::<String, Vec<TokenTree>>::new();
        let mut args_iter = TokenIter::new(args_tokens);
        while args_iter.peek().is_some() {
            let (key, _) = args_iter.expect_ident(None)?;
            args_iter.expect_punct(Some('='))?;
            let mut depth: usize = 0;
            let mut val: Vec<TokenTree> = Vec::new();
            while args_iter.peek().is_some() {
                match args_iter.peek().unwrap() {
                    TokenTree::Punct('<', Spacing::Alone, _) => {
                        depth += 1;
                        val.push(args_iter.peek().unwrap().to_owned());
                    }
                    TokenTree::Punct('>', _, span) => {
                        depth = depth.checked_sub(1).ok_or(ParseError::UnmatchedAngleBracket(*span))?;
                        val.push(args_iter.peek().unwrap().to_owned());
                    }
                    TokenTree::Punct(',', _, _) => {
                        if depth == 0 {
                            args_iter.next();
                            break;
                        }
                        else {
                            val.push(args_iter.peek().unwrap().to_owned());
                        }
                    }
                    _ => val.push(args_iter.peek().unwrap().to_owned()),
                }
                
                args_iter.next();
            }
            
            args.insert(key.to_owned(), val.to_owned());
        }
        
        Ok(ParsedAttribute { name: name, args, span })
    }
}