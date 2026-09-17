//! Storage-side half of the table-of-content shortcut (prototype).
//!
//! The shortcut contract lives in `collection::toc_shortcut` so that
//! `lib/collection` never depends on `lib/storage`; the table of content
//! implements it here and resolves collection names on the caller's behalf.

use collection::toc_shortcut::TocShortcut;

use super::TableOfContent;
use crate::content_manager::errors::StorageError;
use crate::rbac::CollectionPass;

impl TocShortcut for TableOfContent {}

impl TableOfContent {
    /// Resolve `collection` and run the shortcut hook against it.
    pub async fn touch_collection(
        &self,
        collection: &CollectionPass<'_>,
    ) -> Result<(), StorageError> {
        let collection = self.get_collection(collection).await?;
        TocShortcut::touch(self, &collection);
        Ok(())
    }
}
