use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, marker::PhantomData};

use crate::token::{Delimiter, TokenTree};

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>);
}
impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(self.to_owned());
    }
}
impl ToTokens for u8 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for u16 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for u32 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for u64 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for u128 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for usize {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for i8 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for i16 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for i32 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for i64 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for i128 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for isize {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for f32 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for f64 {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{}", self)));
    }
}
impl ToTokens for char {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{:?}", self)));
    }
}
impl<T: ToTokens> ToTokens for &[T] {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for item in *self {
            item.to_tokens(tokens);
        }
    }
}
impl<T: ToTokens, const N: usize> ToTokens for &[T; N] {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for item in *self {
            item.to_tokens(tokens);
        }
    }
}
impl ToTokens for str {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{:?}", self)));
    }
}
impl ToTokens for &str {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{:?}", self)));
    }
}
impl ToTokens for String {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Literal(format!("{:?}", self)));
    }
}
impl<T: ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        T::to_tokens(self, tokens);
    }
}
impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for item in self {
            item.to_tokens(tokens);
        }
    }
}
impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        if let Some(data) = self {
            data.to_tokens(tokens);
        }
    }
}
impl<T: ToTokens, E: ToTokens> ToTokens for Result<T, E> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        match self {
            Ok(ok) => ok.to_tokens(tokens),
            Err(err) => err.to_tokens(tokens),
        }
    }
}
impl ToTokens for () {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Group(Delimiter::Parenthesis, vec![]));
    }
}
impl<T: ToTokens> ToTokens for PhantomData<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::Ident(format!("PhantomData")));
    }
}
impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        T::to_tokens(&self, tokens);
    }
}
impl<'a, B> ToTokens for Cow<'a, B>
where B: ToOwned + ?Sized, B: ToTokens {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        B::to_tokens(&self, tokens);
    }
}
impl<T> ToTokens for HashSet<T>
where T: Ord, T: ToTokens {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        let mut hs_vec: Vec<_> = self.iter().collect();
        hs_vec.sort();
        for item in hs_vec {
            item.to_tokens(tokens);
        }
    }
}
impl<K, V> ToTokens for HashMap<K, V>
where K: Ord, K: ToTokens, V: ToTokens {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        let mut hm_vec: Vec<_> = self.iter().collect();
        hm_vec.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in hm_vec {
            k.to_tokens(tokens);
            v.to_tokens(tokens);
        }
    }
}
impl<T: ToTokens> ToTokens for BTreeSet<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for item in self {
            item.to_tokens(tokens);
        }
    }
}
impl<K: ToTokens, V: ToTokens> ToTokens for BTreeMap<K, V> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for (k, v) in self {
            k.to_tokens(tokens);
            v.to_tokens(tokens);
        }
    }
}