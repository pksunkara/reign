use reign_task::{Error, Task};

pub struct Migrate {}

impl Task for Migrate {
    fn command(&self) -> String {
        "migrate".into()
    }

    fn short_about(&self) -> String {
        "".into()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}
