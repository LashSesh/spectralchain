/*!
 * Steganography Operator (T)
 *
 * Blueprint Formel: T(m') = Embed(m', Carrier)
 *
 * Implementierung:
 * - Zero-Width Steganographie (Text)
 * - LSB Steganographie (Bilder)
 * - Frequency-Domain Steganographie (Audio - Placeholder)
 */

use crate::{QuantumOperator, QuantumOpsError, Result};
use serde::{Deserialize, Serialize};

/// Carrier-Typ für Steganographie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarrierType {
    /// Zero-Width Characters in Text
    ZeroWidth(String),
    /// LSB in Image (PNG)
    Image(Vec<u8>),
    /// Placeholder für Audio-Steganographie
    Audio(Vec<u8>),
}

/// Steganography Operator
pub struct SteganographyOperator;

impl SteganographyOperator {
    pub fn new() -> Self {
        Self
    }

    /// Embed payload in carrier
    pub fn embed(&self, payload: &[u8], carrier: CarrierType) -> Result<Vec<u8>> {
        match carrier {
            CarrierType::ZeroWidth(text) => self.embed_zero_width(payload, &text),
            CarrierType::Image(image_data) => self.embed_lsb(payload, &image_data),
            CarrierType::Audio(_) => Err(QuantumOpsError::NotSupported(
                "Audio steganography not yet implemented".to_string(),
            )),
        }
    }

    /// Extract payload from steganographic data
    pub fn extract(&self, stego_data: &[u8], carrier_type_hint: &str) -> Result<Vec<u8>> {
        match carrier_type_hint {
            "zero_width" => self.extract_zero_width(stego_data),
            "image" => self.extract_lsb(stego_data),
            _ => Err(QuantumOpsError::InvalidInput(format!(
                "Unknown carrier type: {}",
                carrier_type_hint
            ))),
        }
    }

    /// Zero-Width Steganography (Text-based)
    ///
    /// Uses Unicode zero-width characters to encode binary data
    fn embed_zero_width(&self, payload: &[u8], text: &str) -> Result<Vec<u8>> {
        const ZERO_WIDTH_SPACE: char = '\u{200B}';
        const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';

        let mut result = String::with_capacity(text.len() + payload.len() * 8);

        // Add original text
        result.push_str(text);

        // Encode payload as zero-width characters
        for byte in payload {
            for bit in (0..8).rev() {
                if (byte >> bit) & 1 == 1 {
                    result.push(ZERO_WIDTH_NON_JOINER);
                } else {
                    result.push(ZERO_WIDTH_SPACE);
                }
            }
        }

        Ok(result.into_bytes())
    }

    /// Extract from zero-width steganography
    fn extract_zero_width(&self, stego_data: &[u8]) -> Result<Vec<u8>> {
        const ZERO_WIDTH_SPACE: char = '\u{200B}';
        const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';

        let text = String::from_utf8(stego_data.to_vec())
            .map_err(|e| QuantumOpsError::SteganographyError(format!("Invalid UTF-8: {}", e)))?;

        let mut payload = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for ch in text.chars() {
            if ch == ZERO_WIDTH_SPACE || ch == ZERO_WIDTH_NON_JOINER {
                current_byte <<= 1;
                if ch == ZERO_WIDTH_NON_JOINER {
                    current_byte |= 1;
                }
                bit_count += 1;

                if bit_count == 8 {
                    payload.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
        }

        Ok(payload)
    }

    /// LSB Steganography (Image-based)
    ///
    /// Embeds data in least significant bits of image pixels
    fn embed_lsb(&self, payload: &[u8], image_data: &[u8]) -> Result<Vec<u8>> {
        // Simple LSB implementation
        if payload.len() * 8 > image_data.len() {
            return Err(QuantumOpsError::SteganographyError(
                "Payload too large for carrier image".to_string(),
            ));
        }

        let mut result = image_data.to_vec();
        let mut bit_index = 0;

        for byte in payload {
            for bit_pos in (0..8).rev() {
                let bit = (byte >> bit_pos) & 1;
                result[bit_index] = (result[bit_index] & 0xFE) | bit;
                bit_index += 1;
            }
        }

        Ok(result)
    }

    /// Extract from LSB steganography
    fn extract_lsb(&self, stego_data: &[u8]) -> Result<Vec<u8>> {
        // Extract until we hit a null terminator or run out of data
        let mut payload = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for &pixel in stego_data {
            current_byte = (current_byte << 1) | (pixel & 1);
            bit_count += 1;

            if bit_count == 8 {
                if current_byte == 0 {
                    break; // Null terminator
                }
                payload.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        Ok(payload)
    }
}

impl Default for SteganographyOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Input für Steganographie
#[derive(Debug, Clone)]
pub struct SteganographyInput {
    pub payload: Vec<u8>,
    pub carrier: CarrierType,
}

impl QuantumOperator for SteganographyOperator {
    type Input = SteganographyInput;
    type Output = Vec<u8>;
    type Params = ();

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        self.embed(&input.payload, input.carrier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_width_roundtrip() {
        let op = SteganographyOperator::new();
        let payload = b"Secret message";
        let carrier = "This is a public message.".to_string();

        let stego = op
            .embed(payload, CarrierType::ZeroWidth(carrier.clone()))
            .unwrap();

        // Verify original text is still present
        let stego_str = String::from_utf8(stego.clone()).unwrap();
        assert!(stego_str.starts_with("This is a public message."));

        // Extract payload
        let extracted = op.extract(&stego, "zero_width").unwrap();
        assert_eq!(&extracted[..], &payload[..]);
    }

    #[test]
    fn test_lsb_roundtrip() {
        let op = SteganographyOperator::new();
        let payload = b"Hidden\0"; // Null-terminated
        let carrier = vec![128u8; 1000]; // Dummy image data

        let stego = op.embed(payload, CarrierType::Image(carrier)).unwrap();
        assert_eq!(stego.len(), 1000);

        // Extract payload
        let extracted = op.extract(&stego, "image").unwrap();
        assert_eq!(&extracted[..], b"Hidden");
    }

    #[test]
    fn test_lsb_payload_too_large() {
        let op = SteganographyOperator::new();
        let payload = vec![0u8; 1000];
        let carrier = vec![128u8; 100]; // Too small

        let result = op.embed(&payload, CarrierType::Image(carrier));
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_payload() {
        let op = SteganographyOperator::new();
        let payload: &[u8] = b"";
        let carrier = "Text".to_string();

        let stego = op.embed(payload, CarrierType::ZeroWidth(carrier)).unwrap();
        let stego_str = String::from_utf8(stego.clone()).unwrap();
        assert_eq!(stego_str, "Text");
    }
}
