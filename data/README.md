# data — CSVs consolidados

Diretório das tabelas de métricas produzidas pelos experimentos. No scaffold
nenhuma tabela existe ainda; tudo abaixo é a convenção para as primeiras
execuções.

## Convenções

1. **Uma tabela por execução.** Cada execução de experimento grava exatamente um
   CSV; reexecuções geram arquivos novos, nunca sobrescrevem os anteriores.
2. **Nome do arquivo:** `expNN_assunto_YYYYMMDD.csv` (exemplo:
   `exp01_bits_por_peso_20261025.csv`).
3. **Formato:** UTF-8, delimitador vírgula, cabeçalho na primeira linha, sem
   acentos nos nomes de coluna.
4. **Colunas fixas, nesta ordem:**

   ```csv
   metrica,valor,unidade,parametros,data
   ```

   - `metrica` — nome da métrica medida (ex.: `bits_por_peso`);
   - `valor` — valor numérico da medição;
   - `unidade` — unidade do valor (ex.: `bits`, `ciclos`, `fracao`);
   - `parametros` — parâmetros da execução (ex.: `n=784;densidade=0.10`);
   - `data` — data da execução, no formato `YYYY-MM-DD`.

5. **Citabilidade:** todo CSV desta pasta é citável nas tabelas de métricas
   oficiais dos papers e relatórios; nenhum número entra em publicação sem o CSV
   correspondente e o comando exato que o gerou.
