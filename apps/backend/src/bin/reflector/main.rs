use std::collections::HashSet;
use tokio_postgres::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    println!("Connecting to database at {}", database_url);

    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect(&database_url, tokio_postgres::tls::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Get all tables
    let tables = get_tables(&client).await?;

    // Collect all enum types used
    let mut used_enums = HashSet::new();
    let mut all_columns = Vec::new();

    for table in &tables {
        let columns = get_columns(&client, table).await?;
        for column in &columns {
            if is_custom_enum(&column.data_type) {
                used_enums.insert(column.data_type.clone());
            }
        }
        all_columns.push((table.clone(), columns));
    }

    // Generate the enums file
    let enums_map = get_all_enums(&client).await?;
    let mut enums_code =
        String::from("// Auto-generated database enums\nuse pgmap::postgres_enum;\n\n");
    for (enum_name, enum_values) in &enums_map {
        enums_code.push_str(&generate_enum(enum_name, enum_values));
        enums_code.push('\n');
    }
    let enums_path = "src/models/enums.rs";
    std::fs::write(enums_path, enums_code)?;
    println!("Generated enums at {}", enums_path);

    // Generate the models file
    let mut generated_code = String::from(
        "// Auto-generated database models\n\
         use uuid::Uuid;\n\
         use chrono::{DateTime, Utc};\n\
         use pgmap::FromRow;\n\
         use serde::{Serialize, Deserialize};\n\
         use utoipa::ToSchema;\n",
    );

    if !used_enums.is_empty() {
        generated_code.push_str("use super::enums::*;\n");
    }

    generated_code.push('\n');

    for (table, columns) in all_columns {
        generated_code.push_str(&generate_struct(&table, &columns));
        generated_code.push('\n');
    }

    // Write to file
    let output_path = "src/models/db.rs";
    std::fs::write(output_path, generated_code)?;
    println!("Generated models at {}", output_path);

    Ok(())
}

async fn get_tables(client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            "SELECT table_name FROM information_schema.tables 
             WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
             ORDER BY table_name",
            &[],
        )
        .await?;

    let tables: Vec<String> = rows
        .iter()
        .map(|row| row.get::<_, String>(0))
        .filter(|name| !name.starts_with('_'))
        .collect();

    Ok(tables)
}

#[derive(Debug)]
struct Column {
    name: String,
    data_type: String,
    is_nullable: bool,
}

async fn get_columns(
    client: &Client,
    table_name: &str,
) -> Result<Vec<Column>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            "SELECT c.column_name, 
                    COALESCE(t.typname, c.data_type) as data_type, 
                    c.is_nullable 
             FROM information_schema.columns c
             LEFT JOIN pg_catalog.pg_type t ON c.udt_name = t.typname
             WHERE c.table_name = $1 
             ORDER BY c.ordinal_position",
            &[&table_name],
        )
        .await?;

    let columns = rows
        .iter()
        .map(|row| Column {
            name: row.get(0),
            data_type: row.get(1),
            is_nullable: row.get::<_, String>(2) == "YES",
        })
        .collect();

    Ok(columns)
}

fn generate_struct(table_name: &str, columns: &[Column]) -> String {
    let struct_name = to_pascal_case(table_name);
    let mut output = format!(
        "#[derive(FromRow, Serialize, Deserialize, ToSchema)]\npub struct {} {{\n",
        struct_name
    );

    for column in columns {
        if column.data_type == "USER-DEFINED" || column.data_type == "ARRAY" {
            println!("{:?}", column);
        }
        let rust_type = pg_type_to_rust(&column.data_type, column.is_nullable);
        output.push_str(&format!(
            "    pub {}: {},\n",
            key_sanitizer(&column.name),
            rust_type
        ));
    }

    output.push('}');
    output
}

fn is_custom_enum(pg_type: &str) -> bool {
    matches!(
        pg_type,
        "user_role" | "feature_flag_type" | "value_type" | "match_operator" | "audience_scope"
    )
}

fn pg_type_to_rust_enum(pg_type: &str) -> &'static str {
    match pg_type {
        "user_role" => "UserRole",
        "feature_flag_type" => "FeatureFlagType",
        "value_type" => "ValueType",
        "match_operator" => "MatchOperator",
        "audience_scope" => "AudienceScope",
        _ => "String",
    }
}

fn pg_type_to_rust(pg_type: &str, is_nullable: bool) -> String {
    let base_type = if is_custom_enum(pg_type) {
        pg_type_to_rust_enum(pg_type).to_string()
    } else {
        match pg_type {
            "uuid" => "Uuid",
            "text" | "character varying" | "varchar" | "char" | "character" => "String",
            "integer" | "int4" => "i32",
            "bigint" | "int8" => "i64",
            "smallint" | "int2" => "i16",
            "real" | "float4" => "f32",
            "double precision" | "float8" => "f64",
            "boolean" | "bool" => "bool",
            "timestamp" | "timestamp without time zone" => "DateTime<Utc>",
            "timestamp with time zone" | "timestamptz" => "DateTime<Utc>",
            "date" => "chrono::NaiveDate",
            "time" | "time without time zone" => "chrono::NaiveTime",
            "json" | "jsonb" => "serde_json::Value",
            "numeric" | "decimal" => "rust_decimal::Decimal",
            "bytea" => "Vec<u8>",
            _ => "String", // Default fallback
        }
        .to_string()
    };

    if is_nullable {
        format!("Option<{}>", base_type)
    } else {
        base_type
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn key_sanitizer(key: &str) -> String {
    match key {
        "type" => "r#type".to_string(),
        other => other.to_string(),
    }
}

async fn get_all_enums(
    client: &Client,
) -> Result<std::collections::BTreeMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            "SELECT t.typname, e.enumlabel 
             FROM pg_enum e
             JOIN pg_type t ON e.enumtypid = t.oid
             WHERE t.typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')
             ORDER BY t.typname, e.enumsortorder",
            &[],
        )
        .await?;

    let mut enums_map: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for row in rows {
        let enum_name: String = row.get(0);
        let enum_value: String = row.get(1);
        enums_map
            .entry(enum_name)
            .or_insert_with(Vec::new)
            .push(enum_value);
    }

    Ok(enums_map)
}

fn generate_enum(enum_name: &str, values: &[String]) -> String {
    let rust_enum_name = pg_enum_name_to_rust(enum_name);
    let mut output = format!("postgres_enum!({}, \"{}\",\n", rust_enum_name, enum_name);

    for value in values {
        let rust_variant = to_pascal_case(value);
        output.push_str(&format!("    {} => \"{}\",\n", rust_variant, value));
    }

    output.push_str(");\n");
    output
}

fn pg_enum_name_to_rust(enum_name: &str) -> String {
    to_pascal_case(enum_name)
}
