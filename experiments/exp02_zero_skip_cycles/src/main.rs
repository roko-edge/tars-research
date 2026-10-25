//! exp02_zero_skip_cycles — measurement of the cycles saved by zero-skip in
//! Conv2D + ReLU, to be validated with Verilator simulation of the RTL.

use tars::math::{relu, relu_deriv_from_z};

const BANNER: &str = "exp02_zero_skip_cycles | Hypothesis: zero-skip in Conv2D + ReLU saves MAC cycles proportional to the fraction of exact-zero ReLU outputs.";

/// Fraction of exact-zero ReLU outputs — the fraction of MACs that zero-skip
/// can bypass during inference.
pub fn skippable_mac_fraction(pre_activations: &[f32]) -> f32 {
    let zeros = pre_activations.iter().filter(|&&z| relu(z) == 0.0).count();
    zeros as f32 / pre_activations.len() as f32
}

/// Fraction of training updates kept: where the ReLU derivative is zero the
/// weight gradient is zero and the update can be skipped as well.
pub fn kept_update_fraction(pre_activations: &[f32]) -> f32 {
    let updates = pre_activations
        .iter()
        .filter(|&&z| relu_deriv_from_z(z) != 0.0)
        .count();
    updates as f32 / pre_activations.len() as f32
}

/// Demo pre-activations: symmetric around zero, half negative, half positive,
/// no exact zero.
fn demo_pre_activations() -> Vec<f32> {
    (0..16)
        .map(|i| {
            if i % 2 == 0 {
                -1.0 - i as f32 * 0.01
            } else {
                1.0 + i as f32 * 0.01
            }
        })
        .collect()
}

fn main() {
    println!("{BANNER}");
    let z = demo_pre_activations();
    println!(
        "demo: {:.1}% of MACs skippable at inference, {:.1}% of training updates kept",
        100.0 * skippable_mac_fraction(&z),
        100.0 * kept_update_fraction(&z),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_relu_zeros_exactly_the_negative_half() {
        // Given: 16 symmetric pre-activations — 8 negative, 8 positive.
        let z = demo_pre_activations();

        // When: we measure the skippable (inference) and kept-update
        // (training) fractions.
        let skippable = skippable_mac_fraction(&z);
        let kept_updates = kept_update_fraction(&z);

        // Then: ReLU zeroes exactly the negative half, and the zero
        // derivative keeps updates only for the positive half.
        assert_eq!(skippable, 0.5);
        assert_eq!(kept_updates, 0.5);
        assert_eq!(relu(-3.0), 0.0);
        assert_eq!(relu(3.0), 3.0);
        assert_eq!(relu_deriv_from_z(-3.0), 0.0);
        assert_eq!(relu_deriv_from_z(3.0), 1.0);
    }
}
