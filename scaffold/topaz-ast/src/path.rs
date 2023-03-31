use crate::ident::Ident;
use crate::punctuated::Punctuated;
use crate::Token;
use std::string::ToString;

pub struct Path(pub Punctuated<Ident, Token![::]>);

impl From<Ident> for Path {
    fn from(value: Ident) -> Self {
        Self(value.into())
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl crate::private::_Tokens for Path {}

pub enum CallPath {
    OnObject(DottedPath),
    OnModule(Path)
}

pub struct DottedPath(pub Punctuated<Ident, Token![.]>);

impl From<Ident> for DottedPath {
    fn from(value: Ident) -> Self {
        Self(value.into())
    }
}

impl ToString for DottedPath {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl PartialEq for DottedPath {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl crate::private::_Tokens for DottedPath {}