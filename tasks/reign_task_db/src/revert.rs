use reign_task::{Error, Task};

pub struct Revert {}

impl Task for Revert {
    fn command(&self) -> String {
        "revert".into()
    }

    fn short_about(&self) -> String {
        "".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}
