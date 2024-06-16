#[cfg(feature = "cli")]
use {
    anyhow::{Context, Result},
    clap::{Parser as ClapParser, Subcommand},
    semver::Version,
    slang_solidity::kinds::NonterminalKind,
    slang_solidity::language::Language,
    std::fs,
    std::path::PathBuf,
    std::process::ExitCode,
};

// Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
#[cfg(feature = "cli")]
mod supress_api_dependencies {
    use {
        ariadne as _, metaslang_cst as _, paste as _, serde as _, strum as _, strum_macros as _,
        thiserror as _,
    };
}

#[cfg(feature = "cli")]
#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a Solidity (*.sol) source file, and outputs any syntax errors, or a JSON concrete syntax tree
    Parse {
        /// File path to the Solidity (*.sol) source file to parse
        file_path: String,

        /// The Solidity language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// Print the concrete syntax tree as JSON
        #[clap(long)]
        json: bool,
    },
}

#[cfg(feature = "cli")]
fn main() -> Result<ExitCode> {
    match Cli::parse().command {
        Commands::Parse {
            file_path,
            version,
            json,
        } => execute_parse_command(&file_path, version, json),
    }
}

#[cfg(feature = "cli")]
fn execute_parse_command(file_path_string: &str, version: Version, json: bool) -> Result<ExitCode> {
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find file path: {file_path_string:?}"))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let output = language.parse(NonterminalKind::SourceUnit, &input);

    let errors = output.errors();
    for error in errors {
        const COLOR: bool = true;
        let report = slang_solidity::diagnostic::render(error, file_path_string, &input, COLOR);
        eprintln!("{report}");
    }

    if json {
        let root_node = output.tree();
        let json = serde_json::to_string_pretty(&root_node)?;
        println!("{json}");
    }

    if errors.is_empty() {
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!("Couldn't parse the Solidity source file.");
        Ok(ExitCode::FAILURE)
    }
}

#[cfg(feature = "cli")]
#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
