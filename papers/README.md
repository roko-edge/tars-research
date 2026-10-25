# papers — artigos, relatórios e slides

Onde vivem os textos acadêmicos do laboratório, todos sob CC BY 4.0 (ver
[LICENSE-TEXT.md](../LICENSE-TEXT.md)).

## Organização planejada

```text
papers/
├── articles/   artigos LaTeX submetidos a conferências e periódicos
├── ic/         relatórios de Iniciação Científica (UFCA)
└── slides/     apresentações de defesa, seminários e divulgação
```

## Regras

1. Cada artigo ou relatório cita apenas números que existam como CSV em
   [`data/`](../data/), seguindo as convenções de
   [`data/README.md`](../data/README.md).
2. Fontes em LaTeX; a compilação é responsabilidade de cada autor — este
   repositório não mantém pipeline de build de textos.
3. Relatórios de IC seguem o calendário e o modelo da UFCA; cada edição anual
   ganha um subdiretório próprio (ex.: `ic/2026/`).
