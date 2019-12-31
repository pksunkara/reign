use dotenv;
use env_logger;
use std::env;

fn build_env_file_heirarchy(environment: String) -> Vec<String> {
    let mut heirarchy: Vec<String> = environment.split('.').map(String::from).collect();
    let length = heirarchy.len();

    for i in 0..length {
        for j in i + 1..length {
            heirarchy[i] = format!("{}.{}", heirarchy[j], heirarchy[i]);
        }
    }

    heirarchy.reverse();
    heirarchy
}

fn load_env_files() {
    let environment = env::var("REIGN_ENV").unwrap_or_else(|_| "development".to_string());

    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    for item in build_env_file_heirarchy(environment).iter() {
        dotenv::from_filename(&format!(".env.{}", item)).ok();
        dotenv::from_filename(&format!(".env.{}.local", item)).ok();
    }
}

pub fn boot() {
    load_env_files();

    // TODO: Allow custom loggers by adding an option to exclude this call
    env_logger::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_env_file_heirarchy() {
        assert_eq!(
            build_env_file_heirarchy(String::from("joe.qa.staging")),
            ["staging", "staging.qa", "staging.qa.joe"]
        );
    }
}
