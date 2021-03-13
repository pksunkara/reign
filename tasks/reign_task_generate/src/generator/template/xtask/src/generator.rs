use reign_task::{serde_json::json, workspace_dir, Error, Task, Template};

pub struct {{klass}} {}

impl Task for {{klass}} {
    fn command(&self) -> String {
        "{{name}}".into()
    }

    fn short_about(&self) -> String {
        "Generate a {{name}}".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        let ws_dir = workspace_dir()?;

        Template::new(&ws_dir)
            .run()
    }
}
