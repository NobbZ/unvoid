pub mod ty;

use std::sync::Arc;

use eyre::Result;
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, Diagnostics, Sources, Vm,
};

use self::ty::author;
use self::ty::version;

pub fn prepare_context() -> Result<Context> {
    // Manifest::register(&mut prelude)?;
    // Version::register(&mut prelude)?;

    let mut context = Context::with_default_modules()?;
    context.install(author::module()?)?;
    context.install(version::module()?)?;

    Ok(context)
}

pub fn init_rune_vm(sources: &mut Sources) -> Result<Vm> {
    let context = prepare_context()?;

    let runtime = Arc::new(context.runtime()?);

    let mut diagnostics = Diagnostics::new();

    let build_result = rune::prepare(sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, sources)?;
    }

    let unit = build_result?;

    Ok(Vm::new(runtime, Arc::new(unit)))
}
