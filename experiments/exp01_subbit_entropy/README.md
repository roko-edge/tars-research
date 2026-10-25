# exp01 — subbit_entropy

Prova prática do empacotamento sub-bit `[skip_zeros | sign_bit]`: medir se o
custo médio de armazenar uma camada ternária esparsa fica abaixo de 1 bit por
peso (trit), e comparar com TerEffic e BITCOS (referências a definir).

## Hipótese

Para streams ternários {-1, 0, +1} com alta densidade de zeros (regime típico de
75-90% após ternarização por limiar), o esquema de tokens de 5 bits do motor —
`t[4:1] = skip_zeros`, `t[0] = sign_bit`, com salto longo e fim de linha
in-band — custa em média menos de 1 bit/trit, superando o armazenamento denso
de 2 bits/trit.

## Método

1. Gerar linhas de pesos reais com o inicializador Xavier do motor
   (`tars::math::xavier`) e ternarizar por limiar.
2. Codificar cada linha no esquema de 5 bits: tokens 0-29 = peso ativo
   (skip <= 14 | sinal), 30 = salto longo de 15 posições, 31 = fim de linha.
3. Medir bits/peso da stream codificada em função da densidade de zeros.
4. Comparar contra o denso (2 bits/trit), o piso entrópico da linha e os
   trabalhos TerEffic e BITCOS (referências a definir).

## Como rodar

```bash
cargo run -p exp01_subbit_entropy
cargo test -p exp01_subbit_entropy
```

## Saída esperada

CSV consolidado em `../../data/` (conforme `../../data/README.md`): custo em
bits/peso por linha e por densidade, frente ao armazenamento denso e ao piso
entrópico.

## Status

Scaffold. O codificador/decodificador de tokens já está implementado e testado
(a ida e volta preserva posições e sinais de linhas geradas pelo motor); os
benchmarks com números e as comparações com TerEffic e BITCOS ainda não foram
executados.
