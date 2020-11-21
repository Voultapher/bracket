//! Errors generated when compiling templates.
use std::fmt;
use thiserror::Error;

#[derive(Error, Eq, PartialEq)]
pub enum SyntaxError {
    #[error("Syntax error, expecting identifier")]
    ExpectedIdentifier(String),
    #[error("Syntax error, block name must be an identifier")]
    BlockName(String),
    #[error(
        "Syntax error, new lines in raw literals must be escaped (\\n)"
    )]
    LiteralNewline(String),
    #[error(
        "Syntax error, explicit this reference must be at the start of a path"
    )]
    UnexpectedPathExplicitThis(String),
    #[error("Syntax error, parent scopes must be at the start of a path")]
    UnexpectedPathParent(String),
    #[error(
        "Syntax error, local scope identifiers must be at the start of a path"
    )]
    UnexpectedPathLocal(String),
    #[error("Syntax error, expected identifier but got path delimiter")]
    UnexpectedPathDelimiter(String),
    #[error("Syntax error, parent scopes and local identifiers are mutually exclusive")]
    UnexpectedPathParentWithLocal(String),
    #[error(
        "Syntax error, parent scopes and explicit this are mutually exclusive"
    )]
    UnexpectedPathParentWithExplicit(String),
    #[error("Syntax error, expected path delimiter (.)")]
    ExpectedPathDelimiter(String),
    #[error("Syntax error, sub-expression not terminated")]
    OpenSubExpression(String),
    #[error("Syntax error, closing name does not match")]
    TagNameMismatch(String),
    #[error("Syntax error, got a closing tag but no block is open")]
    BlockNotOpen(String),

    #[error("Syntax error, sub-expression was not terminated")]
    SubExpressionNotTerminated(String),
    #[error("Syntax error, link was not terminated")]
    LinkNotTerminated(String),
    #[error("Syntax error, raw block open tag was not terminated")]
    RawBlockOpenNotTerminated(String),
    #[error("Syntax error, raw block was not terminated")]
    RawBlockNotTerminated(String),
    #[error("Syntax error, raw comment was not terminated")]
    RawCommentNotTerminated(String),
    #[error("Syntax error, raw statement was not terminated")]
    RawStatementNotTerminated(String),
    #[error("Syntax error, comment was not terminated")]
    CommentNotTerminated(String),

    #[error("Syntax error, block target sub expressions are only supported for partials")]
    BlockTargetSubExpr(String),
    #[error("Syntax error, path is empty")]
    EmptyPath(String),
    #[error("Syntax error, path component type could not be identified")]
    ComponentType(String),
    #[error("Syntax error, partials and conditionals may not be combined")]
    MixedPartialConditional(String),

    #[error("Syntax error, expecting JSON literal token")]
    TokenJsonLiteral(String),
    #[error("Syntax error, expecting raw literal token")]
    TokenRawLiteral(String),
    #[error("Syntax error, unexpected token parsing quoted literal (\"\")")]
    TokenDoubleQuoteLiteral(String),
    #[error("Syntax error, unexpected token parsing quoted literal ('')")]
    TokenSingleQuoteLiteral(String),
    #[error("Syntax error, unexpected token parsing quoted literal ([])")]
    TokenArrayLiteral(String),
    #[error("Syntax error, unexpected token parsing link")]
    TokenLink(String),
    #[error("Syntax error, unexpected token parsing path")]
    TokenParameterPath(String),
    #[error("Syntax error, unexpected token, expecting end of raw block")]
    TokenEndRawBlock(String),
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.to_string())?;
        match *self {
            Self::ExpectedIdentifier(ref source)
            | Self::BlockName(ref source)
            | Self::LiteralNewline(ref source)
            | Self::UnexpectedPathExplicitThis(ref source)
            | Self::UnexpectedPathParent(ref source)
            | Self::UnexpectedPathLocal(ref source)
            | Self::UnexpectedPathDelimiter(ref source)
            | Self::UnexpectedPathParentWithLocal(ref source)
            | Self::UnexpectedPathParentWithExplicit(ref source)
            | Self::ExpectedPathDelimiter(ref source)
            | Self::OpenSubExpression(ref source)
            | Self::TagNameMismatch(ref source)
            | Self::SubExpressionNotTerminated(ref source)
            | Self::LinkNotTerminated(ref source)
            | Self::RawBlockNotTerminated(ref source)
            | Self::RawCommentNotTerminated(ref source)
            | Self::RawStatementNotTerminated(ref source)
            | Self::CommentNotTerminated(ref source)
            | Self::BlockTargetSubExpr(ref source)
            | Self::EmptyPath(ref source)
            | Self::ComponentType(ref source)
            | Self::MixedPartialConditional(ref source)
            | Self::RawBlockOpenNotTerminated(ref source)
            | Self::TokenJsonLiteral(ref source)
            | Self::TokenRawLiteral(ref source)
            | Self::TokenDoubleQuoteLiteral(ref source)
            | Self::TokenSingleQuoteLiteral(ref source)
            | Self::TokenArrayLiteral(ref source)
            | Self::TokenLink(ref source)
            | Self::TokenParameterPath(ref source)
            | Self::TokenEndRawBlock(ref source)
            | Self::BlockNotOpen(ref source) => write!(f, "{}", source)?,
        }
        Ok(())
    }
}
