use std::vec;

use proc_macro::{Ident, Punct, Group, TokenStream};
use spyne_syntax::token::{TokenTree, Delimiter, Spacing};

pub fn quote_help(template: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut vec: Vec<TokenTree> = Vec::new();
    vec.push(TokenTree::Ident(format!("let")));
    vec.push(TokenTree::Ident(format!("mut")));
    vec.push(TokenTree::Ident(format!("vec")));
    vec.push(TokenTree::Punct(':', Spacing::Alone));
    vec.push(TokenTree::Ident(format!("Vec")));
    vec.push(TokenTree::Punct('<', Spacing::Alone));
    vec.push(TokenTree::Ident(format!("TokenTree")));
    vec.push(TokenTree::Punct('>', Spacing::Alone));
    vec.push(TokenTree::Punct('=', Spacing::Alone));
    vec.push(TokenTree::Ident(format!("Vec")));
    vec.push(TokenTree::Punct(':', Spacing::Joint));
    vec.push(TokenTree::Punct(':', Spacing::Joint));
    vec.push(TokenTree::Ident(format!("new")));
    vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![]));
    vec.push(TokenTree::Punct(';', Spacing::Alone));
    
    for tok in template {
        match tok {
            TokenTree::Group(Delimiter::Bracket, t) => {
                for item in t {
                    vec.push(item);
                }
                vec.push(TokenTree::Punct('.', Spacing::Alone));
                vec.push(TokenTree::Ident(format!("to_tokens")));
                vec.push(TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Punct('&', Spacing::Alone),
                    TokenTree::Ident(format!("mut")),
                    TokenTree::Ident(format!("vec"))
                ]));
            }
            _ => {
                vec.push(TokenTree::Ident(format!("vec")));
                vec.push(TokenTree::Punct('.', Spacing::Alone));
                vec.push(TokenTree::Ident(format!("push")));
                vec.push(TokenTree::Group(Delimiter::Parenthesis, {
                    let mut args: Vec<TokenTree> = Vec::new();
                    quote_token(&tok, &mut args);
                    
                    args
                }));
            }
        }
        
        vec.push(TokenTree::Punct(';', Spacing::Alone));
    }
    
    vec.push(TokenTree::Ident(format!("return")));
    vec.push(TokenTree::Ident(format!("vec")));
    vec.push(TokenTree::Punct(';', Spacing::Alone));
    
    vec
}

fn quote_token(token: &TokenTree, stream: &mut Vec<TokenTree>) {
    stream.push(TokenTree::Ident(format!("TokenTree")));
    stream.push(TokenTree::Punct(':', Spacing::Joint));
    stream.push(TokenTree::Punct(':', Spacing::Joint));
    match token {
        TokenTree::Ident(i) => {
            stream.push(TokenTree::Ident(format!("Ident")));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Literal(format!("{:?}", i)),
                TokenTree::Punct('.', Spacing::Alone),
                TokenTree::Ident(format!("to_string")),
                TokenTree::Group(Delimiter::Parenthesis, vec![])
            ]));
        }
        TokenTree::Punct(p, s) => {
            stream.push(TokenTree::Ident(format!("Punct")));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Literal(format!("{:?}", p)));
                items.push(TokenTree::Punct('.', Spacing::Alone));
                items.push(TokenTree::Ident(format!("to_owned")));
                items.push(TokenTree::Group(Delimiter::Parenthesis, vec![]));
                items.push(TokenTree::Punct(',', Spacing::Alone));
                items.push(TokenTree::Ident(format!("Spacing")));
                items.push(TokenTree::Punct(':', Spacing::Joint));
                items.push(TokenTree::Punct(':', Spacing::Joint));
                match s {
                    Spacing::Alone => items.push(TokenTree::Ident(format!("Alone"))),
                    Spacing::Joint => items.push(TokenTree::Ident(format!("Joint")))
                }
                
                items
            }));
        }
        TokenTree::Literal(l) => {
            stream.push(TokenTree::Ident(format!("Literal")));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Literal(format!("{:?}", l)),
                TokenTree::Punct('.', Spacing::Alone),
                TokenTree::Ident(format!("to_string")),
                TokenTree::Group(Delimiter::Parenthesis, vec![])
            ]));
        }
        TokenTree::Group(d, t) => {
            stream.push(TokenTree::Ident(format!("Group")));
            stream.push(TokenTree::Group(Delimiter::Parenthesis, {
                let mut items: Vec<TokenTree> = Vec::new();
                items.push(TokenTree::Ident(format!("Delimiter")));
                items.push(TokenTree::Punct(':', Spacing::Joint));
                items.push(TokenTree::Punct(':', Spacing::Joint));
                match d {
                    Delimiter::Parenthesis => items.push(TokenTree::Ident(format!("Parenthesis"))),
                    Delimiter::Brace => items.push(TokenTree::Ident(format!("Brace"))),
                    Delimiter::Bracket => items.push(TokenTree::Ident(format!("Bracket"))),
                    Delimiter::None => items.push(TokenTree::Ident(format!("None")))
                }
                items.push(TokenTree::Punct(',', Spacing::Alone));
                items.push(TokenTree::Ident(format!("vec")));
                items.push(TokenTree::Punct('!', Spacing::Alone));
                let mut inner_stream: Vec<TokenTree> = Vec::new();
                for item in t {
                    quote_token(item, &mut inner_stream);
                    inner_stream.push(TokenTree::Punct(',', Spacing::Alone));
                }
                items.push(TokenTree::Group(Delimiter::Bracket, inner_stream));
                
                items
            }));
        }
    }
}

pub fn from_stream(stream: TokenStream) -> Vec<TokenTree> {
    let mut out: Vec<TokenTree> = Vec::new();
    for tok in stream {
        match tok {
            proc_macro::TokenTree::Ident(i) => {
                out.push(TokenTree::Ident(i.to_string()));
            }
            proc_macro::TokenTree::Punct(p) => {
                match p.spacing() {
                    proc_macro::Spacing::Alone => out.push(TokenTree::Punct(p.as_char(), Spacing::Alone)),
                    proc_macro::Spacing::Joint => out.push(TokenTree::Punct(p.as_char(), Spacing::Joint)),
                }
            }
            proc_macro::TokenTree::Literal(l) => {
                out.push(TokenTree::Literal(l.to_string()));
            }
            proc_macro::TokenTree::Group(g) => {
                match g.delimiter() {
                    proc_macro::Delimiter::Parenthesis => out.push(TokenTree::Group(Delimiter::Parenthesis, from_stream(g.stream()))),
                    proc_macro::Delimiter::Brace => out.push(TokenTree::Group(Delimiter::Brace, from_stream(g.stream()))),
                    proc_macro::Delimiter::Bracket => out.push(TokenTree::Group(Delimiter::Bracket, from_stream(g.stream()))),
                    proc_macro::Delimiter::None => out.push(TokenTree::Group(Delimiter::None, from_stream(g.stream()))),
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
            TokenTree::Ident(i) => stream_vec.push(proc_macro::TokenTree::Ident(Ident::new(&i, proc_macro::Span::call_site()))),
            TokenTree::Punct(c, s) => {
                match s {
                    Spacing::Alone => stream_vec.push(proc_macro::TokenTree::Punct(Punct::new(c, proc_macro::Spacing::Alone))),
                    Spacing::Joint => stream_vec.push(proc_macro::TokenTree::Punct(Punct::new(c, proc_macro::Spacing::Joint)))
                }
            },
            TokenTree::Literal(l) => stream_vec.push(proc_macro::TokenTree::Literal(l.parse().unwrap())),
            TokenTree::Group(d, t) => {
                match d {
                    Delimiter::Parenthesis => stream_vec.push(proc_macro::TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, to_stream(t)))),
                    Delimiter::Brace => stream_vec.push(proc_macro::TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, to_stream(t)))),
                    Delimiter::Bracket => stream_vec.push(proc_macro::TokenTree::Group(Group::new(proc_macro::Delimiter::Bracket, to_stream(t)))),
                    Delimiter::None => stream_vec.push(proc_macro::TokenTree::Group(Group::new(proc_macro::Delimiter::None, to_stream(t)))),
                }
            }
        }
    }
    stream.extend(stream_vec);
    
    stream
}

#[cfg(test)]
mod test {
    use spyne_syntax::token::TokenTree;
    use super::*;
    
    #[test]
    fn test_quote() {
        let template: Vec<TokenTree> = vec![
            TokenTree::Ident(format!("impl")),
            TokenTree::Ident(format!("ToTokens")),
            TokenTree::Ident(format!("for")),
            TokenTree::Group(Delimiter::Bracket, vec![
                TokenTree::Ident(format!("self")),
                TokenTree::Punct('.', Spacing::Alone),
                TokenTree::Ident(format!("data"))
            ]),
            TokenTree::Group(Delimiter::Brace, vec![])
        ];
        let out = quote_help(template);
        
        let expected: Vec<TokenTree> = vec![
            TokenTree::Ident(format!("let")),
            TokenTree::Ident(format!("mut")),
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct(':', Spacing::Alone),
            TokenTree::Ident(format!("Vec")),
            TokenTree::Punct('<', Spacing::Alone),
            TokenTree::Ident(format!("TokenTree")),
            TokenTree::Punct('>', Spacing::Alone),
            TokenTree::Punct('=', Spacing::Alone),
            TokenTree::Ident(format!("Vec")),
            TokenTree::Punct(':', Spacing::Joint),
            TokenTree::Punct(':', Spacing::Joint),
            TokenTree::Ident(format!("new")),
            TokenTree::Group(Delimiter::Parenthesis, vec![]),
            TokenTree::Punct(';', Spacing::Alone),
            
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("push")),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree")),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Ident(format!("Ident")),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"impl\"")),
                    TokenTree::Punct('.', Spacing::Alone),
                    TokenTree::Ident(format!("to_string")),
                    TokenTree::Group(Delimiter::Parenthesis, vec![]),
                ]),
            ]),
            TokenTree::Punct(';', Spacing::Alone),
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("push")),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree")),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Ident(format!("Ident")),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"ToTokens\"")),
                    TokenTree::Punct('.', Spacing::Alone),
                    TokenTree::Ident(format!("to_string")),
                    TokenTree::Group(Delimiter::Parenthesis, vec![]),
                ]),
            ]),
            TokenTree::Punct(';', Spacing::Alone),
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("push")),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree")),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Ident(format!("Ident")),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Literal(format!("\"for\"")),
                    TokenTree::Punct('.', Spacing::Alone),
                    TokenTree::Ident(format!("to_string")),
                    TokenTree::Group(Delimiter::Parenthesis, vec![]),
                ]),
            ]),
            TokenTree::Punct(';', Spacing::Alone),
            TokenTree::Ident(format!("self")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("data")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("to_tokens")),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Punct('&', Spacing::Alone),
                TokenTree::Ident(format!("mut")),
                TokenTree::Ident(format!("vec"))
            ]),
            TokenTree::Punct(';', Spacing::Alone),
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct('.', Spacing::Alone),
            TokenTree::Ident(format!("push")),
            TokenTree::Group(Delimiter::Parenthesis, vec![
                TokenTree::Ident(format!("TokenTree")),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Punct(':', Spacing::Joint),
                TokenTree::Ident(format!("Group")),
                TokenTree::Group(Delimiter::Parenthesis, vec![
                    TokenTree::Ident(format!("Delimiter")),
                    TokenTree::Punct(':', Spacing::Joint),
                    TokenTree::Punct(':', Spacing::Joint),
                    TokenTree::Ident(format!("Brace")),
                    TokenTree::Punct(',', Spacing::Alone),
                    TokenTree::Ident(format!("vec")),
                    TokenTree::Punct('!', Spacing::Alone),
                    TokenTree::Group(Delimiter::Bracket, vec![])
                ]),
            ]),
            TokenTree::Punct(';', Spacing::Alone),
            
            TokenTree::Ident(format!("return")),
            TokenTree::Ident(format!("vec")),
            TokenTree::Punct(';', Spacing::Alone)
        ];
        
        assert_eq!(out, expected, "{:?}", out);
    }
}