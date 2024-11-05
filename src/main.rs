use csv::ReaderBuilder; // Biblioteca para ler e processar arquivos CSV
use serde::Deserialize; // Biblioteca para serialização e desserialização de dados
use std::cmp::Ordering; // Usado para comparar valores
use std::collections::BinaryHeap; // Usado para armazenar os vizinhos mais próximos
use std::error::Error; // Usado para tratamento de erros
use std::process::Command; // Usado para executar comandos do sistema

// Estrutura para armazenar um ponto de dados
// Um ponto possui um vetor de características (números) e um rótulo (string)
#[derive(Debug, Clone, Deserialize)]
struct Ponto {
    caracteristicas: Vec<f64>, // Vetor de características do ponto
    rotulo: String, // Rótulo ou classe do ponto
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
    ponto1.caracteristicas.iter() // Iterar sobre as características do primeiro ponto
        .zip(ponto2.caracteristicas.iter()) // Combinar as características dos dois pontos
        .map(|(a, b)| (a - b).powi(2)) // Calcular a diferença ao quadrado para cada par de características
        .sum::<f64>() // Somar as diferenças ao quadrado
        .sqrt() // Calcular a raiz quadrada da soma
}

// Estrutura auxiliar para manter a distância e o rótulo em uma estrutura Heap
#[derive(Debug)]
struct Vizinho {
    distancia: f64, // Distância do vizinho ao ponto de teste
    rotulo: String, // Rótulo do vizinho
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
        // Comparar as distâncias dos vizinhos.
        // A ordem é invertida para que o BinaryHeap funcione como um heap de mínimo,
        // onde o elemento com a menor distância tem a maior prioridade.
        outro.distancia.partial_cmp(&self.distancia).unwrap()
    }
}

impl PartialOrd for Vizinho {
    fn partial_cmp(&self, outro: &Self) -> Option<Ordering> {
        // Usar a função cmp definida em Ord para fornecer a ordenação parcial.
        Some(self.cmp(outro))
    }
}

impl PartialEq for Vizinho {
    fn eq(&self, outro: &Self) -> bool {
        // Dois Vizinhos são considerados iguais se suas distâncias forem iguais.
        self.distancia == outro.distancia
    }
}

impl Eq for Vizinho {} // Trait marcador que indica que a relação de igualdade definida em PartialEq é reflexiva, simétrica e transitiva.

// Função K-Nearest Neighbors
// Dado um conjunto de treinamento, um ponto de teste e um valor k,
// encontra os k vizinhos mais próximos do ponto de teste e retorna o rótulo mais comum
fn knn(treinamento: &[Ponto], ponto_teste: &Ponto, k: usize) -> String {
    let mut heap = BinaryHeap::new(); // Criar um heap para armazenar os vizinhos mais próximos

    // Calcular a distância de cada ponto de treinamento até o ponto de teste
    for ponto_treinamento in treinamento {
        let distancia = distancia_euclidiana(ponto_teste, ponto_treinamento); // Calcular a distância
        heap.push(Vizinho::novo(distancia, ponto_treinamento.rotulo.clone())); // Adicionar o vizinho ao heap
    }

    // Coletar os rótulos dos k vizinhos mais próximos
    let mut k_vizinhos_rotulos = Vec::new(); // Criar um vetor para armazenar os rótulos dos k vizinhos mais próximos
    for _ in 0..k {
        if let Some(vizinho) = heap.pop() { // Remover o vizinho com a menor distância do heap
            k_vizinhos_rotulos.push(vizinho.rotulo); // Adicionar o rótulo do vizinho ao vetor
        }
    }

    // Contar a frequência de cada rótulo nos k vizinhos mais próximos
    let mut contador_rotulos = std::collections::HashMap::new(); // Criar um HashMap para contar a frequência dos rótulos
    for rotulo in k_vizinhos_rotulos {
        *contador_rotulos.entry(rotulo).or_insert(0) += 1; // Incrementar o contador do rótulo
    }

    // Retornar o rótulo mais comum entre os k vizinhos
    contador_rotulos.into_iter() // Iterar sobre os rótulos e suas contagens
        .max_by_key(|&(_, count)| count) // Encontrar o rótulo com a maior contagem
        .map(|(rotulo, _)| rotulo) // Extrair o rótulo
        .unwrap() // Desempacotar o rótulo (assumindo que há pelo menos um rótulo)
}

// Função para carregar dados de um arquivo CSV
// Cada linha do CSV é um ponto com características e um rótulo
fn carregar_dados_do_csv(caminho_arquivo: &str) -> Result<Vec<Ponto>, Box<dyn Error>> {
    let mut leitor = ReaderBuilder::new().from_path(caminho_arquivo)?; // Criar um leitor de CSV
    let mut pontos = Vec::new(); // Criar um vetor para armazenar os pontos

    // Ler cada registro do CSV e criar um Ponto a partir dele
    for resultado in leitor.deserialize() { // Desserializar cada registro do CSV
        let registro: (f64, f64, String) = resultado?; // Converter o registro em uma tupla (f64, f64, String)
        pontos.push(Ponto::novo(vec![registro.0, registro.1], registro.2)); // Criar um Ponto a partir do registro e adicioná-lo ao vetor
    }

    Ok(pontos) // Retornar o vetor de pontos
}

// Função para limpar o terminal
fn limpar_terminal() {
    if cfg!(target_os = "windows") { // Se o sistema operacional for Windows
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap(); // Executar o comando "cls"
    } else { // Se o sistema operacional for Linux ou macOS
        Command::new("clear").status().unwrap(); // Executar o comando "clear"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Limpar o terminal
    limpar_terminal();

    // Carregar dados do arquivo CSV
    let dados_treinamento = carregar_dados_do_csv("src/dados.csv")?; // Carregar os dados de treinamento do arquivo CSV

    // Definir um ponto de teste
    let ponto_teste = Ponto::novo(vec![4.5, 8.0], "Desconhecido".to_string()); // Criar um ponto de teste

    // Aplicar KNN com k=3
    let rotulo = knn(&dados_treinamento, &ponto_teste, 3); // Aplicar o algoritmo KNN com k=3

    // Exibir o rótulo previsto para o ponto de teste
    println!("Rótulo previsto para os dados de teste {:?} é {}", ponto_teste.caracteristicas, rotulo); // Imprimir o rótulo previsto

    Ok(()) // Retornar Ok(()) para indicar que o programa foi executado com sucesso
}