use dotenvy::from_filename;
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

pub(crate) fn load_env_files() {
    let environment = env::var("REIGN_ENV").unwrap_or("development".to_string());
    let heirarchy = build_env_file_heirarchy(environment);

    from_filename(".env").ok();

    for item in &heirarchy {
        from_filename(&format!(".env.{}", item)).ok();
    }

    from_filename(".env.local").ok();

    for item in &heirarchy {
        from_filename(&format!(".env.{}.local", item)).ok();
    }
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
