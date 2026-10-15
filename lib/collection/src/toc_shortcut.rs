//! Domain-side half of the table-of-content shortcut (prototype).
//!
//! `lib/collection` must not depend on `lib/storage` (the table-of-content layer
//! above it), so the shortcut is expressed here as a contract that the table of
//! content implements. This crate only knows how to touch a [`Collection`].

use crate::collection::Collection;

/// Implemented by whatever owns the collections (the storage table of content).
pub trait TocShortcut {
    /// Touch an already-resolved collection.
    fn touch(&self, collection: &Collection) {
        collection.touch();
    }
}
