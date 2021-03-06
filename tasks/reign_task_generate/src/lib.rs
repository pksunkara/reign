#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_task_generate/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

mod model;
mod ws;

use model::Model;

use reign_task::Tasks;

pub fn task() -> Tasks {
    Tasks::new("generate").task(Model {})
}
