use proc_macro::{Group, Ident, Punct, TokenStream};
use spyne_syntax::token::{Delimiter, Spacing, Span, TokenTree};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::iter::Peekable;

thread_local! {
    pub static SPAN_MAP: RefCell<HashMap<usize, proc_macro::Span>> = RefCell::new(HashMap::new());
    pub static SPAN_COUNTER: Cell<usize> = Cell::new(1);
}

pub fn quote_help(template: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut vec: Vec<TokenTree> = Vec::new();
    let vec_id = save_span(proc_macro::Span::mixed_site());
    vec.push(TokenTree::Ident(format!("use"), Span::default()));
    vec.push(TokenTree::Ident(format!("spyne_syntax"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("token"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("Spacing"), Span::default()));
    vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("use"), Span::default()));
    vec.push(TokenTree::Ident(format!("spyne_syntax"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("token"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("Delimiter"), Span::default()));
    vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("use"), Span::default()));
    vec.push(TokenTree::Ident(format!("spyne_syntax"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("tok_gen"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("ToTokens"), Span::default()));
    vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("let"), Span::default()));
    vec.push(TokenTree::Ident(format!("mut"), Span::default()));
    vec.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
    vec.push(TokenTree::Punct(':', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("Vec"), Span::default()));
    vec.push(TokenTree::Punct('<', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
    vec.push(TokenTree::Punct('>', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
    vec.push(TokenTree::Ident(format!("Vec"), Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    vec.push(TokenTree::Ident(format!("new"), Span::default()));
    vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
    vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));

    let mut template_iter = template.into_iter().peekable();
    quote_stream(&mut template_iter, &mut vec, vec_id);
    
    vec.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
    let wrap = TokenTree::Group(Delimiter::Brace, vec, Span::default());
    
    vec![wrap]
}

fn quote_stream(iter: &mut Peekable<impl Iterator<Item = TokenTree>>, stream: &mut Vec<TokenTree>, stream_id: usize) {
    while let Some(tok) = iter.next() {
        match tok {
            TokenTree::Group(Delimiter::Parenthesis, t, _) if matches!(t.first(), Some(TokenTree::Punct('$', _, _))) => {
                quote_repetition(iter, t, stream, stream_id);
            }
            TokenTree::Group(Delimiter::Bracket, t, _) if matches!(t.first(), Some(TokenTree::Punct('$', _, _))) => {
                quote_interpolation(t, stream, stream_id);
            }
            TokenTree::Group(d, t, s) => {
                stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }));
                stream.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                stream.push(TokenTree::Ident(format!("push"), Span::default()));
                stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                    let mut items: Vec<TokenTree> = Vec::new();
                    items.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
                    items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                    items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                    items.push(TokenTree::Ident(format!("Group"), Span::default()));
                    items.push(TokenTree::Group(Delimiter::Parenthesis, {
                        let mut inner_stream: Vec<TokenTree> = Vec::new();
                        inner_stream.push(TokenTree::Ident(format!("Delimiter"), Span::default()));
                        inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                        inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                        match d {
                            Delimiter::Parenthesis => inner_stream.push(TokenTree::Ident(format!("Parenthesis"), Span::default())),
                            Delimiter::Brace => inner_stream.push(TokenTree::Ident(format!("Brace"), Span::default())),
                            Delimiter::Bracket => inner_stream.push(TokenTree::Ident(format!("Bracket"), Span::default())),
                            Delimiter::None => inner_stream.push(TokenTree::Ident(format!("None"), Span::default())),
                        }
                        inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                        inner_stream.push(TokenTree::Group(Delimiter::Brace, {
                            let mut inner_stream: Vec<TokenTree> = Vec::new();
                            let vec_id = save_span(proc_macro::Span::mixed_site());
                            inner_stream.push(TokenTree::Ident(format!("let"), Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("mut"), Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
                            inner_stream.push(TokenTree::Punct(':', Spacing::Alone, Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("Vec"), Span::default()));
                            inner_stream.push(TokenTree::Punct('<', Spacing::Alone, Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
                            inner_stream.push(TokenTree::Punct('>', Spacing::Alone, Span::default()));
                            inner_stream.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("Vec"), Span::default()));
                            inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                            inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                            inner_stream.push(TokenTree::Ident(format!("new"), Span::default()));
                            inner_stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                            inner_stream.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
                            
                            let mut inner_iter = t.into_iter().peekable();
                            quote_stream(&mut inner_iter, &mut inner_stream, vec_id);
                            
                            inner_stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
                            
                            inner_stream
                        }, Span::default()));
                        inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                        inner_stream.extend(quote_span(&s));
                        
                        inner_stream
                    }, Span::default()));
                    
                    items
                }, Span::default()));
            }
            _ => {
                stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }));
                stream.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                stream.push(TokenTree::Ident(format!("push"), Span::default()));
                stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                    let mut args: Vec<TokenTree> = Vec::new();
                    quote_token(&tok, &mut args);
                    
                    args
                }, Span::default()));
            }
        }
        stream.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    }
}

fn quote_token(token: &TokenTree, stream: &mut Vec<TokenTree>) {
    stream.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
    stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
    match token {
        TokenTree::Ident(i, s) => {
            stream.push(TokenTree::Ident(format!("Ident"), Span::default()));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Literal(format!("{:?}", i), Span::default()));
                items.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("to_string"), Span::default()));
                items.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.extend(quote_span(s));
                
                items
            }, Span::default()));
        }
        TokenTree::Punct(p, spacing, span) => {
            stream.push(TokenTree::Ident(format!("Punct"), Span::default()));
            stream.push(TokenTree::Group(
                Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Literal(format!("{:?}", p), Span::default()));
                items.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("to_owned"), Span::default()));
                items.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("Spacing"), Span::default()));
                items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                match spacing {
                    Spacing::Alone => {
                        items.push(TokenTree::Ident(format!("Alone"), Span::default()))
                    }
                    Spacing::Joint => {
                        items.push(TokenTree::Ident(format!("Joint"), Span::default()))
                    }
                }
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.extend(quote_span(span));

                items
            }, Span::default()));
        }
        TokenTree::Literal(l, s) => {
            stream.push(TokenTree::Ident(format!("Literal"), Span::default()));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Literal(format!("{:?}", l), Span::default()));
                items.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("to_string"), Span::default()));
                items.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.extend(quote_span(s));

                items
            }, Span::default()));
        }
        TokenTree::Group(d, t, s) => {
            stream.push(TokenTree::Ident(format!("Group"), Span::default()));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Ident(format!("Delimiter"), Span::default()));
                items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                match d {
                    Delimiter::Parenthesis => {
                        items.push(TokenTree::Ident(format!("Parenthesis"), Span::default()))
                    }
                    Delimiter::Brace => {
                        items.push(TokenTree::Ident(format!("Brace"), Span::default()))
                    }
                    Delimiter::Bracket => {
                        items.push(TokenTree::Ident(format!("Bracket"), Span::default()))
                    }
                    Delimiter::None => {
                        items.push(TokenTree::Ident(format!("None"), Span::default()))
                    }
                }
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("vec"), Span::default()));
                items.push(TokenTree::Punct('!', Spacing::Alone, Span::default()));
                let mut inner_stream: Vec<TokenTree> = Vec::new();
                for item in t {
                    quote_token(item, &mut inner_stream);
                    inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                }
                items.push(TokenTree::Group(
                    Delimiter::Bracket,
                    inner_stream,
                    Span::default(),
                ));
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.extend(quote_span(s));

                items
            }, Span::default()));
        }
    }
}

fn quote_span(span: &Span) -> Vec<TokenTree> {
    let vec: Vec<TokenTree> = vec![
        TokenTree::Ident(format!("spyne_syntax"), Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Ident(format!("token"), Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Ident(format!("Span"), Span::default()),
        TokenTree::Group(Delimiter::Brace, vec![
            TokenTree::Ident(format!("line"), Span::default()),
            TokenTree::Punct(':', Spacing::Alone, Span::default()),
            TokenTree::Literal(format!("{}", span.line), Span::default()),
            TokenTree::Punct(',', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("col"), Span::default()),
            TokenTree::Punct(':', Spacing::Alone, Span::default()),
            TokenTree::Literal(format!("{}", span.col), Span::default()),
            TokenTree::Punct(',', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("pos"), Span::default()),
            TokenTree::Punct(':', Spacing::Alone, Span::default()),
            TokenTree::Literal(format!("{}", span.pos), Span::default()),
            TokenTree::Punct(',', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("id"), Span::default()),
            TokenTree::Punct(':', Spacing::Alone, Span::default()),
            TokenTree::Literal(format!("0"), Span::default())
        ], Span::default())
    ];

    vec
}

fn quote_repetition(template_iter: &mut Peekable<impl Iterator<Item = TokenTree>>, iter: Vec<TokenTree>, stream: &mut Vec<TokenTree>, stream_id: usize) {
    let mut separator = None;
    if let Some(tok) = template_iter.peek() {
        if !matches!(tok, TokenTree::Punct('*', _, _)) {
            separator = template_iter.next();
            stream.push(TokenTree::Ident(format!("let"), Span::default()));
            stream.push(TokenTree::Ident(format!("mut"), Span::default()));
            stream.push(TokenTree::Ident(format!("_____first_"), Span::default()));
            stream.push(TokenTree::Punct(':', Spacing::Alone, Span::default()));
            stream.push(TokenTree::Ident(format!("bool"), Span::default()));
            stream.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
            stream.push(TokenTree::Ident(format!("true"), Span::default()));
            stream.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
        }
    }
    template_iter.next(); // Consume * 
    let mut name_iter = iter.clone().into_iter().peekable();
    name_iter.next(); // Consume $
    let mut count: usize = 0;
    let mut names: Vec<Vec<TokenTree>> = Vec::new();
    while let Some(tok) = name_iter.next() {
        match tok {
            TokenTree::Group(Delimiter::Bracket, t, _) if matches!(t.first(), Some(TokenTree::Punct('$', _, _))) => {
                stream.push(TokenTree::Ident(format!("let"), Span::default()));
                stream.push(TokenTree::Ident(format!("mut"), Span::default()));
                stream.push(TokenTree::Ident(format!("_____i_{}", count), Span::default()));
                stream.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
                stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                    let mut name: Vec<TokenTree> = Vec::new();
                    for item in t.into_iter().skip(1) {
                        name.push(item);
                    }
                    names.push(name.to_vec());
                    
                    name
                }, Span::default()));
                stream.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                stream.push(TokenTree::Ident(format!("into_iter"), Span::default()));
                stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                stream.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
                
                count += 1;
            }
            _ => (),
        }
    }
    stream.push(TokenTree::Ident(format!("while"), Span::default()));
    stream.push(TokenTree::Ident(format!("let"), Span::default()));
    stream.push(TokenTree::Group(Delimiter::Parenthesis, {
        let mut left_vec: Vec<TokenTree> = Vec::new();
        for (i, _) in names.iter().enumerate() {
            left_vec.push(TokenTree::Ident(format!("Some"), Span::default()));
            left_vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![TokenTree::Ident(format!("_____binding_{}", i), Span::default())], Span::default()));
            left_vec.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
        }
        
        left_vec
    }, Span::default()));
    stream.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
    stream.push(TokenTree::Group(Delimiter::Parenthesis, {
        let mut right_vec: Vec<TokenTree> = Vec::new();
        for i in 0..count {
            right_vec.push(TokenTree::Ident(format!("_____i_{}", i), Span::default()));
            right_vec.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
            right_vec.push(TokenTree::Ident(format!("next"), Span::default()));
            right_vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
            right_vec.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
        }
        
        right_vec
    }, Span::default()));
    stream.push(TokenTree::Group(Delimiter::Brace, {
        let mut loop_body: Vec<TokenTree> = Vec::new();
        if let Some(sep) = separator {
            loop_body.push(TokenTree::Ident(format!("if"), Span::default()));
            loop_body.push(TokenTree::Punct('!', Spacing::Alone, Span::default()));
            loop_body.push(TokenTree::Ident(format!("_____first_"), Span::default()));
            loop_body.push(TokenTree::Group(Delimiter::Brace, vec![
                TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }),
                TokenTree::Punct('.', Spacing::Alone, Span::default()),
                TokenTree::Ident(format!("push"), Span::default()),
                TokenTree::Group(Delimiter::Parenthesis, {
                    let mut sep_vec: Vec<TokenTree> = Vec::new();
                    quote_token(&sep, &mut sep_vec);
                    
                    sep_vec
                }, Span::default()),
                TokenTree::Punct(';', Spacing::Alone, Span::default())
            ], Span::default()));
            loop_body.push(TokenTree::Ident(format!("_____first_"), Span::default()));
            loop_body.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
            loop_body.push(TokenTree::Ident(format!("false"), Span::default()));
            loop_body.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
        }
        let mut body_iter = iter.into_iter().peekable();
        body_iter.next(); // Consume $
        let mut bind_idx: usize = 0;
        while let Some(tok) = body_iter.next() {
            match tok {
                TokenTree::Group(Delimiter::Parenthesis, t, _) if matches!(t.first(), Some(TokenTree::Punct('$', _, _))) => {
                    quote_repetition(template_iter, t, &mut loop_body, stream_id);
                }
                TokenTree::Group(Delimiter::Bracket, t, _) if matches!(t.first(), Some(TokenTree::Punct('$', _, _))) => {
                    let span = match t[0] {
                        TokenTree::Punct(_, _, s) => s,
                        _ => unreachable!()
                    };
                    loop_body.push(TokenTree::Ident(format!("_____binding_{}", bind_idx), Span::default()));
                    loop_body.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                    loop_body.push(TokenTree::Ident(format!("to_tokens"), span));
                    loop_body.push(TokenTree::Group(Delimiter::Parenthesis, vec![
                        TokenTree::Punct('&', Spacing::Alone, Span::default()),
                        TokenTree::Ident(format!("mut"), Span::default()),
                        TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }),
                    ], Span::default()));
                    
                    loop_body.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
                    bind_idx += 1;
                }
                TokenTree::Group(d, t, s) => {
                    loop_body.push(TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }));
                    loop_body.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                    loop_body.push(TokenTree::Ident(format!("push"), Span::default()));
                    loop_body.push(TokenTree::Group(Delimiter::Parenthesis, {
                        let mut items: Vec<TokenTree> = Vec::new();
                        items.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
                        items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                        items.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                        items.push(TokenTree::Ident(format!("Group"), Span::default()));
                        items.push(TokenTree::Group(Delimiter::Parenthesis, {
                            let mut inner_stream: Vec<TokenTree> = Vec::new();
                            inner_stream.push(TokenTree::Ident(format!("Delimiter"), Span::default()));
                            inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                            inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                            match d {
                                Delimiter::Parenthesis => inner_stream.push(TokenTree::Ident(format!("Parenthesis"), Span::default())),
                                Delimiter::Brace => inner_stream.push(TokenTree::Ident(format!("Brace"), Span::default())),
                                Delimiter::Bracket => inner_stream.push(TokenTree::Ident(format!("Bracket"), Span::default())),
                                Delimiter::None => inner_stream.push(TokenTree::Ident(format!("None"), Span::default())),
                            }
                            inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                            inner_stream.push(TokenTree::Group(Delimiter::Brace, {
                                let mut inner_stream: Vec<TokenTree> = Vec::new();
                                let vec_id = save_span(proc_macro::Span::mixed_site());
                                inner_stream.push(TokenTree::Ident(format!("let"), Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("mut"), Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
                                inner_stream.push(TokenTree::Punct(':', Spacing::Alone, Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("Vec"), Span::default()));
                                inner_stream.push(TokenTree::Punct('<', Spacing::Alone, Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("TokenTree"), Span::default()));
                                inner_stream.push(TokenTree::Punct('>', Spacing::Alone, Span::default()));
                                inner_stream.push(TokenTree::Punct('=', Spacing::Alone, Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("Vec"), Span::default()));
                                inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                                inner_stream.push(TokenTree::Punct(':', Spacing::Joint, Span::default()));
                                inner_stream.push(TokenTree::Ident(format!("new"), Span::default()));
                                inner_stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()));
                                inner_stream.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
                                
                                let mut inner_iter = t.into_iter().peekable();
                                quote_stream(&mut inner_iter, &mut inner_stream, vec_id);
                                
                                inner_stream.push(TokenTree::Ident(format!("_____tokens_"), Span { id: vec_id, ..Default::default() }));
                                
                                inner_stream
                            }, Span::default()));
                            inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                            inner_stream.extend(quote_span(&s));
                            
                            inner_stream
                        }, Span::default()));
                        
                        items
                    }, Span::default()));
                }
                _ => {
                    loop_body.push(TokenTree::Ident(format!("_____tokens_"),Span { id: stream_id, ..Default::default() }));
                    loop_body.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                    loop_body.push(TokenTree::Ident(format!("push"), Span::default()));
                    loop_body.push(TokenTree::Group(Delimiter::Parenthesis, {
                        let mut args: Vec<TokenTree> = Vec::new();
                        quote_token(&tok, &mut args);
                        
                        args
                    }, Span::default()));
                }
            }
            
            loop_body.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
        }
        
        loop_body
    }, Span::default()));
}

fn quote_interpolation(iter: Vec<TokenTree>, stream: &mut Vec<TokenTree>, stream_id: usize) {
    let span = match iter[0] {
        TokenTree::Punct(_, _, s) => s,
        _ => unreachable!(),
    };
    stream.push(TokenTree::Group(
        Delimiter::Parenthesis, {
        let mut items: Vec<TokenTree> = Vec::new();
        for item in iter.iter().skip(1) {
            items.push(item.to_owned());
        }

        items
    }, Span::default()));
    stream.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
    stream.push(TokenTree::Ident(format!("to_tokens"), span));
    stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![
        TokenTree::Punct('&', Spacing::Alone, Span::default()),
        TokenTree::Ident(format!("mut"), Span::default()),
        TokenTree::Ident(format!("_____tokens_"), Span { id: stream_id, ..Default::default() }),
    ], Span::default()));
}

pub fn from_stream(stream: TokenStream) -> Vec<TokenTree> {
    let mut out: Vec<TokenTree> = Vec::new();
    for tok in stream {
        match tok {
            proc_macro::TokenTree::Ident(i) => {
                out.push(TokenTree::Ident(i.to_string(), Span { id: save_span(i.span()), ..Default::default() }));
            }
            proc_macro::TokenTree::Punct(p) => match p.spacing() {
                proc_macro::Spacing::Alone => out.push(TokenTree::Punct(p.as_char(), Spacing::Alone, Span { id: save_span(p.span()), ..Default::default() })),
                proc_macro::Spacing::Joint => out.push(TokenTree::Punct(p.as_char(), Spacing::Joint, Span { id: save_span(p.span()), ..Default::default() })),
            },
            proc_macro::TokenTree::Literal(l) => {
                out.push(TokenTree::Literal(l.to_string(), Span { id: save_span(l.span()), ..Default::default() }));
            }
            proc_macro::TokenTree::Group(g) => match g.delimiter() {
                proc_macro::Delimiter::Parenthesis => out.push(TokenTree::Group(Delimiter::Parenthesis, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                proc_macro::Delimiter::Brace => out.push(TokenTree::Group(Delimiter::Brace, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                proc_macro::Delimiter::Bracket => out.push(TokenTree::Group(Delimiter::Bracket, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                proc_macro::Delimiter::None => out.push(TokenTree::Group(Delimiter::None, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
            },
        }
    }

    out
}

pub fn to_stream(out: Vec<TokenTree>) -> TokenStream {
    let mut stream_vec: Vec<proc_macro::TokenTree> = Vec::new();
    let mut stream = TokenStream::new();
    for tok in out {
        match tok {
            TokenTree::Ident(i, s) => {
                stream_vec.push(proc_macro::TokenTree::Ident(Ident::new(&i, get_span(s.id))))
            }
            TokenTree::Punct(c, spacing, span) => match spacing {
                Spacing::Alone => {
                    let mut punct = Punct::new(c, proc_macro::Spacing::Alone);
                    punct.set_span(get_span(span.id));
                    stream_vec.push(proc_macro::TokenTree::Punct(punct));
                }
                Spacing::Joint => {
                    let mut punct = Punct::new(c, proc_macro::Spacing::Joint);
                    punct.set_span(get_span(span.id));
                    stream_vec.push(proc_macro::TokenTree::Punct(punct))
                }
            },
            TokenTree::Literal(l, s) => {
                let mut literal = proc_macro::TokenTree::Literal(l.parse().unwrap());
                literal.set_span(get_span(s.id));
                stream_vec.push(literal);
            }
            TokenTree::Group(d, t, s) => match d {
                Delimiter::Parenthesis => {
                    let mut group = Group::new(proc_macro::Delimiter::Parenthesis, to_stream(t));
                    group.set_span(get_span(s.id));
                    stream_vec.push(proc_macro::TokenTree::Group(group));
                }
                Delimiter::Brace => {
                    let mut group = Group::new(proc_macro::Delimiter::Brace, to_stream(t));
                    group.set_span(get_span(s.id));
                    stream_vec.push(proc_macro::TokenTree::Group(group));
                }
                Delimiter::Bracket => {
                    let mut group = Group::new(proc_macro::Delimiter::Bracket, to_stream(t));
                    group.set_span(get_span(s.id));
                    stream_vec.push(proc_macro::TokenTree::Group(group));
                }
                Delimiter::None => {
                    let mut group = Group::new(proc_macro::Delimiter::None, to_stream(t));
                    group.set_span(get_span(s.id));
                    stream_vec.push(proc_macro::TokenTree::Group(group));
                }
            },
        }
    }
    stream.extend(stream_vec);

    stream
}

fn save_span(span: proc_macro::Span) -> usize {
    let id = SPAN_COUNTER.with(|state| state.get());
    SPAN_MAP.with(|state| state.borrow_mut().insert(id, span));
    SPAN_COUNTER.with(|state| state.set(state.get() + 1));

    id
}

fn get_span(id: usize) -> proc_macro::Span {
    if id == 0 {
        return proc_macro::Span::call_site();
    }

    SPAN_MAP.with(|state| state.borrow().get(&id).unwrap().clone())
}