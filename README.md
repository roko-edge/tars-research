# tars-research

Laboratório acadêmico do motor TARS ([roko-edge/tars](https://github.com/roko-edge/tars)):
experimentos reprodutíveis em Rust e papers da Universidade Federal do Cariri (UFCA).

## Missão

Este repositório é o braço experimental da pesquisa: cada hipótese sobre
quantização, codificação de pesos e arquiteturas eficientes vira um experimento
executável que consome o motor real, grava métricas consolidadas em CSV sob
`data/` e alimenta os artigos e relatórios de Iniciação Científica publicados
pela UFCA. Os resultados, por sua vez, retornam ao motor público roko-edge/tars
como evidência para decisões de design.

## Estrutura

```text
tars-research/
├── Cargo.toml            raiz do workspace (members = experiments/*)
├── Cargo.lock            commitado: workspace de binários é reproduzível
├── LICENSE               MIT (código)
├── LICENSE-TEXT.md       CC BY 4.0 (papers e textos)
├── data/                 CSVs consolidados (uma tabela por execução)
├── papers/               artigos LaTeX, relatórios de IC (UFCA) e slides
└── experiments/
    ├── exp01_subbit_entropy/        custo < 1 bit/trit no empacotamento [skip_zeros | sign_bit]
    ├── exp02_zero_skip_cycles/      ciclos economizados pelo zero-skip em Conv2D + ReLU
    └── exp03_iso_memory_bnn_vs_tnn/ comparativo iso-memória: BNN densa x TNN esparsa
```

## Como rodar

Pré-requisito: clone irmão do motor em `../tars` (ver seção abaixo).

> **Estado do build (2026-10-25):** os experimentos compilam contra o `main`
> do motor irmão. O motor está em desenvolvimento ativo e, nesta data, o
> `main` dele está no meio de um refactor interno e não compila — portanto o
> `cargo build` deste workspace falha até o motor voltar a compilar. A
> lógica de empacotamento, entropia e iso-memória é autocontida em cada
> `experiments/exp0N/src/main.rs` e pode ser conferida independentemente.

```bash
cargo build                          # compila o workspace
cargo test                           # testes de sanidade de todos os experimentos
cargo run -p exp01_subbit_entropy    # banner + hipótese + demonstração
cargo run -p exp02_zero_skip_cycles
cargo run -p exp03_iso_memory_bnn_vs_tnn
```

## Dependência do motor

Os experimentos dependem do motor real via path, declarada uma única vez na raiz
e herdada por cada crate de experimento:

```toml
# Cargo.toml (raiz)
[workspace.dependencies]
tars = { path = "../tars" }
```

Nota de reprodutibilidade externa: a path dependency assume o clone irmão do
motor em `../tars`. Quando o motor publicar uma tag git estável, os experimentos
passarão a referenciar a tag no lugar do path (por exemplo
`tars = { git = "...", tag = "vX.Y.Z" }`), garantindo que qualquer executor
externo reproduza os números sem depender do layout local de diretórios.

## Licenças

- Código (crates em `experiments/`): MIT — ver [LICENSE](LICENSE).
- Textos (papers, relatórios, slides e Markdown): CC BY 4.0 — ver
  [LICENSE-TEXT.md](LICENSE-TEXT.md).

## Documentação do motor

A documentação de referência do motor — contratos, research logs e decisões de
arquitetura — vive em `../tars/docs/`, em especial os research logs de
quantização e codificação ternária em `../tars/docs/project/research/`.
