// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{define_rc_wrapper, define_wrapper, ffi, rust};

//================================================
//
// resource nonterminal-node
//
//================================================

define_rc_wrapper! { NonterminalNode {
    fn kind(&self) -> ffi::NonterminalKind {
        self.0.kind.into()
    }

    fn text_len(&self) -> ffi::TextIndex {
        self.0.text_len.into()
    }

    fn children(&self) -> Vec<ffi::Node> {
        todo!()
    }

    fn create_cursor(&self, _text_offset: ffi::TextIndex) -> ffi::Cursor {
        todo!()
    }

    fn unparse(&self) -> String {
        std::rc::Rc::clone(&self.0).unparse()
    }
} }

//================================================
//
// resource terminal-node
//
//================================================

define_rc_wrapper! { TerminalNode {
    fn kind(&self) -> ffi::TerminalKind {
        self.0.kind.into()
    }

    fn text(&self) -> String {
        self.0.text.clone()
    }

    fn text_len(&self) -> ffi::TextIndex {
        rust::TextIndex::from(&self.0.text).into()
    }
} }

//================================================
//
// variant node
//
//================================================

impl From<rust::Node> for ffi::Node {
    fn from(node: rust::Node) -> Self {
        match node {
            rust::Node::Nonterminal(node) => Self::Nonterminal(node.into()),
            rust::Node::Terminal(node) => Self::Terminal(node.into()),
        }
    }
}

//================================================
//
// resource cursor
//
//================================================

define_wrapper! { Cursor {
    fn reset(&self) {
        todo!()
    }

    fn complete(&self) {
        todo!()
    }

    fn is_completed(&self) -> bool {
        todo!()
    }

    fn clone(&self) -> ffi::Cursor {
        todo!()
    }

    fn spawn(&self) -> ffi::Cursor {
        todo!()
    }

    fn node(&self) -> ffi::Node {
        todo!()
    }

    fn label(&self) -> Option<ffi::EdgeLabel> {
        todo!()
    }

    fn text_offset(&self) -> ffi::TextIndex {
        todo!()
    }

    fn text_range(&self) -> ffi::TextRange {
        todo!()
    }

    fn depth(&self) -> u32 {
        todo!()
    }

    fn ancestors(&self) -> Vec<ffi::NonterminalNode> {
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

    fn go_to_next_terminal_with_kind(&self, _kind: ffi::TerminalKind) -> bool {
        todo!()
    }

    fn go_to_next_terminal_with_kinds(&self, _kinds: Vec<ffi::TerminalKind>) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal(&self) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal_with_kind(&self, _kind: ffi::NonterminalKind) -> bool {
        todo!()
    }

    fn go_to_next_nonterminal_with_kinds(&self, _kinds: Vec<ffi::NonterminalKind>) -> bool {
        todo!()
    }

    fn query(&self, _queries: Vec<ffi::Query>) -> ffi::QueryMatchIterator {
        todo!()
    }
} }

//================================================
//
// resource query
//
//================================================

define_wrapper! { Query {
    fn parse(_text: String) -> Result<ffi::Query, ()> {
        todo!()
    }
} }

//================================================
//
// resource query-match-iterator
//
//================================================

define_wrapper! { QueryMatchIterator {
    fn next(&self) -> Result<Option<ffi::QueryMatch>, ()> {
        todo!()
    }
} }

//================================================
//
// record query-match
//
//================================================

impl From<rust::QueryMatch> for ffi::QueryMatch {
    fn from(value: rust::QueryMatch) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            query_number: value.query_number as u32,
            captures: vec![],
        }
    }
}

//================================================
//
// interface cst
//
//================================================

impl ffi::cst::Guest for super::API {
    type NonterminalNode = NonterminalNodeWrapper;
    type TerminalNode = TerminalNodeWrapper;
    type Cursor = CursorWrapper;
    type Query = QueryWrapper;
    type QueryMatchIterator = QueryMatchIteratorWrapper;
}
