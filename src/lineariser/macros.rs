//! Lineariser macro helpers

#![expect(clippy::arbitrary_source_item_ordering, reason = "scope macro usage")]

/// Helper macro to create attribute keywords.
macro_rules! attr {
    ($y:ident $t:ident) => {
        $crate::parser::api::Attribute::Keyword($crate::parser::api::AttributeKeyword::$y(
            $crate::parser::api::$y::$t,
        ))
    };
}

pub(super) use attr;
