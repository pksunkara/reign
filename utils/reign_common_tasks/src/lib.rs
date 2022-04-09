#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

use reign_task::Task;
use reign_task_cargo::Cargo;

pub fn tasks() -> Vec<Box<dyn Task>> {
    vec![
        Box::new(
            Cargo::new("test")
                .about("Run test suite for the application")
                .args(&["test"]),
        ),
        Box::new(
            Cargo::new("build")
                .about("Build the application as a binary for production")
                .args(&["build", "--release"]),
        ),
        Box::new(reign_task_generate::task()),
        Box::new(reign_task_db::task()),
    ]
}
