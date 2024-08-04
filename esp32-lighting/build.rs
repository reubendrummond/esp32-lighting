use std::{env, path::Path};

fn main() {
    embuild::espidf::sysenv::output();

    let is_ci = env::var("CI").is_ok();
    let env_file = if is_ci {
        Path::new("../.env.example")
    } else {
        Path::new("../.env")
    };

    let config = dotenv_build::Config {
        filename: std::path::Path::new(env_file),
        fail_if_missing_dotenv: true,
        ..Default::default()
    };

    dotenv_build::output(config).unwrap();
}
