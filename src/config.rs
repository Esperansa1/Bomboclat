use std::env;
use std::process;

pub struct Config {
    pub csgoroll_session: String,
    pub cf_clearance: String,
    pub capsolver_api_key: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let csgoroll_session = Self::require_var("CSGOROLL_SESSION");
        let cf_clearance = Self::require_var("CF_CLEARANCE");
        let capsolver_api_key = Self::require_var("CAPSOLVER_API_KEY");

        println!(
            "[config] All credentials loaded: CSGOROLL_SESSION=***, CF_CLEARANCE=***, CAPSOLVER_API_KEY=***"
        );

        Config {
            csgoroll_session,
            cf_clearance,
            capsolver_api_key,
        }
    }

    /// Returns the full Cookie header value to send on every request.
    pub fn cookie_header(&self) -> String {
        format!(
            "session={}; cf_clearance={}",
            self.csgoroll_session, self.cf_clearance
        )
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
