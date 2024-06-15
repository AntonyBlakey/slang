pub mod cst;
pub mod diagnostic;
pub mod kinds;
pub mod language;
pub mod text_index;

#[path = "generated/slang.rs"]
pub mod slang;

// #[path = "generated/ast_selectors.rs"]
// pub mod ast_selectors;

pub mod ffi {
    pub use crate::wit::slang::exports::nomic::slang::{
        cst::{
            self, Cursor, GuestCursor, GuestNonterminalNode, GuestQuery, GuestQueryMatchIterator,
            GuestTerminalNode, Node, NonterminalNode, Query, QueryMatch, QueryMatchIterator,
            TerminalNode,
        },
        diagnostic::Severity,
        kinds::{EdgeLabel, NonterminalKind, TerminalKind},
        language::{
            self, GuestLanguage, GuestParseError, GuestParseOutput, Language, ParseError,
            ParseOutput,
        },
        text_index::{TextIndex, TextRange},
    };
}

pub mod rust {
    pub use crate::{
        cst::{Edge, Node, NonterminalNode, TerminalNode},
        cursor::Cursor,
        diagnostic::{Diagnostic, Severity},
        kinds::{EdgeLabel, NonterminalKind, TerminalKind},
        language::Language,
        parse_error::ParseError,
        parse_output::ParseOutput,
        query::{Query, QueryMatch, QueryMatchIterator},
        text_index::{TextIndex, TextRange},
    };
}

macro_rules! define_wrapper_base {
    ($name:ident, $rust:ty, $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] ($rust);
            impl From<$rust> for ffi::$name {
                #[inline]
                fn from(value: $rust) -> Self {
                    ffi::$name::new( [<$name Wrapper>] (value))
                }
            }
            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_wrapper {
    ($name:ident $impl:tt) => {
        $crate::wit::define_wrapper_base!($name, rust::$name, $impl);
    };
}

macro_rules! define_rc_wrapper {
    ($name:ident $impl:tt) => {
        $crate::wit::define_wrapper_base!($name, std::rc::Rc<rust::$name>, $impl);
    };
}

macro_rules! define_refcell_wrapper {
    ($name:ident $impl:tt) => {
        $crate::wit::define_wrapper_base!($name, std::cell::RefCell<rust::$name>, $impl);
    };
}

macro_rules! enum_to_enum {
    ($name:ident) => {
        enum_to_enum!(ffi::$name, rust::$name);
        enum_to_enum!(rust::$name, ffi::$name);
    };
    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            #[inline]
            fn from(value: $from) -> Self {
                unsafe { core::mem::transmute(value) }
            }
        }
    };
}

// The trick: https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use {
    define_rc_wrapper, define_refcell_wrapper, define_wrapper, define_wrapper_base, enum_to_enum,
};

#[allow(clippy::upper_case_acronyms)]
pub struct API;
