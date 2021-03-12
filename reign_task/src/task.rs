use crate::Error;

pub trait Task {
    fn command(&self) -> String;

    fn short_about(&self) -> String;

    fn long_about(&self) -> String {
        self.short_about()
    }

    #[doc(hidden)]
    fn list(&self) -> Vec<(Vec<String>, String)> {
        vec![(vec![self.command()], self.short_about())]
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error>;
}
