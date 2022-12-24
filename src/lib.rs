use postal::{Components, InitOptions, ParseAddressOptions, PostalError};
use std::sync::MutexGuard;

pub use postal::Component;
pub use postal::Context;

pub fn make_new() -> Result<Context, PostalError> {
    let mut ctx = Context::new();
    ctx.init(InitOptions {
        expand_address: false,
        parse_address: true,
    })?;
    Ok(ctx)
}

pub fn parse(
    ctx: MutexGuard<Context>,
    s: &str,
) -> Result<Vec<(String, String)>, PostalError> {
    let mut opts = ParseAddressOptions::new();
    ctx.parse_address(s, &mut opts).map(|x| {
        x.map(|s| (s.label.to_string(), s.value.to_string()))
            .collect::<Vec<_>>()
    })
}
