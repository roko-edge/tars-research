//! exp01_subbit_entropy — practical proof that the sub-bit packing
//! `[skip_zeros | sign_bit]` costs less than 1 bit per weight on sparse
//! ternary layers.

use tars::math::xavier;

const BANNER: &str = "exp01_subbit_entropy | Hypothesis: the [skip_zeros | sign_bit] packing costs less than 1 bit/trit on sparse ternary layers.";

/// Largest active-weight token (skip = 14); bit 0 carries the sign.
const TOKEN_ACTIVE_MAX: u8 = 29;
/// Long-jump token: advances 15 positions without touching the accumulator.
const TOKEN_LONG_JUMP: u8 = 30;
/// End-of-row token: fires the per-neuron epilogue.
const TOKEN_EOR: u8 = 31;
/// Token width, in bits.
const TOKEN_BITS: f64 = 5.0;

/// Local sign-bit convention: 0 = weight +1, 1 = weight -1.
fn sign_bit(trit: i8) -> u8 {
    if trit < 0 { 1 } else { 0 }
}

/// Emits long jumps until the zero gap fits the skip field (<= 14) of the next
/// active token and returns that remainder.
fn flush_gap(tokens: &mut Vec<u8>, gap: u32) -> u32 {
    let mut rest = gap;
    while rest > 14 {
        tokens.push(TOKEN_LONG_JUMP);
        rest -= 15;
    }
    rest
}

/// Encodes a ternary row in the engine's 5-bit scheme: tokens 0-29 = active
/// weight (skip | sign), 30 = long jump, 31 = end of row. Trailing zeros cost
/// only the EOR token, no jumps.
pub fn encode_row(trits: &[i8]) -> Vec<u8> {
    let mut tokens = Vec::with_capacity(trits.len() / 8 + 2);
    let mut gap: u32 = 0;
    for &trit in trits {
        if trit == 0 {
            gap += 1;
            continue;
        }
        let rest = flush_gap(&mut tokens, gap);
        tokens.push((rest as u8) << 1 | sign_bit(trit));
        gap = 0;
    }
    tokens.push(TOKEN_EOR);
    tokens
}

/// Decodes a token stream back into the ternary row of `n_weights` weights;
/// independent oracle for the round-trip test.
pub fn decode_row(tokens: &[u8], n_weights: usize) -> Vec<i8> {
    let mut row = vec![0i8; n_weights];
    let mut i: usize = 0;
    for &token in tokens {
        match token {
            TOKEN_EOR => break,
            TOKEN_LONG_JUMP => i += 15,
            active => {
                i += (active >> 1) as usize;
                row[i] = if active & 1 == 1 { -1 } else { 1 };
                i += 1;
            }
        }
    }
    row
}

/// Threshold ternary quantization: |w| <= threshold becomes 0, otherwise the
/// sign is kept.
pub fn trinarize(weights: &[f32], threshold: f32) -> Vec<i8> {
    weights
        .iter()
        .map(|&w| {
            if w > threshold {
                1
            } else if w < -threshold {
                -1
            } else {
                0
            }
        })
        .collect()
}

/// Average cost of the encoded stream, in bits per weight.
pub fn bits_per_weight(tokens: &[u8], n_weights: usize) -> f64 {
    TOKEN_BITS * tokens.len() as f64 / n_weights as f64
}

fn main() {
    println!("{BANNER}");

    // Demo pipeline: Xavier weights from the engine -> threshold ternarization
    // (90% of the Xavier theoretical amplitude sqrt(3/n) keeps ~10% active) ->
    // 5-bit tokens.
    let n: usize = 784;
    let amplitude = (3.0_f32 / n as f32).sqrt();
    let trits = trinarize(&xavier(n), 0.9 * amplitude);
    let tokens = encode_row(&trits);

    let active = tokens.iter().filter(|&&t| t <= TOKEN_ACTIVE_MAX).count();
    let jumps = tokens.iter().filter(|&&t| t == TOKEN_LONG_JUMP).count();
    println!(
        "demo: {n} weights -> {} 5-bit tokens ({} active, {} long jumps, 1 EOR) = {:.3} bits/weight",
        tokens.len(),
        active,
        jumps,
        bits_per_weight(&tokens, n),
    );

    // Round-trip self-check on the demo row.
    let restored = decode_row(&tokens, n);
    let preserved = restored.iter().zip(&trits).filter(|(a, b)| a == b).count();
    println!("self-check: {preserved}/{n} weights preserved by the round trip");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reference row with 50 weights: +1 at position 0, -1 at position 31
    /// after a gap of 30 zeros, zeros through the end.
    fn reference_row() -> Vec<i8> {
        let mut trits = vec![0i8; 50];
        trits[0] = 1;
        trits[31] = -1;
        trits
    }

    #[test]
    fn packing_costs_less_than_one_bit_per_weight_on_sparse_row() {
        // Given: a sparse row of 50 weights with a 30-zero gap in the middle.
        let trits = reference_row();

        // When: we encode it in the 5-bit [skip_zeros | sign_bit] scheme.
        let tokens = encode_row(&trits);

        // Then: +1 with skip 0, two long jumps cover the 30-zero gap, -1
        // with skip 0, plus the EOR — exactly 5 tokens.
        assert_eq!(
            tokens,
            vec![0, TOKEN_LONG_JUMP, TOKEN_LONG_JUMP, 1, TOKEN_EOR]
        );

        // And: the average cost is 0.5 bits/weight — below 1 bit/trit.
        let cost = bits_per_weight(&tokens, trits.len());
        assert_eq!(cost, 0.5);
        assert!(cost < 1.0);
    }

    #[test]
    fn round_trip_preserves_row_generated_by_the_engine() {
        // Given: a real weight row from the engine's Xavier initializer,
        // ternarized at 90% of the sqrt(3/n) amplitude (~10% active weights).
        let n: usize = 784;
        let amplitude = (3.0_f32 / n as f32).sqrt();
        let trits = trinarize(&xavier(n), 0.9 * amplitude);

        // When: we encode and decode back.
        let restored = decode_row(&encode_row(&trits), n);

        // Then: positions and signs are preserved exactly.
        assert_eq!(restored, trits);
    }
}
