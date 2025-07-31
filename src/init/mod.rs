pub mod dev;
pub mod release;

/// Load environment variables from `.env` file adjacent to the binary.
/// Crashes the program if not found.
#[macro_export]
macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(&format!("[env] {} is not set", $name))
    };
}
