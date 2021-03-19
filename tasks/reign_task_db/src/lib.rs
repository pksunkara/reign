#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_task_db/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

mod migrate;
mod revert;
mod status;

use migrate::Migrate;
use revert::Revert;
use status::Status;

use reign_task::Tasks;

pub fn task() -> Tasks {
    Tasks::new("db")
        .task(Migrate {})
        .task(Status {})
        .task(Revert {})
}
