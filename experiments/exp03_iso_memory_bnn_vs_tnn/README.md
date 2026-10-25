# exp03 — iso_memory_bnn_vs_tnn

Comparativo iso-memória entre uma rede densa binária {-1, +1} (BNN) e uma rede
ternária esparsa {-1, 0, +1} (TNN).

## Hipótese

Uma TNN esparsa cabe no mesmo orçamento de bits de pesos de uma BNN densa
enquanto sua densidade ficar abaixo de 1/(1 + log2 n) por camada de n pesos;
acima da fronteira, a BNN densa (1 bit/peso) é mais econômica.

## Método

1. Modelo de custo: BNN densa = 1 bit de sinal por peso; TNN esparsa armazena
   apenas pesos ativos, cada um como índice (log2 n bits) + sinal (1 bit).
2. Medir densidades reais por camada a partir de pesos ternários do motor
   (base escalada do gerador `tars::math::ternary_rng`).
3. Cruzar as densidades medidas com a fronteira iso-memória e registrar os
   custos em bits por camada.

## Como rodar

```bash
cargo run -p exp03_iso_memory_bnn_vs_tnn
cargo test -p exp03_iso_memory_bnn_vs_tnn
```

## Saída esperada

CSV consolidado em `../../data/` (conforme `../../data/README.md`): custo em
bits por camada para BNN densa e TNN esparsa, densidade medida e posição
relativa à fronteira iso-memória.

## Status

Scaffold. O modelo de custo iso-memória já está implementado e testado
(incluindo a verificação da base ternária do motor); redes reais treinadas
ainda não foram medidas.
