use spow::wasm::pow_work;
use wasm_bindgen::prelude::*;

mod utils;

/// Calculates the Proof of Work for the given challenge
#[wasm_bindgen]
pub async fn pow_work_wasm(challenge: &str) -> Option<String> {
    #[cfg(feature = "debug")]
    utils::set_panic_hook();
    pow_work(challenge).ok()
}
