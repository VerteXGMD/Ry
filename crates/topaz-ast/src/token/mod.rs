#![allow(deprecated)]
use topaz_macro::everything;

everything! {
    // delimiters
    [()] => delim::Parentheses,
    [[]] => delim::Brackets,
    [<>] => delim::AngleBracket,
    [""] => delim::StringDelim,
    //[''] => delim::CharDelim,
    [{}] => delim::Curly,
    // punctuation
    [,] => punctuation::Comma,
    [:] => punctuation::Colon,
    [;] => punctuation::Semi,
    [.] => punctuation::Dot,
    [->] => punctuation::Arrow,
    [::] => punctuation::DoubleColon,
    [=] => punctuation::Equal,
    [-] => punctuation::Minus,
    [+] => punctuation::Plus,
    // prefixes
    [&] => prefix::Ref,
    // keywords
    [mut] => keyword::Mut,
    [func] => keyword::Func,
    [let] => keyword::Let,
    [maybe] => keyword::Maybe,
    [some] => keyword::Some,
    [nope] => keyword::Nope,
    [typealias] => keyword::TypeAlias,
    [this] => keyword::This,
    [super] => keyword::Super,
    [gem] => keyword::Gem,
    [import] => keyword::Import
}

pub mod delim;
pub mod keyword;
pub mod prefix;
pub mod punctuation;
pub use keyword::*;
pub use prefix::*;
pub use punctuation::*;
