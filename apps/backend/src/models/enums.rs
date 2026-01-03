use std::error::Error;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

macro_rules! impl_enum {
    ($enum_name:ident, $pg_name:expr, $($variant:ident => $str:expr),*) => {
        impl ToSql for $enum_name {
            fn to_sql(
                &self,
                ty: &Type,
                out: &mut tokio_postgres::types::private::BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                let s = match self {
                    $($enum_name::$variant => $str),*
                };
                <&str as ToSql>::to_sql(&s, ty, out)
            }

            fn accepts(ty: &Type) -> bool {
                ty.name() == $pg_name
            }

            fn to_sql_checked(
                &self,
                ty: &Type,
                out: &mut tokio_postgres::types::private::BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                if !Self::accepts(ty) {
                    return Err("Wrong type".into());
                }
                self.to_sql(ty, out)
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

impl_enum!(UserRole, "user_role",
    Admin => "admin",
    User => "user",
    Viewer => "viewer"
);

#[derive(Debug, Clone, Copy)]
pub enum FeatureFlagType {
    Boolean,
    Multivariate,
    Json,
}

impl_enum!(FeatureFlagType, "feature_flag_type",
    Boolean => "boolean",
    Multivariate => "multivariate",
    Json => "json"
);

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    Json,
}

impl_enum!(ValueType, "value_type",
    String => "string",
    Number => "number",
    Boolean => "boolean",
    Json => "json"
);

#[derive(Debug, Clone, Copy)]
pub enum MatchOperator {
    Eq,
    Neq,
    In,
    Nin,
    Gt,
    Lt,
    Gte,
    Lte,
    Contains,
    Ncontains,
}

impl_enum!(MatchOperator, "match_operator",
    Eq => "eq",
    Neq => "neq",
    In => "in",
    Nin => "nin",
    Gt => "gt",
    Lt => "lt",
    Gte => "gte",
    Lte => "lte",
    Contains => "contains",
    Ncontains => "ncontains"
);

#[derive(Debug, Clone, Copy)]
pub enum AudienceScope {
    Global,
    Inline,
}

impl_enum!(AudienceScope, "audience_scope",
    Global => "global",
    Inline => "inline"
);
