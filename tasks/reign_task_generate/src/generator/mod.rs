use anyhow::anyhow;
use inflector::cases::{classcase::to_class_case, snakecase::to_snake_case};
use reign_task::{serde_json::json, workspace_dir, Error, Task, Template};

pub struct Generator {}

impl Task for Generator {
    fn command(&self) -> String {
        "generator".into()
    }

    fn short_about(&self) -> String {
        "Generate a reign task to generate things".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        if args.len() < 1 {
            return Err(anyhow!("please specify the name of the generator"))?;
        }

        let name = to_snake_case(&args[0]);
        let klass = to_class_case(&args[0]);

        let ws_dir = workspace_dir()?;

        Template::new(&ws_dir)
            .render(
                &["xtask", "src", &format!("{}.rs", name)],
                include_str!("template/xtask/src/generator.rs"),
                json!({
                    "name": name,
                    "klass": klass,
                }),
            )
            .edit(&["xtask", "src", "main.rs"], move |data| {
                if data.contains(&format!("mod {};", name)) {
                    Ok(data)
                } else {
                    Ok(data
                        .replacen(
                            "use reign_task::Tasks;\n",
                            &format!("use reign_task::Tasks;\n\nmod {};\n", name),
                            1,
                        )
                        .replacen(
                            ".parse();",
                            &format!(".task({}::{} {{}}).parse();", name, klass),
                            1,
                        ))
                }
            })
            .run()
    }
}
