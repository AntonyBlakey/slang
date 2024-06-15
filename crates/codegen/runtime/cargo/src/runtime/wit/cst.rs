use super::{define_rc_wrapper, define_refcell_wrapper, define_wrapper, ffi, rust};

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

define_refcell_wrapper! { Cursor {
    fn reset(&self) {
        self.0.borrow_mut().reset();
    }

    fn complete(&self) {
        self.0.borrow_mut().complete();
    }

    fn is_completed(&self) -> bool {
        self.0.borrow().is_completed()
    }

    fn clone(&self) -> ffi::Cursor {
        self.0.borrow().clone().into()
    }

    fn spawn(&self) -> ffi::Cursor {
        self.0.borrow().spawn().into()
    }

    fn node(&self) -> ffi::Node {
        self.0.borrow().node().into()
    }

    fn label(&self) -> Option<ffi::EdgeLabel> {
        self.0.borrow().label().map(Into::into)
    }

    fn text_offset(&self) -> ffi::TextIndex {
        self.0.borrow().text_offset().into()
    }

    fn text_range(&self) -> ffi::TextRange {
        self.0.borrow().text_range().into()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn depth(&self) -> u32 {
        self.0.borrow().depth() as u32
    }

    fn ancestors(&self) -> Vec<ffi::NonterminalNode> {
        self.0.borrow().ancestors().map(Into::into).collect()
    }

    fn go_to_next(&self) -> bool {
        self.0.borrow_mut().go_to_next()
    }

    fn go_to_next_non_descendent(&self) -> bool {
        self.0.borrow_mut().go_to_next_non_descendent()
    }

    fn go_to_previous(&self) -> bool {
        self.0.borrow_mut().go_to_previous()
    }

    fn go_to_parent(&self) -> bool {
        self.0.borrow_mut().go_to_parent()
    }

    fn go_to_first_child(&self) -> bool {
        self.0.borrow_mut().go_to_first_child()
    }

    fn go_to_last_child(&self) -> bool {
        self.0.borrow_mut().go_to_last_child()
    }

    fn go_to_nth_child(&self, child_number: u32) -> bool {
        self.0.borrow_mut().go_to_nth_child(child_number as usize)
    }

    fn go_to_next_sibling(&self) -> bool {
        self.0.borrow_mut().go_to_next_sibling()
    }

    fn go_to_previous_sibling(&self) -> bool {
        self.0.borrow_mut().go_to_previous_sibling()
    }

    fn go_to_next_terminal(&self) -> bool {
        self.0.borrow_mut().go_to_next_terminal()
    }

    fn go_to_next_terminal_with_kind(&self, kind: ffi::TerminalKind) -> bool {
        self.0.borrow_mut().go_to_next_terminal_with_kind(kind.into())
    }

    fn go_to_next_terminal_with_kinds(&self, kinds: Vec<ffi::TerminalKind>) -> bool {
        let kinds = kinds.into_iter().map(Into::into).collect::<Vec<_>>();
        self.0.borrow_mut().go_to_next_terminal_with_kinds(&kinds)
    }

    fn go_to_next_nonterminal(&self) -> bool {
        self.0.borrow_mut().go_to_next_nonterminal()
    }

    fn go_to_next_nonterminal_with_kind(&self, kind: ffi::NonterminalKind) -> bool {
        self.0.borrow_mut().go_to_next_nonterminal_with_kind(kind.into())
    }

    fn go_to_next_nonterminal_with_kinds(&self, kinds: Vec<ffi::NonterminalKind>) -> bool {
        let kinds = kinds.into_iter().map(Into::into).collect::<Vec<_>>();
        self.0.borrow_mut().go_to_next_nonterminal_with_kinds(&kinds)
    }

    fn query(&self, queries: Vec<ffi::Query>) -> ffi::QueryMatchIterator {
        let queries:Vec<rust::Query> = queries.iter().map(|q|{
            let q: &rust::Query = q.into();
            q.clone()
        }).collect();

        self.0.borrow().clone().query(queries).into()
    }
} }

//================================================
//
// resource query
//
//================================================

define_wrapper! { Query {
    fn parse(text: String) -> Result<ffi::Query, ffi::QueryError> {
        rust::Query::parse(&text).map_err(Into::into).map(Into::into)
    }
} }

//================================================
//
// record query-error
//
//================================================

impl From<rust::QueryError> for ffi::QueryError {
    fn from(value: rust::QueryError) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            message: value.message,
            line: value.line as u32,
            column: value.column as u32,
        }
    }
}

//================================================
//
// resource query-match-iterator
//
//================================================

define_refcell_wrapper! { QueryMatchIterator {
    fn next(&self) -> Option<ffi::QueryMatch> {
        self.0.borrow_mut().next().map(Into::into)
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
