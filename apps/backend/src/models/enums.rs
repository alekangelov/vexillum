use std::error::Error;
use tokio_postgres::types::{IsNull, ToSql, Type};

macro_rules! postgres_enum {
    ($enum_name:ident, $pg_name:expr, $($variant:ident => $str:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $enum_name {
            $($variant),*
        }

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

postgres_enum!(UserRole, "user_role",
    Admin => "admin",
    User => "user",
    Viewer => "viewer"
);

postgres_enum!(FeatureFlagType, "feature_flag_type",
    Boolean => "boolean",
    Multivariate => "multivariate",
    Json => "json"
);

postgres_enum!(ValueType, "value_type",
    String => "string",
    Number => "number",
    Boolean => "boolean",
    Json => "json"
);

postgres_enum!(MatchOperator, "match_operator",
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

postgres_enum!(AudienceScope, "audience_scope",
    Global => "global",
    Inline => "inline"
);
