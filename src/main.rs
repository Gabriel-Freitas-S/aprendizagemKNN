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