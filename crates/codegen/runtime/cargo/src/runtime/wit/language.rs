use rust::Diagnostic as _;

use super::{define_wrapper, ffi, rust};

//================================================
//
// resource language
//
//================================================

define_wrapper! { Language {
    fn new(_version: String) -> Result<ffi::Language, String> {
        todo!()
    }

    fn version(&self) -> String {
        self.0.version.to_string()
    }

    fn supported_versions(&self) -> Vec<String> {
        rust::Language::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect()
    }

    fn parse(&self, kind: ffi::NonterminalKind, input: String) -> ffi::ParseOutput {
        self.0.parse(kind.into(), &input).into()
    }
} }

//================================================
//
// resource parse-error
//
//================================================

define_wrapper! { ParseError {
    fn severity(&self) -> ffi::Severity {
        self.0.severity().into()
    }

    fn text_range(&self) -> ffi::TextRange {
        self.0.text_range().into()
    }

    fn message(&self) -> String {
        self.0.message()
    }
} }

//================================================
//
// resource parse-output
//
//================================================

define_wrapper! { ParseOutput {
    fn tree(&self) -> ffi::Node {
        self.0.tree().into()
    }

    fn errors(&self) -> Vec<ffi::ParseError> {
        self.0.errors().iter().map(|e| e.clone().into()).collect()
    }

    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    fn create_tree_cursor(&self) -> ffi::Cursor {
        todo!()
    }
} }

//================================================
//
// interface language
//
//================================================

impl ffi::language::Guest for super::API {
    type Language = LanguageWrapper;
    type ParseError = ParseErrorWrapper;
    type ParseOutput = ParseOutputWrapper;
}
