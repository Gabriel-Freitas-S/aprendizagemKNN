# Implementação do Algoritmo K-Nearest Neighbors (KNN) em Rust

## Índice

- [Introdução](#introdução)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Entendendo o Algoritmo KNN](#entendendo-o-algoritmo-knn)
- [Conjunto de Dados](#conjunto-de-dados)
- [Explicação Detalhada do Código](#explicação-detalhada-do-código)
    - [Bibliotecas Utilizadas](#bibliotecas-utilizadas)
    - [Estruturas de Dados Principais](#estruturas-de-dados-principais)
        - [Estrutura Ponto](#estrutura-ponto)
        - [Estrutura Vizinho](#estrutura-vizinho)
    - [Funções Principais](#funções-principais)
        - [Distância Euclidiana](#distância-euclidiana)
        - [Algoritmo KNN](#algoritmo-knn)
    - [Entrada e Saída](#entrada-e-saída)
        - [Leitura do CSV](#leitura-do-csv)
- [Como Executar](#como-executar)
- [Requisitos](#requisitos)
- [Exemplo de Uso](#exemplo-de-uso)

## Introdução

Este projeto implementa o algoritmo de aprendizado de máquina K-Nearest Neighbors (KNN) usando a linguagem Rust. O KNN é
um dos algoritmos mais simples e intuitivos de aprendizado de máquina, sendo excelente para iniciantes na área.

### O que é o KNN?

O KNN é um algoritmo que classifica um novo dado baseado nas características dos dados mais próximos a ele. Imagine que
você tem várias frutas em uma mesa e quer identificar uma fruta desconhecida. Você provavelmente olharia para as frutas
mais parecidas com ela para tentar adivinhar que fruta é. O KNN faz exatamente isso, mas usando medidas matemáticas!

## Estrutura do Projeto

O projeto está organizado em três arquivos principais:

- `main.rs`: Contém a implementação do algoritmo
- `dados.csv`: Arquivo com os dados de treinamento
- `README.md`: Este arquivo de documentação

## Entendendo o Algoritmo KNN

O KNN funciona em 4 passos principais:

1. **Receber um novo dado para classificar**
    - Ex: Um ponto com coordenadas [4.5, 8.0]

2. **Calcular a distância deste novo dado para todos os dados de treinamento**
    - Usamos a distância euclidiana (como se fosse medir com uma régua)
    - Quanto menor a distância, mais próximo o ponto está

3. **Selecionar os K vizinhos mais próximos**
    - Se K=3, pegamos os 3 pontos com menor distância
    - K é um número que escolhemos (neste projeto, usamos K=3)

4. **Classificar baseado na maioria**
    - Olhamos a classe mais comum entre os K vizinhos
    - Esta será a classe prevista para o novo dado

## Conjunto de Dados

O arquivo `dados.csv` contém nosso conjunto de dados com:

- Duas características (feature1 e feature2)
- Um rótulo (label) que pode ser: Classe A, B, C ou D
- Total de dezenas de exemplos para treinamento

Exemplo do formato:

```csv
feature1,feature2,label
1.0,2.0,Classe A
2.0,3.0,Classe A
3.0,3.0,Classe B
```

## Explicação Detalhada do Código

### Bibliotecas Utilizadas

```rust
use csv::ReaderBuilder;        // Para ler arquivos CSV
use serde::Deserialize;        // Para converter dados do CSV para nossas estruturas
use std::collections::BinaryHeap; // Para ordenar os vizinhos mais próximos
```

### Estruturas de Dados Principais

#### Estrutura Ponto

```rust
struct Ponto {
    caracteristicas: Vec<f64>, // Vetor com as características do ponto
    rotulo: String,           // Classe do ponto (A, B, C ou D)
}
```

- `Vec<f64>`: É como um array dinâmico de números decimais
- Cada ponto tem características (coordenadas) e um rótulo (classe)

#### Estrutura Vizinho

```rust
struct Vizinho {
    distancia: f64,  // Distância até o ponto de teste
    rotulo: String,  // Classe deste vizinho
}
```

### Funções Principais

#### Distância Euclidiana

```rust
fn distancia_euclidiana(ponto1: &Ponto, ponto2: &Ponto) -> f64 {
    // Calcula a distância entre dois pontos
    ponto1.caracteristicas.iter()
        .zip(ponto2.caracteristicas.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
```

- Calcula a distância "em linha reta" entre dois pontos
- Usa o teorema de Pitágoras generalizado

#### Algoritmo KNN

```rust
fn knn(treinamento: &[Ponto], ponto_teste: &Ponto, k: usize) -> String {
    let mut heap = BinaryHeap::new();
    // ... código de classificação
}
```

- Recebe os dados de treinamento, um ponto para classificar e o valor de K
- Retorna a classe prevista para o ponto de teste

### Entrada e Saída

#### Leitura do CSV

```rust
fn carregar_dados_do_csv(caminho_arquivo: &str) -> Result<Vec<Ponto>, Box<dyn Error>> {
    // Código para ler o arquivo CSV e converter em pontos
}
```

- Lê o arquivo CSV e converte cada linha em um `Ponto`
- Trata possíveis erros durante a leitura

## Como Executar

1. Instale o Rust (https://rustup.rs/)
2. Clone este repositório
3. No terminal, navegue até a pasta do projeto
4. Execute:

```bash
cargo run
```

## Requisitos

- Rust
- Cargo (gerenciador de pacotes do Rust)
- Bibliotecas:
    - csv
    - serde

## Exemplo de Uso

```rust
// Criar um ponto de teste
let ponto_teste = Ponto::novo(vec![4.5, 8.0], "Desconhecido".to_string());

// Classificar usando KNN
let rotulo = knn(&dados_treinamento, &ponto_teste, 3);

// Ver o resultado
println!("Classe prevista: {}", rotulo);
```

### Saída Esperada:
```
Rótulo previsto para os dados de teste [4.5, 8.0] é Classe A

Process finished with exit code 0
```







