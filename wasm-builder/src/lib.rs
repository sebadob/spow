use wasm_bindgen::prelude::*;

#[cfg(feature = "server")]
use spow::pow::Pow as SPow;

mod utils;

#[cfg(feature = "server")]
#[wasm_bindgen]
pub struct Pow;

#[cfg(feature = "server")]
#[wasm_bindgen]
impl Pow {
    /// Initialize the PoW backend with a random secret.
    ///
    /// This or `init()` must be called before creating new `Pow` instances.
    ///
    /// You can call `init()` instead of `init_random()` to initialize with a chosen secret, which
    /// is necessary if multiple backends must be allowed to validate challenges.
    pub fn init_random() -> Result<(), String> {
        SPow::init_random().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Initialize the PoW backend with a chosen secret.
    ///
    /// This or `init_random()` must be called before creating new `Pow` instances.
    pub fn init(secret: String) {
        SPow::init(secret)
    }

    /// Create a new PoW challenge.
    pub fn build_challenge(valid_seconds: u32, difficulty: Option<u8>) -> Result<String, String> {
        if let Some(difficulty) = difficulty {
            SPow::with_difficulty(difficulty, valid_seconds)
        } else {
            SPow::new(valid_seconds)
        }
            .map_err(|e| e.to_string())
            .map(|pow| pow.build_challenge())
    }

    /// Perform the work and generate a PoW
    pub fn work(challenge: &str) -> Result<String, String> {
        SPow::work(challenge).map_err(|e| e.to_string())
    }

    /// Validate a solved PoW
    ///
    /// It will return the challenge after successful validation, which could be used do implement
    /// re-use mechanisms or something like that.
    pub fn validate(pow: &str) -> Result<String, String> {
        let challenge = SPow::validate(pow).map_err(|e| e.to_string())?;
        Ok(challenge.to_string())
    }
}

/// Calculates the Proof of Work for the given challenge
#[cfg(not(feature = "server"))]
#[wasm_bindgen]
pub async fn pow_work_wasm(challenge: &str) -> Option<String> {
    #[cfg(feature = "debug")]
    utils::set_panic_hook();
    spow::wasm::pow_work(challenge).ok()
}
