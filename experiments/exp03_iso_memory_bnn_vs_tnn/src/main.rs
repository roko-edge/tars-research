//! exp03_iso_memory_bnn_vs_tnn — iso-memory comparison between a dense binary
//! network {-1, +1} and a sparse ternary network {-1, 0, +1}.

const BANNER: &str = "exp03_iso_memory_bnn_vs_tnn | Hypothesis: a sparse TNN fits the bit budget of a dense BNN while its density stays below 1/(1 + log2 n).";

/// Weight-storage cost of a dense BNN: one sign bit per weight.
pub fn dense_bnn_bits(n_weights: usize) -> u64 {
    n_weights as u64
}

/// Weight-storage cost of a sparse TNN: only active weights are stored, each
/// as an index (log2 n bits) + sign (1 bit).
pub fn sparse_tnn_bits(n_weights: usize, density: f64) -> f64 {
    let index_bits = (n_weights as f64).log2();
    density * n_weights as f64 * (index_bits + 1.0)
}

/// Maximum density at which a sparse TNN still fits the bit budget of a dense
/// BNN with the same number of weights (iso-memory frontier).
pub fn iso_memory_density(n_weights: usize) -> f64 {
    1.0 / (1.0 + (n_weights as f64).log2())
}

fn main() {
    println!("{BANNER}");
    let n: usize = 1024;
    println!(
        "demo: layer with {n} weights — dense BNN = {} bits; sparse TNN at 8% density = {:.0} bits; iso-memory frontier = density {:.3}",
        dense_bnn_bits(n),
        sparse_tnn_bits(n, 0.08),
        iso_memory_density(n),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tars::math::ternary_rng;

    #[test]
    fn tnn_wins_below_and_loses_above_the_iso_memory_frontier() {
        // Given: a layer with 1024 weights (log2 = 10, frontier at 1/11).
        let n: usize = 1024;
        let frontier = iso_memory_density(n);

        // When: we compare costs below and above the frontier.
        // Then: below, the sparse TNN fits the dense-BNN budget; above, the
        // dense BNN is the cheaper option.
        assert!(sparse_tnn_bits(n, 0.5 * frontier) < dense_bnn_bits(n) as f64);
        assert!(sparse_tnn_bits(n, 0.08) < dense_bnn_bits(n) as f64);
        assert!(sparse_tnn_bits(n, 1.5 * frontier) > dense_bnn_bits(n) as f64);
    }

    #[test]
    fn engine_ternary_rng_generates_only_the_ternary_basis() {
        // Given: the engine's ternary generator (basis scaled by sqrt(3/2)
        // for unit variance, per tars::math).
        let scale = (3.0_f32 / 2.0).sqrt();

        // When: we sample ternary weights.
        // Then: every value belongs exactly to {-scale, 0, +scale}.
        for _ in 0..512 {
            let w = ternary_rng();
            let in_basis = w == 0.0 || (w - scale).abs() < 1e-6 || (w + scale).abs() < 1e-6;
            assert!(in_basis, "value outside the ternary basis: {w}");
        }
    }
}
