/// Creates a PostgreSQL enum type that implements `ToSql` for seamless database integration.
///
/// This macro generates:
/// - A public Rust enum with `Debug`, `Clone`, and `Copy` derives
/// - A `ToSql` implementation to serialize the enum to PostgreSQL
/// - Support for PostgreSQL enum type checking and validation
///
/// # Arguments
///
/// - `$enum_name`: The name of the enum to create
/// - `$pg_name`: The name of the PostgreSQL enum type (as a string literal)
/// - `$variant => $str`: Pairs mapping enum variants to their PostgreSQL string representations
///
/// # Example
///
/// ```ignore
/// postgres_enum!(
///   Status,  "status",
///   Active => "active",
///   Inactive => "inactive"
/// );
///
/// // Generates:
/// // pub enum Status { Active, Inactive }
/// // impl ToSql for Status { ... }
/// ```
#[macro_export]
macro_rules! postgres_enum {
    ($enum_name:ident, $pg_name:expr, $($variant:ident => $str:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum $enum_name {
            $($variant),*
        }

        impl ::tokio_postgres::types::ToSql for $enum_name {
            fn to_sql(
                &self,
                ty: &::tokio_postgres::types::Type,
                out: &mut ::tokio_postgres::types::private::BytesMut,
            ) -> ::std::result::Result<::tokio_postgres::types::IsNull, ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Sync + ::std::marker::Send>> {
                let s = match self {
                    $($enum_name::$variant => $str),*
                };
                <&str as ::tokio_postgres::types::ToSql>::to_sql(&s, ty, out)
            }

            fn accepts(ty: &::tokio_postgres::types::Type) -> bool {
                ty.name() == $pg_name
            }

            fn to_sql_checked(
                &self,
                ty: &::tokio_postgres::types::Type,
                out: &mut ::tokio_postgres::types::private::BytesMut,
            ) -> ::std::result::Result<::tokio_postgres::types::IsNull, ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Sync + ::std::marker::Send>> {
                if !Self::accepts(ty) {
                    return ::std::result::Result::Err("Wrong type".into());
                }
                self.to_sql(ty, out)
            }
        }

        impl<'a> ::tokio_postgres::types::FromSql<'a> for $enum_name {
            fn from_sql(
                _ty: &::tokio_postgres::types::Type,
                raw: &'a [u8],
            ) -> ::std::result::Result<Self, ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Sync + ::std::marker::Send>> {
                match ::std::str::from_utf8(raw)? {
                    $($str => ::std::result::Result::Ok($enum_name::$variant)),*,
                    s => ::std::result::Result::Err(::std::format!("invalid {}: {}", stringify!($enum_name), s).into()),
                }
            }

            fn accepts(ty: &::tokio_postgres::types::Type) -> bool {
                ty.name() == $pg_name
            }
        }
    };
}
