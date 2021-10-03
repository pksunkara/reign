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

    #[doc(hidden)]
    fn run_with_prev(&self, args: Vec<String>, _prev: Vec<String>) -> Result<(), Error> {
        self.run(args)
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error>;
}
