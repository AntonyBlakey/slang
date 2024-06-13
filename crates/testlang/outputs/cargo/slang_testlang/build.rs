use std::path::Path;

use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;
use infra_utils::cargo::CargoWorkspace;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("slang_testlang")?.join("src/generated");

    OutputLanguage::Cargo.generate_runtime(&language, &output_dir)?;
    OutputLanguage::Cargo.generate_wit(Path::new(
        "/workspaces/slang/crates/testlang/outputs/cargo/slang_testlang/src/generated/wit",
    ))?;
    Ok(())
}
