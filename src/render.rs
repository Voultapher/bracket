//! Render a template to output using the data.
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{
    error::RenderError,
    helper::Helper,
    template::Template,
    json,
    output::Output,
    parser::ast::{Call, CallTarget, Node, Block, ParameterValue, Path},
    registry::Registry,
};

#[derive(Debug)]
pub enum EvalResult<'render> {
    Json(Option<&'render Value>),
}

#[derive(Debug)]
pub struct Scope<'source, 'render> {
    value: Option<&'source Value>,
    locals: HashMap<String, &'render Value>,
    phantom: PhantomData<&'render Value>,
}

impl<'source, 'render> Scope<'source, 'render> {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            phantom: PhantomData,
            value: None,
        }
    }

    pub fn set_local(&mut self, name: &str, value: &'render Value) {
        self.locals.insert(format!("@{}", name), value);
    }

    pub fn set_base_value(&mut self, value: &'source Value) {
        self.value = Some(value);
    }

    pub fn base_value(&self) -> &Option<&'source Value> {
        &self.value
    }
}

//pub struct Context<'source> {
    //template: Option<Node<'source>>,
//}

//impl<'source> Context<'source> {
    //pub fn template(&'source self) -> &Option<Node<'source>> {
        //&self.template 
    //}
//}

pub struct Render<'reg, 'render, 'source> {
    source: &'source str,
    registry: &'reg Registry<'reg>,
    root: Value,
    writer: Box<&'render mut dyn Output>,
    scopes: Vec<Scope<'source, 'render>>,
    callee: Option<&'source Call<'source>>,
    trim_start: bool,
    trim_end: bool,
    prev_node: Option<&'source Node<'source>>,
    next_node: Option<&'source Node<'source>>,
    //context: Option<Context<'source>>,
    template: Option<Template<'source>>,
}

impl<'reg, 'render, 'source> Render<'reg, 'render, 'source> {
    pub fn new<T: Serialize>(
        source: &'source str,
        registry: &'reg Registry<'reg>,
        data: &T,
        writer: Box<&'render mut dyn Output>,
    ) -> Result<Self, RenderError> {
        let root = serde_json::to_value(data).map_err(RenderError::from)?;
        Ok(Self {
            source,
            registry,
            root,
            writer,
            scopes: Vec::new(),
            callee: None,
            trim_start: false,
            trim_end: false,
            prev_node: None,
            next_node: None,
            template: None,
        })
    }

    //pub fn render(&mut self) -> Result<(), RenderError> {
        //println!("RENDER THE INNER TEMPLATE...");
        //if let Some(template) = self.template.take() {
            //println!("Got inner template to render...") ;
            ////let node = template.node();
            ////self.render_node(template.node())?;
            ////node.foo();
        //}
        //Ok(())
    //}

    //pub fn context(&self) -> &Option<Context<'source>> {
        //&self.context
    //}

    pub fn out(&mut self) -> &mut Box<&'render mut dyn Output> {
        &mut self.writer 
    }

    fn write_str(
        &mut self,
        s: &str,
        escape: bool,
    ) -> Result<usize, RenderError> {

        let val = if self.trim_start { s.trim_start() } else { s };
        let val = if self.trim_end { val.trim_end() } else { val };
        if val.is_empty() {
            return Ok(0);
        }

        if escape {
            let handler = self.registry.escape();
            let escaped = handler(val);
            Ok(self.writer.write_str(&escaped).map_err(RenderError::from)?)
        } else {
            Ok(self.writer.write_str(val).map_err(RenderError::from)?)
        }
    }

    pub fn push_scope(&mut self) -> &mut Scope<'source, 'render> {
        let scope = Scope::new();
        self.scopes.push(scope);
        self.scopes.last_mut().unwrap()
    }

    pub fn pop_scope(&mut self) -> Option<Scope<'source, 'render>> {
        self.scopes.pop()
    }

    pub fn scope(&self) -> Option<&Scope<'source, 'render>> {
        self.scopes.last()
    }

    pub fn scope_mut(&mut self) -> Option<&mut Scope<'source, 'render>> {
        self.scopes.last_mut()
    }

    pub fn root(&self) -> &Value {
        &self.root
    }

    pub fn scopes(&self) -> &Vec<Scope<'source, 'render>> {
        &self.scopes
    }

    fn lookup(
        path: &Path,
        root: &'render Value,
        scopes: &'render Vec<Scope<'source, 'render>>,
    ) -> Option<&'render Value> {
        let scope = scopes.last();

        println!("Lookup path {:?}", path.as_str());

        // Handle explicit `@root` reference
        if path.is_root() {
            let parts = path
                .components()
                .iter()
                .skip(1)
                .map(|c| c.as_str())
                .collect();
            return json::find_parts(parts, root);
        // Handle explicit this only
        } else if path.is_explicit() && path.components().len() == 1 {
            println!("Got explicit this!!!");
            let this = if let Some(scope) = scope {
                if let Some(base) = scope.base_value() {
                    println!("Got explicit this with a scope base value!!!");
                    base    
                } else { root }
            } else { root };
            return Some(this)
        } else if path.is_simple() {
            let name = path.as_str();
            if let Some(scope) = scope {
                //println!("Look up in current scope...");
            } else {
                //println!("Look up in root scope...");
                let parts =
                    path.components().iter().map(|c| c.as_str()).collect();
                return json::find_parts(parts, root);
            }
        }
        None
    }

    pub fn is_truthy(&self, value: &Value) -> bool {
        json::is_truthy(value)
    }

    pub fn arguments(&self) -> Vec<&'source Value> {
        if let Some(call) = self.callee {
            call.arguments()
                .iter()
                .map(|p| {
                    match p {
                        ParameterValue::Json(val) => val,
                        _ => {
                            // TODO: lookup paths
                            // TODO: evaluate sub-expressions
                            &Value::Null
                        }
                    }
                })
                .collect()
        } else { Vec::new() }
    }

    pub fn hash(&self) -> HashMap<String, &'source Value> {

        if let Some(call) = self.callee {
            call.hash()
                .iter()
                .map(|(k, p)| {
                    match p {
                        ParameterValue::Json(val) => {
                            (k.to_string(), val)
                        }
                        _ => {
                            // TODO: lookup paths
                            // TODO: evaluate sub-expressions
                            (k.to_string(), &Value::Null)
                        }
                    }
                })
                .collect::<HashMap<String, &'source Value>>()
        } else { HashMap::new() }
    }

    pub fn invoke(
        &mut self,
        call: &'source Call,
        name: &str,
        helper: &'reg Box<dyn Helper + 'reg>,
    ) -> Result<Option<Value>, RenderError> {
        self.callee = Some(call);
        helper.call(self)?;
        self.callee = None;
        Ok(None)
    }

    fn statement(
        &mut self,
        call: &'source Call,
    ) -> Result<EvalResult, RenderError> {
        if call.is_partial() {
            println!("Got partial call for statement!");
        } else {
            //println!("Evaluating a call {:?}", call);
            match call.target() {
                CallTarget::Path(ref path) => {
                    if path.is_simple() {
                        if let Some(helper) =
                            self.registry.get_helper(path.as_str())
                        {
                            //println!("Found a helper for the simple path!");
                            self.invoke(call, path.as_str(), helper)?;
                        } else {
                            return Ok(EvalResult::Json(Render::lookup(
                                path,
                                self.root(),
                                self.scopes(),
                            )));
                        }
                    } else {
                        return Ok(EvalResult::Json(Render::lookup(
                            path,
                            self.root(),
                            self.scopes(),
                        )));
                    }
                }
                _ => todo!("Handle sub expressions"),
            }
        }
        Ok(EvalResult::Json(None))
    }

    fn block(
        &mut self,
        block: &'source Block<'source>,
    ) -> Result<EvalResult, RenderError> {
        println!("Render a block...");
        let call = block.call();

        if call.is_partial() {
            println!("Got partial call for block!");
        } else {

            println!("Call the block...");

            //println!("Evaluating a call {:?}", call);
            match call.target() {
                CallTarget::Path(ref path) => {
                    if path.is_simple() {
                        if let Some(helper) =
                            self.registry.get_helper(path.as_str())
                        {
                            self.template = Some(Template::new(self.source, Node::Fragment(block)));
                            println!(
                                "Found a helper for the block path {}", path.as_str());
                            self.invoke(call, path.as_str(), helper)?;
                        } else {
                            return Ok(EvalResult::Json(Render::lookup(
                                path,
                                self.root(),
                                self.scopes(),
                            )));
                        }
                    } else {
                        return Ok(EvalResult::Json(Render::lookup(
                            path,
                            self.root(),
                            self.scopes(),
                        )));
                    }
                }
                _ => todo!("Handle sub expressions"),
            }
        }

        Ok(EvalResult::Json(None))
    }

    pub(crate) fn render_node(
        &mut self,
        node: &'source Node<'source>,
    ) -> Result<(), RenderError> {

        self.trim_start = if let Some(node) = self.prev_node {
            node.trim_after()
        } else { false };

        self.trim_end = if let Some(node) = self.next_node {
            node.trim_before()
        } else { false };

        //let trim_after = node.trim_after();
        //println!("Has trim before {}", trim_before);
        //println!("Has trim after {}", trim_after);

        match node {
            Node::Text(ref n) => {
                self.write_str(n.as_str(), false)?;
            }
            Node::RawBlock(ref n) => {
                self.write_str(n.between(), false)?;
            }
            Node::RawStatement(ref n) => {
                let raw = &n.as_str()[1..];
                self.write_str(raw, false)?;
            }
            Node::RawComment(_) => {}
            Node::Comment(_) => {}
            Node::Document(ref doc) => {
                let mut it = doc.nodes().iter();
                self.next_node = it.next();
                while let Some(node) = self.next_node {
                    self.next_node = it.next();
                    self.render_node(node)?;
                }
            }
            Node::Statement(ref call) => {
                let result = self.statement(call)?;
                match result {
                    EvalResult::Json(maybe_json) => {
                        //println!("Got maybe json {:?}", maybe_json);
                        if let Some(value) = maybe_json {
                            let val = json::stringify(value)?;
                            //println!("Got a json string result {}", val);
                            self.write_str(&val, call.is_escaped())?;
                        } else {
                            //todo!("Error on missing varaible.");
                        }
                    }
                }
            }
            Node::Block(ref block) => {
                println!("got block to render...");
                self.block(block);
                // TODO: call partial / helper for blocks
                //for node in block.nodes().iter() {
                    //self.render(node)?;
                //}
            }
            _ => todo!("Render other node types"),
        }

        self.prev_node = Some(node);

        Ok(())
    }
}
