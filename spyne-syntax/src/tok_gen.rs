use crate::token::TokenTree;

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

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, tokens: &mut Vec<TokenTree>) {
        for item in self {
            item.to_tokens(tokens);
        }
    }
}