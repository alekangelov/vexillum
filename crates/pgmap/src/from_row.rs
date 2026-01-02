use tokio_postgres::Row;

/// Trait for types that can be constructed from a PostgreSQL row
pub trait FromRow: Sized {
    /// Convert a tokio_postgres Row into Self
    fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>>;
    fn from_rows(rows: &[Row]) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        rows.iter().map(Self::from_row).collect()
    }
}

/// Derive macro for FromRow
///
/// # Example
/// ```ignore
/// #[derive(FromRow)]
/// struct User {
///     id: i32,
///     name: String,
/// }
/// ```
pub use pgmap_derive::FromRow;
