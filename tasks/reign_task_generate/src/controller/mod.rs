use anyhow::anyhow;
use inflector::{cases::tablecase::to_table_case, string::pluralize::to_plural};
use reign_task::{serde_json::json, workspace_dir, Error, Task, Template};

pub struct Controller {}

impl Task for Controller {
    fn command(&self) -> String {
        "controller".into()
    }

    fn short_about(&self) -> String {
        "Generate a reign controller".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        if args.len() < 1 {
            return Err(anyhow!("please specify the name of the controller"))?;
        }

        let name = to_plural(&to_table_case(&args[0]));
        let actions = args[1..].to_vec();

        let ws_dir = workspace_dir()?;

        Template::new(&ws_dir)
            .render(
                &["src", "controllers", &format!("{}.rs", name)],
                include_str!("template/src/controllers/controller.rs"),
                json!({
                    "actions": actions,
                }),
            )
            .edit(&["src", "controllers", "mod.rs"], move |data| {
                if data.contains(&format!("pub mod {}", name)) {
                    Ok(data)
                } else {
                    Ok(format!("{}pub mod {};\n", data, name))
                }
            })
            .run()
    }
}
