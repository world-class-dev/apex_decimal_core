// Fixed-Point Precision Vector Decoy Stripping & Reverse Execution
pub struct ObfuscatedPayload {
    pub raw_stream: Vec<i128>, // has real numbers and Decoy linked
}

impl ObfuscatedPayload {
    // Inject Decoys and change Stream system
    pub fn encode_value(real_value: i128, salt: i128) -> Vec<i128> {
        let scaled = real_value * 100_000_000; // Shift 8 Decimals
        let decoy1 = scaled ^ 0xDEADBEEF;       // Decoy Number 1
        let decoy2 = scaled.wrapping_add(salt); // Decoy Number 2
        
        // Return Stream to reverse (Reverse) with Decoy Primitives
        vec![decoy2, scaled, decoy1]
    }

    // Execute Math to bake back (Reverse Processing) and skip Decoys
    pub fn decode_and_compute_reverse(stream: &[i128]) -> i128 {
        // Traversal from last index to beginning (Reverse)
        let mut target_val: i128 = 0;
        
        for val in stream.iter().rev() {
            // Skim and identify Real Value unsing Bitwise Masking
            if !Self::is_decoy(val) {
                target_val = *val;
                break;
            }
        }
        target_val
    }

    #[inline(always)]
    fn is_decoy(val: &i128) -> bool {
        // Pattern Recognition Match ya Decoys (Fast Bitwise Validation)
        (*val & 0xDEADBEEF) == 0xDEADBEEF
    }
}
