# Implementação do Algoritmo K-Nearest Neighbors (KNN) em Rust

Este programa em Rust implementa o algoritmo de aprendizado de máquina K-Nearest Neighbors (KNN). O KNN é um algoritmo simples e versátil usado para classificação e regressão. Neste caso, ele é usado para classificar pontos de dados em diferentes classes.

## Arquivo `main.rs`

Este arquivo contém o código Rust que implementa o algoritmo KNN e a lógica para carregar e processar dados de um arquivo CSV.

**Código:**

```rust
use csv::ReaderBuilder;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::process::Command;
use std::env;

// Estrutura para armazenar um ponto de dados
// Um ponto possui um vetor de características (números) e um rótulo (string)
#[derive(Debug, Clone, Deserialize)]
struct Ponto {
    caracteristicas: Vec<f64>,
    rotulo: String,
}

impl Ponto {
    // Função para criar uma nova instância de Ponto
    fn novo(caracteristicas: Vec<f64>, rotulo: String) -> Self {
        Self { caracteristicas, rotulo }
    }
}

// Função para calcular a distância euclidiana entre dois pontos
// A distância euclidiana é a raiz quadrada da soma das diferenças quadradas de cada coordenada
fn distancia_euclidiana(ponto1: &Ponto, ponto2: &Ponto) -> f64 {
    ponto1.caracteristicas.iter()
        .zip(ponto2.caracteristicas.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

// Estrutura auxiliar para manter a distância e o rótulo em uma estrutura Heap
#[derive(Debug)]
struct Vizinho {
    distancia: f64,
    rotulo: String,
}

impl Vizinho {
    // Função para criar uma nova instância de Vizinho
    fn novo(distancia: f64, rotulo: String) -> Self {
        Self { distancia, rotulo }
    }
}

// Implementação para comparação de distância (menor distância tem prioridade)
// Implementar o trait Ord para permitir a estrutura ser usada em um BinaryHeap
impl Ord for Vizinho {
    fn cmp(&self, outro: &Self) -> Ordering {
        outro.distancia.partial_cmp(&self.distancia).unwrap()
    }
}

impl PartialOrd for Vizinho {
    fn partial_cmp(&self, outro: &Self) -> Option<Ordering> {
        Some(self.cmp(outro))
    }
}

impl PartialEq for Vizinho {
    fn eq(&self, outro: &Self) -> bool {
        self.distancia == outro.distancia
    }
}

impl Eq for Vizinho {}

// Função K-Nearest Neighbors
// Dado um conjunto de treinamento, um ponto de teste e um valor k,
// encontra os k vizinhos mais próximos do ponto de teste e retorna o rótulo mais comum
fn knn(treinamento: &[Ponto], ponto_teste: &Ponto, k: usize) -> String {
    let mut heap = BinaryHeap::new();
    // Calcular a distância de cada ponto de treinamento até o ponto de teste
    for ponto_treinamento in treinamento {
        let distancia = distancia_euclidiana(ponto_teste, ponto_treinamento);
        heap.push(Vizinho::novo(distancia, ponto_treinamento.rotulo.clone()));
    }

    // Coletar os rótulos dos k vizinhos mais próximos
    let mut k_vizinhos_rotulos = Vec::new();
    for _ in 0..k {
        if let Some(vizinho) = heap.pop() {
            k_vizinhos_rotulos.push(vizinho.rotulo);
        }
    }

    // Contar a frequência de cada rótulo nos k vizinhos mais próximos
    let mut contador_rotulos = std::collections::HashMap::new();
    for rotulo in k_vizinhos_rotulos {
        *contador_rotulos.entry(rotulo).or_insert(0) += 1;
    }

    // Retornar o rótulo mais comum entre os k vizinhos
    contador_rotulos.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(rotulo, _)| rotulo)
        .unwrap()
}

// Função para carregar dados de um arquivo CSV
// Cada linha do CSV é um ponto com características e um rótulo
fn carregar_dados_do_csv(caminho_arquivo: &str) -> Result<Vec<Ponto>, Box<dyn Error>> {
    let mut leitor = ReaderBuilder::new().from_path(caminho_arquivo)?;
    let mut pontos = Vec::new();

    // Ler cada registro do CSV e criar um Ponto a partir dele
    for resultado in leitor.deserialize() {
        let registro: (f64, f64, String) = resultado?;
        pontos.push(Ponto::novo(vec![registro.0, registro.1], registro.2));
    }

    Ok(pontos)
}

// Função para limpar o terminal
fn limpar_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Limpar o terminal
    limpar_terminal();
    // Carregar dados do arquivo CSV
    let dados_treinamento = carregar_dados_do_csv("src/dados.csv")?;
    // Definir um ponto de teste
    let ponto_teste = Ponto::novo(vec![4.5, 8.0], "Desconhecido".to_string());
    // Aplicar KNN com k=3
    let rotulo = knn(&dados_treinamento, &ponto_teste, 3);
    // Exibir o rótulo previsto para o ponto de teste
    println!("Rótulo previsto para os dados de teste {:?} é {}", ponto_teste.caracteristicas, rotulo);
    Ok(())
}
```

**Explicação detalhada do código:**

1. **Importar bibliotecas:**
    - `csv`: Usada para ler e processar arquivos CSV.
    - `serde`: Usada para serialização e desserialização de dados.
    - `std::cmp::Ordering`: Usada para comparar valores.
    - `std::collections::BinaryHeap`: Usada para armazenar os vizinhos mais próximos.
    - `std::error::Error`: Usada para tratamento de erros.
    - `std::process::Command`: Usada para executar comandos do sistema.
    - `std::env`: Usada para interagir com o ambiente do sistema.

2. **Estruturas de dados:**
    - `Ponto`: Representa um ponto de dados com características (números) e um rótulo (classe).
    - `Vizinho`: Armazena a distância e o rótulo de um ponto de dados em relação a outro ponto.

3. **Funções:**
    - `distancia_euclidiana`: Calcula a distância euclidiana entre dois pontos.
    - `knn`: Implementa o algoritmo KNN.
        - Calcula a distância entre o ponto de teste e cada ponto de treinamento.
        - Armazena as distâncias e rótulos em um heap.
        - Seleciona os k vizinhos mais próximos.
        - Encontra o rótulo mais comum entre os k vizinhos.
    - `carregar_dados_do_csv`: Carrega dados de um arquivo CSV.
    - `limpar_terminal`: Limpa o terminal.

4. **Função principal (`main`)**:
    - Limpa o terminal.
    - Carrega os dados de treinamento do arquivo CSV.
    - Define um ponto de teste.
    - Aplica o algoritmo KNN ao ponto de teste.
    - Exibe o rótulo previsto para o ponto de teste.

## Exemplo de Uso

O arquivo `dados.csv` contém dados de treinamento com duas características (feature1 e feature2) e um rótulo (label). O programa define um ponto de teste com características `[4.5, 8.0]` e usa o algoritmo KNN com `k=3` para prever o rótulo do ponto de teste.

**Saída:**

```
Rótulo previsto para os dados de teste [4.5, 8.0] é ClasseB
```

## Como Executar

1. Certifique-se de ter o Rust instalado em seu sistema.
2. Clone este repositório.
3. Navegue até o diretório do projeto.
4. Execute o comando `


#### Trabalho faculdade | Aprendizagem KNN