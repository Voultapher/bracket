// Parses a double-quoted JSON-style string into tokens.
use logos::{Lexer, Logos, Span};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Outer {
    #[token("\"")]
    Start,

    #[error]
    Error,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Inner {
    #[regex(r#"[^\\"]+"#)]
    Text,

    #[token("\\n")]
    EscapedNewline,

    //#[regex(r"\\u\{[^}]*\}")]
    //EscapedCodepoint,
    #[token(r#"\""#)]
    EscapedQuote,

    #[token("\"")]
    End,

    #[error]
    Error,
}

/*
#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
    InnerToken(Inner),
    OuterToken(Outer),
}

enum Modes<'source> {
    Outer(Lexer<'source, Outer>),
    Inner(Lexer<'source, Inner>),
}

impl<'source> Modes<'source> {
    fn new(s: &'source str) -> Self {
        Self::Outer(Outer::lexer(s))
    }
}

struct ModeBridge<'source> {
    mode: Modes<'source>,
}

// Clones as we switch between modes
impl<'source> Iterator for ModeBridge<'source> {
    type Item = (Tokens, Span);
    fn next(&mut self) -> Option<Self::Item> {
        use Tokens::*;
        match &mut self.mode {
            Modes::Inner(inner) => {
                //let result = inner.next();
                //if Some(Inner::End(r#"""#)) == result {
                //self.mode = Modes::Outer(inner.to_owned().morph());
                //}
                //result.map(InnerToken)

                let result = inner.next();
                let span = inner.span();
                //println!("Inner span {:?}", span);
                if let Some(token) = result {
                    if Inner::End == token {
                        self.mode = Modes::Outer(inner.to_owned().morph());
                    }
                    Some((InnerToken(token), span))
                } else {
                    None
                }
            }
            Modes::Outer(outer) => {
                //let result = outer.next();
                //if Some(Outer::Start(r#"""#)) == result {
                //self.mode = Modes::Inner(outer.to_owned().morph());
                //}
                //result.map(OuterToken)

                let result = outer.next();
                let span = outer.span();
                //println!("Outer span {:?}", span);
                if let Some(token) = result {
                    if Outer::Start == token {
                        self.mode = Modes::Inner(outer.to_owned().morph());
                    }
                    Some((OuterToken(token), span))
                } else {
                    None
                }
            }
        }
    }
}
*/

pub fn lex(s: &str) -> Vec<(super::modes::Tokens<Outer, Inner>, Span)> {
    super::modes::lex::<Outer, Inner>(s, Outer::Start, Inner::End)
    //let moded = ModeBridge {
        //mode: Modes::new(s),
    //};
    //let results: Vec<(Tokens, Span)> = moded.collect();
    //results
}
