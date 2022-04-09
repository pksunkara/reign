#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

mod controller;
mod generator;
mod migration;
mod model;

use controller::Controller;
use generator::Generator;
use migration::Migration;
use model::Model;

use reign_task::Tasks;

pub fn task() -> Tasks {
    Tasks::new("generate")
        .task(Model {})
        .task(Controller {})
        .task(Generator {})
        .task(Migration {})
}
