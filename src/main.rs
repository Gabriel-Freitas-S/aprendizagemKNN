// ==================== IMPORTAÇÃO DE BIBLIOTECAS ====================
use csv::ReaderBuilder;        // Biblioteca externa para manipulação de arquivos CSV
use serde::Deserialize;        // Biblioteca para converter (deserializar) dados de forma automática
use std::cmp::Ordering;        // Módulo padrão para definir como comparar elementos
use std::collections::BinaryHeap; // Estrutura de dados de fila de prioridade (heap)
use std::error::Error;         // Trait para tratamento padronizado de erros
use std::process::Command;     // Módulo para executar comandos do sistema operacional

// ==================== ESTRUTURA DE DADOS PRINCIPAIS ====================
// #[derive] são atributos em Rust que adicionam funcionalidades às estruturas
// Debug: permite imprimir a estrutura para debug
// Clone: permite criar cópias da estrutura
// Deserialize: permite converter dados externos (como CSV) para esta estrutura
#[derive(Debug, Clone, Deserialize)]
struct Ponto {
    caracteristicas: Vec<f64>, // Vec<f64> é um vetor dinâmico de números decimais
    rotulo: String,            // String é o tipo de texto em Rust
}

// impl em Rust define a implementação de métodos para uma estrutura
// Similar a métodos de classe em outras linguagens
impl Ponto {
    // fn define uma função em Rust
    // -> indica o tipo de retorno da função
    // Self refere-se ao tipo atual (Ponto)
    fn novo(caracteristicas: Vec<f64>, rotulo: String) -> Self {
        Self { caracteristicas, rotulo } // Sintaxe curta quando o nome do campo e da variável são iguais
    }
}

// ==================== FUNÇÃO DE DISTÂNCIA ====================
// fn define uma função "solta" (não associada a uma estrutura)
// &Ponto indica uma referência a um Ponto (sem transferir propriedade)
fn distancia_euclidiana(ponto1: &Ponto, ponto2: &Ponto) -> f64 {
    ponto1.caracteristicas.iter()     // iter() cria um iterador sobre as características
        .zip(ponto2.caracteristicas.iter()) // zip combina dois iteradores em pares
        .map(|(a, b)| (a - b).powi(2))     // map transforma cada par em sua diferença ao quadrado
        .sum::<f64>()                       // soma todos os valores (anotação de tipo explícita)
        .sqrt()                             // calcula a raiz quadrada
}

// ==================== ESTRUTURA AUXILIAR PARA VIZINHOS ====================
#[derive(Debug)]
struct Vizinho {
    distancia: f64,
    rotulo: String,
}

impl Vizinho {
    fn novo(distancia: f64, rotulo: String) -> Self {
        Self { distancia, rotulo }
    }
}

// ==================== IMPLEMENTAÇÃO DE ORDENAÇÃO ====================
// Em Rust, para usar uma estrutura em uma coleção ordenada (como BinaryHeap),
// precisamos implementar traits (interfaces) de comparação

// Ord é usado para definir uma ordenação total (todos elementos são comparáveis)
impl Ord for Vizinho {
    fn cmp(&self, outro: &Self) -> Ordering {
        // partial_cmp para f64 retorna Option<Ordering>, unwrap converte para Ordering
        // Invertemos a ordem para ter um heap de mínimo (menor distância = maior prioridade)
        outro.distancia.partial_cmp(&self.distancia).unwrap()
    }
}

// PartialOrd é necessário para tipos que podem ser parcialmente ordenados
impl PartialOrd for Vizinho {
    fn partial_cmp(&self, outro: &Self) -> Option<Ordering> {
        Some(self.cmp(outro))
    }
}

// PartialEq define quando dois elementos são iguais
impl PartialEq for Vizinho {
    fn eq(&self, outro: &Self) -> bool {
        self.distancia == outro.distancia
    }
}

// Eq é um trait marcador que indica que a igualdade é uma relação de equivalência
impl Eq for Vizinho {}

// ==================== ALGORITMO KNN ====================
// &[Ponto] é uma fatia (slice) de Pontos - uma visão de um array
// usize é o tipo usado para índices e tamanhos em Rust
fn knn(treinamento: &[Ponto], ponto_teste: &Ponto, k: usize) -> String {
    // BinaryHeap é uma fila de prioridade que mantém o menor elemento no topo
    let mut heap = BinaryHeap::new();

    // Calcular distâncias e adicionar ao heap
    for ponto_treinamento in treinamento {
        let distancia = distancia_euclidiana(ponto_teste, ponto_treinamento);
        heap.push(Vizinho::novo(distancia, ponto_treinamento.rotulo.clone()));
    }

    // Coletar os k vizinhos mais próximos
    let mut k_vizinhos_rotulos = Vec::new();
    for _ in 0..k {
        // if let é usado para desempacotar Option de forma segura
        if let Some(vizinho) = heap.pop() {
            k_vizinhos_rotulos.push(vizinho.rotulo);
        }
    }

    // Contar frequência dos rótulos usando HashMap
    let mut contador_rotulos = std::collections::HashMap::new();
    for rotulo in k_vizinhos_rotulos {
        // entry API fornece uma maneira elegante de inserir ou atualizar valores
        *contador_rotulos.entry(rotulo).or_insert(0) += 1;
    }

    // Encontrar o rótulo mais frequente
    contador_rotulos.into_iter()
        .max_by_key(|&(_, count)| count) // Encontra entrada com maior contagem
        .map(|(rotulo, _)| rotulo)       // Extrai apenas o rótulo
        .unwrap()                        // Converte Option para valor (assume que existe)
}

// ==================== FUNÇÕES DE ENTRADA/SAÍDA ====================
// Result é um tipo que representa sucesso (Ok) ou erro (Err)
// Box<dyn Error> é um tipo que pode conter qualquer erro
fn carregar_dados_do_csv(caminho_arquivo: &str) -> Result<Vec<Ponto>, Box<dyn Error>> {
    let mut leitor = ReaderBuilder::new().from_path(caminho_arquivo)?; // ? propaga erros
    let mut pontos = Vec::new();

    // deserialize converte cada linha do CSV para uma tupla
    for resultado in leitor.deserialize() {
        let registro: (f64, f64, String) = resultado?;
        pontos.push(Ponto::novo(vec![registro.0, registro.1], registro.2));
    }

    Ok(pontos) // Retorna sucesso com os pontos
}

// Função para limpar o terminal de forma cross-platform
fn limpar_terminal() {
    // cfg! é uma macro que verifica o sistema operacional em tempo de compilação
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

// Esta função calcula o valor de "k" para o algoritmo KNN
// com base no tamanho do conjunto de dados de treinamento.
// Ela usa a heurística de calcular a raiz quadrada do
// total de dados e arredondar o resultado para cima.
fn calcular_k(total_dados: usize) -> usize {
    // Converte o tamanho do conjunto de dados (usize) em um número de ponto flutuante (f64)
    // para poder calcular a raiz quadrada.
    (total_dados as f64)
        // Calcula a raiz quadrada do total de dados.
        .sqrt()
        // Arredonda o resultado para cima, para o próximo número inteiro.
        .ceil()
        // Converte o resultado (f64) de volta para usize, que é o tipo esperado pelo algoritmo KNN.
        as usize
}

// ==================== FUNÇÃO PRINCIPAL ====================
// main() é o ponto de entrada do programa
// -> Result<(), Box<dyn Error>> indica que a função pode retornar erro
fn main() -> Result<(), Box<dyn Error>> {
    limpar_terminal();

    // Carrega dados e trata possíveis erros com ?
    let dados_treinamento = carregar_dados_do_csv("src/dados.csv")?;

    let total_dados = dados_treinamento.len();

    let k = calcular_k(total_dados);

    // Cria um ponto de teste com duas características
    let ponto_teste = Ponto::novo(vec![4.5, 8.0], "Desconhecido".to_string());

    // Executa o algoritmo KNN
    let rotulo = knn(&dados_treinamento, &ponto_teste, k);

    // Exibe resultado
    println!(
        "Rótulo previsto para os dados de teste {:?} é {}",
        ponto_teste.caracteristicas,
        rotulo
    );

    Ok(()) // Retorna sucesso (unit type)
}