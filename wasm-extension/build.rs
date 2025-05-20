/// Loads environment variables from `.env.frontend` at build time
/// and injects them as `env!` constants via `cargo:rustc-env`.
fn main() {
    const DOTENV_PATH: &str = ".env.frontend";

    if let Ok(contents) = std::fs::read_to_string(DOTENV_PATH) {
        for line in contents.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                println!("cargo:rerun-if-env-changed={}", key);
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    } else {
        println!("cargo:warning=No .env.frontend file found. Skipping env injection.");
    }
}
