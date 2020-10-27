use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

use serde_json::Value;

static WHITESPACE: &str = "~";

#[derive(Debug, Eq, PartialEq)]
pub enum Node<'source> {
    Text(Text<'source>),
    Statement(Call<'source>),
    Block(Block<'source>),
}

impl<'source> Node<'source> {
    pub fn as_str(&self) -> &'source str {
        match *self {
            Self::Text(ref n) => n.as_str(),
            Self::Statement(ref n) => n.as_str(),
            Self::Block(ref n) => n.as_str(),
        }
    }

    pub fn trim_before(&self) -> bool {
        match *self {
            Self::Text(_) => false,
            Self::Statement(ref n) => n.open().ends_with(WHITESPACE),
            Self::Block(ref n) => {
                let open = n.open();
                open.len() > 2 && WHITESPACE == &open[2..3]
            }
        }
    }

    pub fn trim_after(&self) -> bool {
        match *self {
            Self::Text(_) => false,
            Self::Statement(ref n) => n.close().starts_with(WHITESPACE),
            Self::Block(ref n) => {
                let open = n.open();
                open.len() > 2 && WHITESPACE == &open[2..3]
            }
        }
    }
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Text(ref n) => n.fmt(f),
            Self::Statement(ref n) => n.fmt(f),
            Self::Block(ref n) => n.fmt(f),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Text<'source>(pub &'source str, pub Range<usize>);

impl<'source> Text<'source> {
    pub fn as_str(&self) -> &'source str {
        &self.0[self.1.start..self.1.end]
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComponentType {
    Parent,
    This,
    Identifier,
    LocalIdentifier,
    Delimiter,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Component(pub ComponentType, pub Range<usize>);

#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    components: Vec<Component>,
}

impl Path {
    pub fn new() -> Self {
        Self {components: Vec::new()} 
    }

    pub fn add_component(&mut self, part: Component) {
        self.components.push(part); 
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty() 
    }

    pub fn is_simple(&self) -> bool {
        return self.components.len() == 1
            && self.components.first().unwrap().0 == ComponentType::Identifier;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParameterValue<'source> {
    Path(Path),
    Json(Value),
    SubExpr(Call<'source>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Call<'source> {
    // Raw source input.
    source: &'source str,
    partial: bool,
    open: Range<usize>,
    close: Range<usize>,
    path: Path,
    arguments: Vec<ParameterValue<'source>>,
    hash: HashMap<String, ParameterValue<'source>>,
}

impl<'source> Call<'source> {
    pub fn new(
        source: &'source str,
        partial: bool,
        open: Range<usize>,
        close: Range<usize>,
    ) -> Self {
        Self {
            source,
            partial,
            open,
            close,
            path: Path::new(),
            arguments: Vec::new(),
            hash: HashMap::new(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path 
    }

    pub fn path_mut(&mut self) -> &mut Path {
        &mut self.path 
    }

    pub fn add_argument(&mut self, arg: ParameterValue<'source>) {
        self.arguments.push(arg); 
    }

    pub fn arguments(&self) -> &Vec<ParameterValue<'source>> {
        &self.arguments
    }

    pub fn as_str(&self) -> &'source str {
        &self.source[self.open.start..self.close.end]
    }

    pub fn open(&self) -> &'source str {
        &self.source[self.open.start..self.open.end]
    }

    pub fn close(&self) -> &'source str {
        &self.source[self.close.start..self.close.end]
    }

    pub fn is_partial(&self) -> bool {
        self.partial 
    }
}

impl fmt::Display for Call<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BlockType {
    Root,
    RawBlock,     // {{{{raw}}}}{{expr}}{{{{/raw}}}}
    RawStatement, // \{{expr}}
    RawComment,   // {{!-- {{expr}} --}}
    Comment,      // {{! comment }}
    Scoped,       // {{#> partial|helper}}{{/partial|helper}}
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Root
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Block<'source> {
    // Raw source input.
    source: &'source str,
    kind: BlockType,
    nodes: Vec<Node<'source>>,
    open: Option<Range<usize>>,
    close: Option<Range<usize>>,
    call: Option<Call<'source>>,
}

impl<'source> Block<'source> {
    pub fn new(
        source: &'source str,
        kind: BlockType,
        open: Option<Range<usize>>,
    ) -> Self {
        Self {
            source,
            kind,
            nodes: Vec::new(),
            open,
            close: None,
            call: None,
        }
    }

    pub fn set_call(&mut self, call: Call<'source>) {
        self.call = Some(call);
    }

    pub(crate) fn exit(&mut self, span: Range<usize>) {
        self.close = Some(span);
    }

    pub fn as_str(&self) -> &'source str {
        match self.kind() {
            BlockType::Root => self.source,
            _ => {
                let open = self.open.clone().unwrap_or(0..0);
                let close = self.close.clone().unwrap_or(0..self.source.len());
                &self.source[open.start..close.end]
            }
        }
    }

    pub fn open(&self) -> &'source str {
        if let Some(ref open) = self.open {
            &self.source[open.start..open.end]
        } else {
            ""
        }
    }

    pub fn between(&self) -> &'source str {
        let open = self.open.clone().unwrap_or(0..0);
        let close = self.close.clone().unwrap_or(0..self.source.len());
        &self.source[open.end..close.start]
    }

    pub fn close(&self) -> &'source str {
        if let Some(ref close) = self.close {
            &self.source[close.start..close.end]
        } else {
            ""
        }
    }

    pub fn push(&mut self, node: Node<'source>) {
        self.nodes.push(node);
    }

    pub fn kind(&self) -> &BlockType {
        &self.kind
    }

    pub fn nodes(&self) -> &'source Vec<Node> {
        &self.nodes
    }

    pub fn trim_before_close(&self) -> bool {
        match self.kind {
            BlockType::Scoped => {
                let close = self.close();
                close.len() > 2 && WHITESPACE == &close[2..3]
            }
            _ => false,
        }
    }

    pub fn trim_after_close(&self) -> bool {
        match self.kind {
            BlockType::Scoped => {
                let close = self.close();
                let pos = close.len() - 3;
                close.len() > 2 && WHITESPACE == &close[pos..pos + 1]
            }
            _ => false,
        }
    }
}

impl fmt::Display for Block<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            BlockType::Root => write!(f, "{}", self.source),
            _ => {
                for t in self.nodes() {
                    t.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}
