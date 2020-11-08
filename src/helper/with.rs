//! Block helper that sets the scope.
use crate::{
    helper::{Assertion, BlockHelper, BlockResult, BlockTemplate},
    render::{Context, Render, Scope},
};

use serde_json::Value;

#[derive(Clone)]
pub struct WithHelper;

impl BlockHelper for WithHelper {
    fn call<'reg, 'source, 'render>(
        &self,
        rc: &mut Render<'reg, 'source, 'render>,
        ctx: &mut Context<'source>,
        block: BlockTemplate<'source>,
    ) -> BlockResult {
        rc.arity(&ctx, 1..1)?;

        let args = ctx.arguments();
        let target = args.get(0).unwrap();
        rc.push_scope(Scope::new());
        if let Some(ref mut scope) = rc.scope_mut() {
            scope.set_base_value(target.clone());
        }
        rc.template(block.template())?;
        rc.pop_scope();
        Ok(())
    }
}
