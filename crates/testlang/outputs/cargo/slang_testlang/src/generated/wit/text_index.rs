// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{ffi, rust};

//================================================
//
// record text-index
//
//================================================

impl From<ffi::TextIndex> for rust::TextIndex {
    fn from(value: ffi::TextIndex) -> Self {
        Self {
            utf8: value.utf8 as usize,
            utf16: value.utf16 as usize,
            line: value.line as usize,
            column: value.column as usize,
        }
    }
}

impl From<&rust::TextIndex> for ffi::TextIndex {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: &rust::TextIndex) -> Self {
        Self {
            utf8: value.utf8 as u32,
            utf16: value.utf16 as u32,
            line: value.line as u32,
            column: value.column as u32,
        }
    }
}

impl From<rust::TextIndex> for ffi::TextIndex {
    fn from(value: rust::TextIndex) -> Self {
        (&value).into()
    }
}

//================================================
//
// record text-range
//
//================================================

impl From<ffi::TextRange> for rust::TextRange {
    fn from(value: ffi::TextRange) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<&rust::TextRange> for ffi::TextRange {
    fn from(value: &rust::TextRange) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<rust::TextRange> for ffi::TextRange {
    fn from(value: rust::TextRange) -> Self {
        (&value).into()
    }
}
