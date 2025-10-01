# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto

Sistema de busca de alta performance desenvolvido em Rust para o catálogo de produtos da MegaStore. O sistema oferece busca rápida e relevante em grandes volumes de dados utilizando estruturas de dados eficientes e algoritmos de ranking avançados.

## Tecnologias Utilizadas

- **Linguagem**: Rust 2021 Edition
- **Bibliotecas Principais**:
  - `fxhash` - Tabelas hash de alta performance
  - `serde/serde_json` - Serialização de dados
  - `rayon` - Processamento paralelo
  - `regex` - Processamento de expressões regulares
  - `criterion` - Benchmarking

## Instruções de Execução

### Pré-requisitos
- Rust 1.70 ou superior
- Cargo

### Compilação e Execução
```bash
# Clonar o repositório
git clone https://github.com/Fabioteixeir/megastore-search-system.git
cd megastore-search-system

# Compilar em modo release (recomendado)
cargo build --release

# Executar o sistema
cargo run --release
