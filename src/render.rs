use serde::Serialize;
use serde_json::Value;

use crate::{
    error::RenderError,
    lexer::ast::{BlockType, Node},
    output::Output,
    registry::Registry,
};

pub trait Renderer<'reg, 'render> {
    fn render(
        &self,
        rc: &mut RenderContext<'reg, 'render>,
    ) -> Result<(), RenderError>;
}

pub struct RenderState {}

impl RenderState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct RenderContext<'reg, 'render> {
    registry: &'reg Registry<'reg>,
    root: Value,
    state: RenderState,
    writer: Box<&'render mut dyn Output>,
}

impl<'reg, 'render> RenderContext<'reg, 'render> {
    pub fn new<T: Serialize>(
        registry: &'reg Registry<'reg>,
        data: &T,
        state: RenderState,
        writer: Box<&'render mut dyn Output>,
    ) -> Result<Self, RenderError> {
        let root = serde_json::to_value(data).map_err(RenderError::from)?;
        Ok(Self {
            registry,
            root,
            state,
            writer,
        })
    }

    pub fn write_str(&mut self, s: &str) -> Result<usize, RenderError> {
        Ok(self.writer.write_str(s).map_err(RenderError::from)?)
    }
}

pub struct Render<'source> {
    node: &'source Node<'source>,
}

impl<'source> Render<'source> {
    pub fn new(node: &'source Node<'source>) -> Self {
        Self { node }
    }

    fn render_node<'reg, 'render>(
        &self,
        node: &Node<'source>,
        rc: &mut RenderContext<'reg, 'render>,
    ) -> Result<(), RenderError> {
        match node {
            Node::Text(ref n) => {
                rc.write_str(n.as_str())?;
            }
            Node::Statement(ref n) => {
                println!("TODO: Evaluate statement in render!");
                rc.write_str(n.as_str())?;
            }
            Node::Block(ref block) => {
                //println!("rendering a block {:?}", block.kind());
                match block.kind() {
                    BlockType::RawBlock => {
                        rc.write_str(block.between())?;
                    }
                    BlockType::RawComment | BlockType::Comment => {
                        // NOTE: must ignore raw comments when rendering
                    }
                    BlockType::RawStatement => {
                        let raw = &block.as_str()[1..];
                        rc.write_str(raw)?;
                    }
                    _ => {
                        for b in block.blocks().iter() {
                            //println!("Rendering block {:?}", b.as_str());
                            self.render_node(b, rc)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'reg, 'render> Renderer<'reg, 'render> for Render<'_> {
    fn render(
        &self,
        rc: &mut RenderContext<'reg, 'render>,
    ) -> Result<(), RenderError> {
        self.render_node(self.node, rc)
    }
}
