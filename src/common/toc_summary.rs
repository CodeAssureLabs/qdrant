//! Application-layer access to the table of content.
//!
//! REST and gRPC entrypoints must not touch `TableOfContent` directly; they
//! obtain it through the `Dispatcher` and hand it to the helpers in this module.

use storage::content_manager::toc::TableOfContent;
use storage::rbac::auth::Auth;

/// A lightweight view of the table of content, scoped to the caller's access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TocSummary {
    /// Number of collections visible to the caller.
    pub collections: usize,
}

/// Summarise the table of content as seen by `auth`.
pub async fn do_toc_summary(toc: &TableOfContent, auth: &Auth) -> TocSummary {
    let collections = toc.all_collections(auth.access("toc_direct")).await.len();
    TocSummary { collections }
}
