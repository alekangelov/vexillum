use pgmap::postgres_enum;

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
