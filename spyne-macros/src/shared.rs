use proc_macro::{Group, Ident, Punct, TokenStream};
use spyne_syntax::token::{Delimiter, Spacing, Span, TokenTree};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

thread_local! {
    pub static SPAN_MAP: RefCell<HashMap<usize, proc_macro::Span>> = RefCell::new(HashMap::new());
    pub static SPAN_COUNTER: Cell<usize> = Cell::new(1);
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