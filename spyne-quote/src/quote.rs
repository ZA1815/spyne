use proc_macro::{Ident, Punct, Group, TokenStream};
use spyne_syntax::token::{Delimiter, Spacing, Span, TokenTree};
use std::collections::HashMap;
use std::cell::{Cell, RefCell};

thread_local! {
    pub static SPAN_MAP: RefCell<HashMap<usize, proc_macro::Span>> = RefCell::new(HashMap::new());
    pub static SPAN_COUNTER: Cell<usize> = Cell::new(1);
}

pub fn quote_help(template: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut vec: Vec<TokenTree> = Vec::new();
    vec.push(TokenTree::Ident(format!("let"), Span::default()));
    vec.push(TokenTree::Ident(format!("mut"), Span::default()));
    vec.push(TokenTree::Ident(format!("vec"), Span::default()));
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
    
    for tok in template {
        match tok {
            TokenTree::Group(Delimiter::Bracket, t, _) => {
                for item in t {
                    vec.push(item);
                }
                vec.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                vec.push(TokenTree::Ident(format!("to_tokens"), Span::default()));
                vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Punct('&', Spacing::Alone, Span::default()),
                    TokenTree::Ident(format!("mut"), Span::default()),
                    TokenTree::Ident(format!("vec"), Span::default())
                ], Span::default()));
            }
            _ => {
                vec.push(TokenTree::Ident(format!("vec"), Span::default()));
                vec.push(TokenTree::Punct('.', Spacing::Alone, Span::default()));
                vec.push(TokenTree::Ident(format!("push"), Span::default()));
                vec.push(TokenTree::Group(Delimiter::Parenthesis, {
                    let mut args: Vec<TokenTree> = Vec::new();
                    quote_token(&tok, &mut args);
                    
                    args
                }, Span::default()));
            }
        }
        
        vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    }
    
    vec.push(TokenTree::Ident(format!("return"), Span::default()));
    vec.push(TokenTree::Ident(format!("vec"), Span::default()));
    vec.push(TokenTree::Punct(';', Spacing::Alone, Span::default()));
    
    vec
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
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
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
                    Spacing::Alone => items.push(TokenTree::Ident(format!("Alone"), Span::default())),
                    Spacing::Joint => items.push(TokenTree::Ident(format!("Joint"), Span::default()))
                }
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
                    Delimiter::Parenthesis => items.push(TokenTree::Ident(format!("Parenthesis"), Span::default())),
                    Delimiter::Brace => items.push(TokenTree::Ident(format!("Brace"), Span::default())),
                    Delimiter::Bracket => items.push(TokenTree::Ident(format!("Bracket"), Span::default())),
                    Delimiter::None => items.push(TokenTree::Ident(format!("None"), Span::default()))
                }
                items.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                items.push(TokenTree::Ident(format!("vec"), Span::default()));
                items.push(TokenTree::Punct('!', Spacing::Alone, Span::default()));
                let mut inner_stream: Vec<TokenTree> = Vec::new();
                for item in t {
                    quote_token(item, &mut inner_stream);
                    inner_stream.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
                }
                items.push(TokenTree::Group(Delimiter::Bracket, inner_stream, Span::default()));
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
                TokenTree::Literal(format!("{}", span.pos), Span::default())
        ], Span::default())
    ];
    
    vec
}

pub fn from_stream(stream: TokenStream) -> Vec<TokenTree> {
    let mut out: Vec<TokenTree> = Vec::new();
    for tok in stream {
        match tok {
            proc_macro::TokenTree::Ident(i) => {
                out.push(TokenTree::Ident(i.to_string(), Span { id: save_span(i.span()), ..Default::default() }));
            }
            proc_macro::TokenTree::Punct(p) => {
                match p.spacing() {
                    proc_macro::Spacing::Alone => out.push(TokenTree::Punct(p.as_char(), Spacing::Alone, Span { id: save_span(p.span()), ..Default::default() })),
                    proc_macro::Spacing::Joint => out.push(TokenTree::Punct(p.as_char(), Spacing::Joint, Span { id: save_span(p.span()), ..Default::default() })),
                }
            }
            proc_macro::TokenTree::Literal(l) => {
                out.push(TokenTree::Literal(l.to_string(), Span { id: save_span(l.span()), ..Default::default() }));
            }
            proc_macro::TokenTree::Group(g) => {
                match g.delimiter() {
                    proc_macro::Delimiter::Parenthesis => out.push(TokenTree::Group(Delimiter::Parenthesis, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                    proc_macro::Delimiter::Brace => out.push(TokenTree::Group(Delimiter::Brace, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                    proc_macro::Delimiter::Bracket => out.push(TokenTree::Group(Delimiter::Bracket, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                    proc_macro::Delimiter::None => out.push(TokenTree::Group(Delimiter::None, from_stream(g.stream()), Span { id: save_span(g.span()), ..Default::default() })),
                }
            }
        }
    }
    
    out
}

pub fn to_stream(out: Vec<TokenTree>) -> TokenStream {
    let mut stream_vec: Vec<proc_macro::TokenTree> = Vec::new();
    let mut stream = TokenStream::new();
    for tok in out {
        match tok {
            TokenTree::Ident(i, s) => stream_vec.push(proc_macro::TokenTree::Ident(Ident::new(&i, get_span(s.id)))),
            TokenTree::Punct(c, spacing, span) => {
                match spacing {
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
                }
            },
            TokenTree::Literal(l, s) => {
                let mut literal = proc_macro::TokenTree::Literal(l.parse().unwrap());
                literal.set_span(get_span(s.id));
                stream_vec.push(literal);
            }
            TokenTree::Group(d, t, s) => {
                match d {
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
                }
            }
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

#[cfg(test)]
mod test {
    use spyne_syntax::token::TokenTree;
    use super::*;
    
    #[test]
    fn test_quote() {
        let template: Vec<TokenTree> = vec![
            TokenTree::Ident(format!("impl"), Span::default()),
            TokenTree::Ident(format!("ToTokens"), Span::default()),
            TokenTree::Ident(format!("for"), Span::default()),
            TokenTree::Group(Delimiter::Bracket, vec![
                TokenTree::Ident(format!("self"), Span::default()),
                TokenTree::Punct('.', Spacing::Alone, Span::default()),
                TokenTree::Ident(format!("data"), Span::default())
            ], Span::default()),
            TokenTree::Group(Delimiter::Brace, vec![], Span::default())
        ];
        let out = quote_help(template);
        
        let expected: Vec<TokenTree> = vec![
            TokenTree::Ident(format!("let"), Span::default()),
            TokenTree::Ident(format!("mut"), Span::default()),
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct(':', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("Vec"), Span::default()),
            TokenTree::Punct('<', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("TokenTree"), Span::default()),
            TokenTree::Punct('>', Spacing::Alone, Span::default()),
            TokenTree::Punct('=', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("Vec"), Span::default()),
            TokenTree::Punct(':', Spacing::Joint, Span::default()),
            TokenTree::Punct(':', Spacing::Joint, Span::default()),
            TokenTree::Ident(format!("new"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("push"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree"), Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Ident(format!("Ident"), Span::default()),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"impl\""), Span::default()),
                    TokenTree::Punct('.', Spacing::Alone, Span::default()),
                    TokenTree::Ident(format!("to_string"), Span::default()),
                    TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()),
                ], Span::default()),
            ], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("push"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree"), Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Ident(format!("Ident"), Span::default()),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"ToTokens\""), Span::default()),
                    TokenTree::Punct('.', Spacing::Alone, Span::default()),
                    TokenTree::Ident(format!("to_string"), Span::default()),
                    TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()),
                ], Span::default()),
            ], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("push"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree"), Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Ident(format!("Ident"), Span::default()),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"for\""), Span::default()),
                    TokenTree::Punct('.', Spacing::Alone, Span::default()),
                    TokenTree::Ident(format!("to_string"), Span::default()),
                    TokenTree::Group(Delimiter::Parenthesis, vec![], Span::default()),
                ], Span::default()),
            ], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("self"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("data"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("to_tokens"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Punct('&', Spacing::Alone, Span::default()),
                TokenTree::Ident(format!("mut"), Span::default()),
                TokenTree::Ident(format!("vec"), Span::default()),
            ], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct('.', Spacing::Alone, Span::default()),
            TokenTree::Ident(format!("push"), Span::default()),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree"), Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Punct(':', Spacing::Joint, Span::default()),
                TokenTree::Ident(format!("Group"), Span::default()),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Ident(format!("Delimiter"), Span::default()),
                    TokenTree::Punct(':', Spacing::Joint, Span::default()),
                    TokenTree::Punct(':', Spacing::Joint, Span::default()),
                    TokenTree::Ident(format!("Brace"), Span::default()),
                    TokenTree::Punct(',', Spacing::Alone, Span::default()),
                    TokenTree::Ident(format!("vec"), Span::default()),
                    TokenTree::Punct('!', Spacing::Alone, Span::default()),
                    TokenTree::Group(Delimiter::Bracket, vec![], Span::default()) 
                ], Span::default()),
            ], Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default()),
            
            TokenTree::Ident(format!("return"), Span::default()),
            TokenTree::Ident(format!("vec"), Span::default()),
            TokenTree::Punct(';', Spacing::Alone, Span::default())
        ];
        
        assert_eq!(out, expected, "{:?}", out);
    }
}