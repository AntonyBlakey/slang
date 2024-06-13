use std::path::Path;

use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;

fn main() -> Result<()> {
    OutputLanguage::Cargo.generate_stubs()?;
    OutputLanguage::Cargo.generate_wit(Path::new(
        "/workspaces/slang/crates/codegen/runtime/cargo/src/runtime/wit",
    ))?;
    Ok(())
}
