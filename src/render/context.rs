//! Context information for the call to a helper.
use serde_json::{Map, Value};
use std::ops::Range;

use crate::{
    error::HelperError,
    helper::HelperResult,
    parser::ast::{Call, Node},
    render::Render,
};

/// Context for the call to a helper exposes immutable access to
/// the arguments and hash parameters for the helper.
///
/// It also provides some useful functions for asserting on argument
/// arity and the type of arguments and hash parameters.
pub struct Context<'call> {
    call: &'call Call<'call>,
    name: String,
    arguments: Vec<Value>,
    hash: Map<String, Value>,
    //template: Option<&'call Node<'call>>,
}

impl<'call> Context<'call> {
    pub fn new(
        call: &'call Call<'call>,
        name: String,
        arguments: Vec<Value>,
        hash: Map<String, Value>,
        //template: Option<&'call Node<'call>>,
    ) -> Self {
        Self {
            call,
            name,
            arguments,
            hash,
            //template,
        }
    }

    //pub fn template(&self) -> Option<&'call Node<'_>> {
        //self.template
    //}

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<Value> {
        &self.arguments
    }

    pub fn hash(&self) -> &Map<String, Value> {
        &self.hash
    }

    pub fn arity(&self, range: Range<usize>) -> HelperResult<()> {
        if range.start == range.end {
            if self.arguments().len() != range.start {
                return Err(HelperError::ArityExact(
                    self.name.clone(),
                    range.start,
                ));
            }
        } else {
            if self.arguments().len() < range.start
                || self.arguments().len() > range.end
            {
                return Err(HelperError::ArityRange(
                    self.name.clone(),
                    range.start,
                    range.end,
                ));
            }
        }
        Ok(())
    }
}
