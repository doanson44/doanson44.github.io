use crate::domain::document::MarkdownDocument;

/// Port for document persistence.
///
/// This trait defines the boundary between application logic and
/// storage infrastructure. The MVP does not require a concrete
/// implementation — documents live in reactive signals.
///
/// Future implementations:
/// - `LocalDocumentRepository` (IndexedDB/LocalStorage)
/// - `HttpDocumentRepository` (REST API)
///
/// Neither the presentation layer nor domain logic depends on
/// concrete implementations of this trait.
pub trait DocumentRepository {
    /// Persist a document.
    fn save(&self, doc: &MarkdownDocument) -> Result<(), String>;

    /// Load a document by ID. Returns `None` if not found.
    fn load(&self, id: &str) -> Result<Option<MarkdownDocument>, String>;

    /// Delete a document by ID.
    fn delete(&self, id: &str) -> Result<(), String>;

    /// List all document summaries (id, title).
    fn list(&self) -> Result<Vec<(String, String)>, String>;
}
