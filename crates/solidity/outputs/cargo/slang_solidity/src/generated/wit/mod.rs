// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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
            GuestTerminalNode, Node, NonterminalNode, Query, QueryError, QueryMatch,
            QueryMatchIterator, TerminalNode,
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
        query::{Query, QueryError, QueryMatch, QueryMatchIterator},
        text_index::{TextIndex, TextRange},
    };
}

macro_rules! define_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (rust::$name);
            impl From<rust::$name> for ffi::$name {
                #[inline]
                fn from(value: rust::$name) -> Self {
                    ffi::$name::new( [<$name Wrapper>] (value))
                }
            }
            impl<'a> From<&'a ffi::$name> for &'a rust::$name {
                #[inline]
                fn from(value: &'a ffi::$name) -> Self {
                    &value.get::<[<$name Wrapper>]>().0
                }
            }
            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_rc_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::rc::Rc<rust::$name>);
            impl From<rust::$name> for ffi::$name {
                #[inline]
                fn from(value: rust::$name) -> Self {
                    ffi::$name::new( [<$name Wrapper>] (std::rc::Rc::new(value)))
                }
            }
            impl From<std::rc::Rc<rust::$name>> for ffi::$name {
                #[inline]
                fn from(value: std::rc::Rc<rust::$name>) -> Self {
                    ffi::$name::new( [<$name Wrapper>] (value))
                }
            }
            impl From<ffi::$name> for std::rc::Rc<rust::$name> {
                #[inline]
                fn from(value: ffi::$name) -> Self {
                    value.get::<[<$name Wrapper>]>().0.clone()
                }
            }
            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_refcell_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::cell::RefCell<rust::$name>);
            impl From<rust::$name> for ffi::$name {
                #[inline]
                fn from(value: rust::$name) -> Self {
                    ffi::$name::new( [<$name Wrapper>] (std::cell::RefCell::new(value)))
                }
            }
            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
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
pub(crate) use {define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum};

#[allow(clippy::upper_case_acronyms)]
pub struct API;
