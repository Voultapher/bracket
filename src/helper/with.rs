//! Block helper that sets the scope.
use crate::{
    helper::{Helper, HelperValue},
    parser::ast::Node,
    render::{Context, Render, Scope},
};

#[derive(Clone)]
pub struct With;

impl Helper for With {
    fn call<'render, 'call>(
        &self,
        rc: &mut Render<'render>,
        ctx: &Context<'call>,
        template: Option<&'render Node<'render>>,
    ) -> HelperValue {
        ctx.arity(1..1)?;

        if let Some(template) = template {
            let args = ctx.arguments();
            let target = args.get(0).unwrap();
            rc.push_scope(Scope::new());
            if let Some(ref mut scope) = rc.scope_mut() {
                scope.set_base_value(target.clone());
            }
            rc.template(template)?;
            rc.pop_scope();
        }

        Ok(None)
    }
}
