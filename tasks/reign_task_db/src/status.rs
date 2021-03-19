use reign_task::{Error, Task};

pub struct Status {}

impl Task for Status {
    fn command(&self) -> String {
        "status".into()
    }

    fn short_about(&self) -> String {
        "".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}
