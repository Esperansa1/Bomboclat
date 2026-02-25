use std::env;
use std::process;

pub struct Config {
    pub csgoroll_session: String,
    pub capsolver_api_key: String,
}

impl Config {
    pub fn load() -> Self {
        // Load .env file if present; silently ignore if missing (env vars may be set externally)
        dotenvy::dotenv().ok();

        let csgoroll_session = Self::require_var("CSGOROLL_SESSION");
        let capsolver_api_key = Self::require_var("CAPSOLVER_API_KEY");

        println!(
            "[config] All credentials loaded: CSGOROLL_SESSION=***, CAPSOLVER_API_KEY=***"
        );

        Config {
            csgoroll_session,
            capsolver_api_key,
        }
    }

    fn require_var(name: &str) -> String {
        match env::var(name) {
            Ok(val) if !val.is_empty() => val,
            Ok(_) => {
                eprintln!("[config] ERROR: Environment variable '{}' is set but empty", name);
                process::exit(1);
            }
            Err(_) => {
                eprintln!(
                    "[config] ERROR: Required environment variable '{}' is not set. \
                     Set it in your environment or in a .env file.",
                    name
                );
                process::exit(1);
            }
        }
    }
}
