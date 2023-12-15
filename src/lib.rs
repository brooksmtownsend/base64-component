cargo_component_bindings::generate!();

use base64::{engine::general_purpose::STANDARD, Engine as _};
use bindings::Guest;

struct Component;

impl Guest for Component {
    #[doc = "Encode bytes into a base64 string"]
    fn encode(input: Vec<u8>) -> String {
        STANDARD.encode(input)
    }

    #[doc = "Decode a base64 string into bytes"]
    fn decode(input: String) -> Result<Vec<u8>, String> {
        STANDARD.decode(input).map_err(|e| e.to_string())
    }
}
