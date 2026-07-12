# Turbo Ecossistema

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-stable_1.75.0%2B-orange.svg)](#rust-toolchain)

Turbo é uma plataforma de infraestrutura e processamento de dados de alto desempenho para **Rust**, com bindings oficiais de alta fidelidade para **Node.js** (TypeScript) e **Python**. 

Concebido como um ecossistema integrado de baixa latência, o Turbo segue a filosofia de *Zero Cost Abstractions*, uso eficiente de CPU/memória (cache locality, SIMD, pools) e segurança de concorrência.

---

## ⚡ Princípios Arquiteturais

* **Zero-Cost Abstractions**: Pague apenas pelo que usar. Abstrações limpas que compilam em código altamente otimizado.
* **Ergonomia e Consistência**: APIs limpas, previsíveis, intuitivas e totalmente consistentes entre as crates.
* **Error Handling**: APIs públicas nunca entram em pânico. Fortemente tipadas com tratamento de erro sem vazamento de abstração.
* **Memory Optimization**: Alocação consciente de memória, reutilização de buffers, slabs, pools e arenas para evitar pressões de GC (Bindings) ou alocador global (Rust).
* **Unsafe Rust**: Altamente documentado, testado e restrito apenas a ganhos de performance comprovados por benchmarks.

---

## 📦 Estrutura do Workspace

O projeto é organizado como um Cargo Workspace único, estruturado da seguinte forma:

```
turbo/
├── Cargo.toml                # Definição do Workspace
├── rust-toolchain.toml       # Versão pinhada do Rust (MSRV)
├── README.md                 # Visão geral
├── LICENSE                   # Licença MIT / Apache 2.0
├── CHANGELOG.md              # Registro de alterações
├── crates/                   # Pacotes Rust internos
│   ├── turbo-core            # Tipos compartilhados, erros, traits e alloc
│   ├── turbo-bytes           # ByteBuffer, reader/writer de alta performance
│   └── ... (outros componentes)
├── bindings/                 # Bindings oficiais para outras linguagens
│   ├── node/                 # napi-rs
│   └── python/               # PyO3
├── benchmarks/               # Suíte Criterion de performance global
└── examples/                 # Exemplos de uso executáveis
```

---

## 🚀 Ordem de Implementação das Crates

1. **`turbo-core`** - Tipos compartilhados, erros, traits e utilitários de alocação.
2. **`turbo-bytes`** - Buffer e fluxos binários.
3. **`turbo-string`** - Processamento de texto otimizado.
4. **`turbo-hash`** - Tabelas de hash personalizadas.
5. **`turbo-collections`** - Coleções especializadas de alta densidade.
6. **`turbo-pool`** - Alocadores de arena e pools de objetos.
7. **`turbo-worker`** - Thread pool para roubo de tarefas (work-stealing).
8. **`turbo-csv`** - Leitor/Gravador CSV de alta velocidade (SIMD).
9. **`turbo-json`** - Parser e serializador JSON DOM/Streaming.
10. **`turbo-query`** - Mecanismo de expressão e pipelines de consulta.
11. **`turbo-data`** - DataFrames e Series.
12. **`turbo-http`** - Parser HTTP de alto desempenho.
13. **`turbo-cache`** - Caches thread-safe concorrentes (LRU, LFU, ARC).
14. **`turbo-log`** - Registro estruturado assíncrono.
15. **`turbo-metrics`** - Contadores, gauges e histogramas.
16. **`turbo-cli`** - Parser de argumentos rápido.
17. **`turbo-config`** - Carregador e validador de configurações.

---

## 📜 Licença

Distribuído sob os termos da licença **MIT** e **Apache 2.0**. Consulte o arquivo [LICENSE](file:///home/lauros/Workspace/projects/turbo-core/LICENSE) para obter detalhes.
