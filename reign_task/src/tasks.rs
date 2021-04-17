use crate::{error::INTERNAL_ERR, Error, Task};

use oclif::finish;

use std::{collections::HashMap, env::args};

/// Reign task for grouping other tasks
///
/// Tasks added later can override the tasks added before
pub struct Tasks {
    name: String,
    tasks: HashMap<String, Box<dyn Task>>,
}

impl Tasks {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            tasks: HashMap::new(),
        }
    }

    pub fn task<T>(mut self, task: T) -> Self
    where
        T: Task + 'static,
    {
        let command = task.command();

        self.tasks.insert(command, Box::new(task));
        self
    }

    pub fn tasks(mut self, tasks: Vec<Box<dyn Task>>) -> Self {
        self.tasks
            .extend(tasks.into_iter().map(|t| (t.command(), t)));
        self
    }

    pub fn parse(&self) {
        let args = args().into_iter().skip(1).collect::<Vec<_>>();

        finish(self.run(args));
    }
}

impl Task for Tasks {
    fn command(&self) -> String {
        self.name.clone()
    }

    fn short_about(&self) -> String {
        "Group of tasks".into()
    }

    #[doc(hidden)]
    fn list(&self) -> Vec<(Vec<String>, String)> {
        self.tasks.iter().fold(vec![], |mut r, t| {
            let list = t.1.list();

            r.extend(list.into_iter().map(|(name, about)| {
                let mut r = vec![self.name.clone()];

                r.extend(name.into_iter());

                (r, about)
            }));

            r
        })
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        if self.name == "reign" {
            println!("");
        }

        if args.len() < 1 {
            return Err(Error::NoArgs(self.name.clone()));
        }

        let (name, rest) = args.split_first().expect(INTERNAL_ERR);

        if name == "tasks" {
            let mut list = self
                .list()
                .into_iter()
                .map(|(name, about)| (name.join(" "), about))
                .collect::<Vec<_>>();

            // Get maximum name length
            let max = list.iter().fold(0, |acc, x| usize::max(acc, x.0.len()));

            // Sort by name
            list.sort_by(|a, b| a.0.cmp(&b.0));

            for (name, about) in list {
                println!("{:width$}\t{}", name, about, width = max);
            }

            return Ok(());
        }

        let task = self
            .tasks
            .get(name)
            .ok_or_else(|| Error::NoTask(name.to_string()))?;

        task.run(rest.to_vec())
    }
}
