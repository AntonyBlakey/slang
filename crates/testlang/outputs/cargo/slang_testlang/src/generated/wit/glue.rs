// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::slang::exports;
use super::slang::nomic;
use crate::wit::slang::export;

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

impl exports::nomic::slang::cst::Guest for MyHost {
    type NonterminalNode = NonterminalNode;
    type TerminalNode = TerminalNode;
    type Cursor = Cursor;
    type Query = Query;
    type QueryMatchIterator = QueryMatchIterator;
}

pub struct NonterminalNode;
impl exports::nomic::slang::cst::GuestNonterminalNode for NonterminalNode {
    fn kind(&self) -> nomic::slang::kinds::NonterminalKind {
        todo!()
    }

    fn text_len(&self) -> exports::nomic::slang::text_index::TextIndex {
        todo!()
    }

    fn children(&self) -> Vec<exports::nomic::slang::cst::Node> {
        todo!()
    }

    fn create_cursor(
        &self,
        _text_offset: exports::nomic::slang::text_index::TextIndex,
    ) -> exports::nomic::slang::cst::Cursor {
        todo!()
    }

    fn to_json(&self) -> String {
        todo!()
    }

    fn unparse(&self) -> String {
        todo!()
    }
}

pub struct TerminalNode;
impl exports::nomic::slang::cst::GuestTerminalNode for TerminalNode {
    fn kind(&self) -> nomic::slang::kinds::TerminalKind {
        todo!()
    }

    fn text(&self) -> String {
        todo!()
    }

    fn to_json(&self) -> String {
        todo!()
    }

    fn text_len(&self) -> exports::nomic::slang::text_index::TextIndex {
        todo!()
    }

    fn create_cursor(
        &self,
        _text_offset: exports::nomic::slang::text_index::TextIndex,
    ) -> exports::nomic::slang::cst::Cursor {
        todo!()
    }
}

pub struct Cursor;
impl exports::nomic::slang::cst::GuestCursor for Cursor {
    fn reset(&self) {
        todo!()
    }

    fn complete(&self) {
        todo!()
    }

    fn is_completed(&self) -> bool {
        todo!()
    }

    fn clone(&self) -> exports::nomic::slang::cst::Cursor {
        todo!()
    }

    fn spawn(&self) -> exports::nomic::slang::cst::Cursor {
        todo!()
    }

    fn node(&self) -> exports::nomic::slang::cst::Node {
        todo!()
    }

    fn label(&self) -> Option<nomic::slang::kinds::EdgeLabel> {
        todo!()
    }

    fn text_offset(&self) -> exports::nomic::slang::text_index::TextIndex {
        todo!()
    }

    fn text_range(&self) -> exports::nomic::slang::text_index::TextRange {
        todo!()
    }

    fn depth(&self) -> u32 {
        todo!()
    }

    fn ancestors(&self) -> Vec<exports::nomic::slang::cst::NonterminalNode> {
        todo!()
    }

    fn go_to_next(&self) -> bool {
        todo!()
    }

    fn go_to_next_non_descendent(&self) -> bool {
        todo!()
    }

    fn go_to_previous(&self) -> bool {
        todo!()
    }

    fn go_to_parent(&self) -> bool {
        todo!()
    }

    fn go_to_first_child(&self) -> bool {
        todo!()
    }

    fn go_to_last_child(&self) -> bool {
        todo!()
    }

    fn go_to_nth_child(&self, _child_number: u32) -> bool {
        todo!()
    }

    fn go_to_next_sibling(&self) -> bool {
        todo!()
    }

    fn go_to_previous_sibling(&self) -> bool {
        todo!()
    }

    fn go_to_next_terminal(&self) -> bool {
        todo!()
    }

    fn go_to_next_terminal_with_kind(&self, _kind: nomic::slang::kinds::TerminalKind) -> bool {
        todo!()
    }

    fn go_to_next_terminal_with_kinds(
        &self,
        _kinds: Vec<nomic::slang::kinds::TerminalKind>,
    ) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal(&self) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal_with_kind(
        &self,
        _kind: nomic::slang::kinds::NonterminalKind,
    ) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal_with_kinds(
        &self,
        _kinds: Vec<nomic::slang::kinds::NonterminalKind>,
    ) -> bool {
        todo!()
    }

    fn query(
        &self,
        _queries: Vec<exports::nomic::slang::cst::Query>,
    ) -> exports::nomic::slang::cst::QueryMatchIterator {
        todo!()
    }
}

pub struct Query;
impl exports::nomic::slang::cst::GuestQuery for Query {
    fn parse(_text: String) -> Result<exports::nomic::slang::cst::Query, ()> {
        todo!()
    }
}

pub struct QueryMatchIterator;
impl exports::nomic::slang::cst::GuestQueryMatchIterator for QueryMatchIterator {
    fn next(&self) -> Result<Option<exports::nomic::slang::cst::QueryMatch>, ()> {
        todo!()
    }
}

impl exports::nomic::slang::diagnostic::Guest for MyHost {
    type Diagnostic = Diagnostic;
}

pub struct Diagnostic;
impl exports::nomic::slang::diagnostic::GuestDiagnostic for Diagnostic {
    fn text_range(&self) -> exports::nomic::slang::text_index::TextRange {
        todo!()
    }

    fn severity(&self) -> exports::nomic::slang::diagnostic::Severity {
        todo!()
    }

    fn message(&self) -> String {
        todo!()
    }
}

impl exports::nomic::slang::language::Guest for MyHost {
    type Language = Language;
    type ParseError = ParseError;
    type ParseOutput = ParseOutput;
}

pub struct Language;
impl exports::nomic::slang::language::GuestLanguage for Language {
    fn new(_version: String) -> Result<exports::nomic::slang::language::Language, String> {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }

    fn supported_versions(&self) -> Vec<String> {
        todo!()
    }

    fn parse(
        &self,
        _kind: nomic::slang::kinds::NonterminalKind,
        _input: String,
    ) -> exports::nomic::slang::language::ParseOutput {
        todo!()
    }
}

/// is-a diagnostic"]
pub struct ParseError;
impl exports::nomic::slang::language::GuestParseError for ParseError {
    fn severity(&self) -> exports::nomic::slang::diagnostic::Severity {
        todo!()
    }

    fn text_range(&self) -> exports::nomic::slang::text_index::TextRange {
        todo!()
    }

    fn message(&self) -> String {
        todo!()
    }
}

pub struct ParseOutput;
impl exports::nomic::slang::language::GuestParseOutput for ParseOutput {
    fn tree(&self) -> exports::nomic::slang::cst::Node {
        todo!()
    }

    fn errors(&self) -> Vec<exports::nomic::slang::language::ParseError> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn create_tree_cursor(&self) -> exports::nomic::slang::cst::Cursor {
        todo!()
    }
}

export!(MyHost);
