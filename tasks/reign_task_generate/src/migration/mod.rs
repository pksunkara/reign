use anyhow::anyhow;
use inflector::cases::snakecase::to_snake_case;
use reign_task::{serde_json::json, workspace_dir, Error, Task, Template};

pub struct Migration {}

impl Task for Migration {
    fn command(&self) -> String {
        "migration".into()
    }

    fn short_about(&self) -> String {
        "Generate a reign migration".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        if args.len() < 1 {
            return Err(anyhow!("please specify the name of the migration"))?;
        }

        let name = to_snake_case(&args[0]);

        let ws_dir = workspace_dir()?;

        Template::new(&ws_dir)
            .render(
                &["migrations", &name, "up.sql"],
                include_str!("template/migrations/migration/up.sql"),
                json!({}),
            )
            .render(
                &["migrations", &name, "down.sql"],
                include_str!("template/migrations/migration/down.sql"),
                json!({}),
            )
            .run()
    }
}
