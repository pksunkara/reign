use anyhow::anyhow;
use atoi::FromRadix10;
use inflector::{
    cases::{classcase::to_class_case, snakecase::to_snake_case, tablecase::to_table_case},
    string::pluralize::to_plural,
};
use reign_task::{serde_json::json, workspace_dir, Error, Task, Template};
use serde::Serialize;

use std::path::PathBuf;

#[derive(Serialize)]
struct Field {
    name: String,
    rust: String,
    sql: String,
    not_null: bool,
}

impl Field {
    fn new(name: &str, from: &str) -> Result<Self, anyhow::Error> {
        let from = from.to_lowercase();

        let (not_null, from) = if from.starts_with("option<") && from.ends_with(">") {
            (false, from[7..(from.len() - 1)].to_string())
        } else {
            (true, from)
        };

        let (sql, rust) = match from.as_str() {
            "string" | "text" => ("TEXT", "String"),
            "i32" | "u32" | "int" | "integer" => ("INTEGER", "i32"),
            "i16" | "u16" | "smallint" => ("SMALLINT", "i16"),
            "i64" | "u64" | "bigint" => ("BIGINT", "i64"),
            "bool" | "boolean" => ("BOOLEAN", "bool"),
            "date" => ("DATE", "chrono::NaiveDate"),
            "time" => ("TIME", "chrono::NaiveTime"),
            "timestamp" => ("TIMESTAMP", "chrono::NaiveDateTime"),
            v => {
                return Err(anyhow!(
                    "unable to recognize `{}` as rust type or postgres type",
                    v
                ))
            }
        };

        Ok(Self {
            name: to_snake_case(name),
            rust: if not_null {
                rust.into()
            } else {
                format!("Option<{}>", rust)
            },
            sql: sql.into(),
            not_null,
        })
    }
}

pub struct Model {}

impl Task for Model {
    fn command(&self) -> String {
        "model".into()
    }

    fn short_about(&self) -> String {
        "Generate a reign model".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        if args.len() < 1 {
            return Err(anyhow!("please specify the name of the model"))?;
        }

        let tablename = to_plural(&to_table_case(&args[0]));
        let fields = &args[1..]
            .iter()
            .map(|x| {
                let parts = x.split(':').collect::<Vec<_>>();

                if parts.len() != 2 {
                    Err(anyhow!(
                        "`{}` is not in the format of column_name:RustType",
                        x
                    ))
                } else {
                    Field::new(parts[0], parts[1])
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let ws_dir = workspace_dir()?;
        let migration = format!(
            "{:04}_create_{}",
            find_next_migration_number(&ws_dir)?,
            tablename
        );

        Template::new(&ws_dir)
            .render(
                &["migrations", &migration, "up.sql"],
                include_str!("template/migrations/migration/up.sql"),
                json!({
                    "name": tablename,
                    "fields": fields,
                }),
            )
            .render(
                &["migrations", &migration, "down.sql"],
                include_str!("template/migrations/migration/down.sql"),
                json!({
                    "name": tablename,
                }),
            )
            .render(
                &["src", "models", &format!("{}.rs", tablename)],
                include_str!("template/src/models/model.rs"),
                json!({
                    "name": to_class_case(&args[0]),
                    "fields": fields,
                }),
            )
            .edit(&["src", "models", "mod.rs"], move |data| {
                if data.contains(&format!("pub mod {}", tablename)) {
                    Ok(data)
                } else {
                    Ok(format!(
                        "{}\n\npub mod {};\npub use {1}::*;\n",
                        data, tablename
                    ))
                }
            })
            .run()
    }
}

fn find_next_migration_number(path: &PathBuf) -> Result<u16, Error> {
    let mut path = path.clone();
    path.push("migrations");

    let mut ret = 0;

    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            let (num, _) = u16::from_radix_10(entry.file_name().to_string_lossy().as_bytes());

            if num > ret {
                ret = num;
            }
        }
    }

    Ok(ret + 1)
}
