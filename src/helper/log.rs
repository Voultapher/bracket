//! Helper to print log messages.
use crate::{
    helper::{Assertion, Helper, ValueResult},
    render::{Context, Render},
};

use log::*;

/// Helper that prints a log message.
///
/// Use the `level` hash parameter to set the log level to one of:
///
/// * trace
/// * debug
/// * info
/// * warn
/// * error
///
#[derive(Clone)]
pub struct LogHelper;

impl Helper for LogHelper {
    fn call<'reg, 'source, 'render, 'call>(
        &self,
        rc: &mut Render<'reg, 'source, 'render>,
        ctx: &mut Context<'call>,
    ) -> ValueResult {
        rc.arity(&ctx, 1..usize::MAX)?;

        let args = ctx.arguments();
        let hash = ctx.hash();

        let message = args
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let level = hash
            .get("level")
            .map(|v| v.as_str())
            .unwrap_or(Some("info"))
            .unwrap();

        match level {
            "error" => error!("{}", message),
            "debug" => debug!("{}", message),
            "warn" => warn!("{}", message),
            "trace" => trace!("{}", message),
            _ => info!("{}", message),
        }

        Ok(None)
    }
}
