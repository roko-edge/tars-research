# exp02 — zero_skip_cycles

Medição dos ciclos economizados pelo zero-skip em Conv2D + ReLU, via simulação
de RTL.

## Hipótese

O zero-skip em Conv2D + ReLU economiza ciclos de MAC proporcionais à fração de
saídas nulas da ReLU; em tensores esparsos, a economia supera o custo da decisão
de pular.

## Método

1. Implementar o núcleo Conv2D + ReLU em Verilog em duas variantes: percorrida
   densa e percorrida com zero-skip.
2. Simular as duas variantes com Verilator, contando ciclos por convolução
   (conferência de formas de onda com Icarus Verilog e GTKWave; verilator e
   iverilog já estão instalados no ambiente de desenvolvimento).
3. No lado Rust, modelar a fração pulável a partir das pré-ativações usando a
   ReLU do motor (`tars::math::relu`) e a derivada
   (`tars::math::relu_deriv_from_z`): a inferência pula os MACs cuja saída já é
   nula; o treino pula os updates de gradiente nulo.
4. Registrar ciclos por convolução, fração de zeros e economia relativa.

## Como rodar

```bash
cargo run -p exp02_zero_skip_cycles
cargo test -p exp02_zero_skip_cycles
```

## Saída esperada

CSV consolidado em `../../data/` (conforme `../../data/README.md`): ciclos por
convolução com e sem zero-skip, fração de saídas nulas da ReLU e economia
relativa por tensor de entrada.

## Status

Scaffold. O RTL e a simulação Verilator ainda não existem; o modelo de fração
pulável sobre a ReLU do motor já está implementado e testado.
