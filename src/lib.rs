use pyo3::prelude::*;

const PRECISION_SCALE: i128 = 100_000_000; // 8 Decimal Places (1.00000000)
const DECOY_MASK: i128 = 0x5A5A_A5A5_5A5A_A5A5; // Bitwise Signature ya kutambua Decoys

#[pyclass]
pub struct ApexEngine {
    // Hidden internal state
}

#[pymethods]
impl ApexEngine {
    #[new]
    fn new() -> Self {
        ApexEngine {}
    }

    /// Encode value into stream with Reverse Decoys (Obfuscation)
    fn pack_transaction(&self, integer_part: i128, fraction_part: i128) -> Vec<i128> {
        // Convert to 8-decimal fixed-point
        let real_scaled = (integer_part * PRECISION_SCALE) + fraction_part;
        
        // Generate Decoy Payloads
        let decoy_1 = real_scaled ^ DECOY_MASK;
        let decoy_2 = real_scaled.wrapping_add(999_999);

        // Return Stream in Reverse Order: [Decoy2, Real_Value, Decoy1]
        vec![decoy_2, real_scaled, decoy_1]
    }

    /// Execute Reverse Traversal Engine & Extract Authentic Decimal Value
    fn unpack_and_verify(&self, stream: Vec<i128>) -> PyResult<(i128, i128)> {
        let mut authentic_val: Option<i128> = None;

        // Traverse Stream REVERSEALLY (Kuanzia Nyuma kuelekea Mbele)
        for chunk in stream.iter().rev() {
            // Skim away decoys via Obfuscation Logic
            if !self.is_decoy(*chunk) {
                authentic_val = Some(*chunk);
                break;
            }
        }

        match authentic_val {
            Some(val) => {
                let integer_part = val / PRECISION_SCALE;
                let fraction_part = val % PRECISION_SCALE;
                Ok((integer_part, fraction_part))
            }
            None => Err(pyo3::exceptions::PyValueError::new_err("Corrupted Payload")),
        }
    }
}

impl ApexEngine {
    #[inline(always)]
    fn is_decoy(&self, val: i128) -> bool {
        // Obfuscated Decoy Detection Primitive
        (val ^ DECOY_MASK) == (val & !DECOY_MASK) || val % 2 == 1 && (val ^ DECOY_MASK) != 0
    }
}

#[pymodule]
fn apex_decimal_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ApexEngine>()?;
    Ok(())
}