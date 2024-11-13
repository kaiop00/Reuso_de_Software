//POSITIVOS
//  calcula a média dos números positivos em um array de 10 elementos
fn media_positivos(arr: [i32; 10]) -> Option<f64> {
    // Filtra os valores positivos do array e armazena-os em um vetor
    let positivos: Vec<i32> = arr.iter().cloned().filter(|&x| x > 0).collect();

    // Verifica se o vetor de números positivos está vazio
    if positivos.is_empty() {
        None // Retorna None se não houver números positivos
    } else {
                // Calcula a soma dos números positivos
        let soma: i32 = positivos.iter().sum();
                // Calcula a média dos números positivos como um valor de ponto flutuante
        let media = soma as f64 / positivos.len() as f64;
        Some(media) 
    }
}
//PARES 

// Função que calcula o produto de todos os números pares em um array de 10 elementos
fn produto_pares(arr: [i32; 10]) -> i32 {
    // Filtra os valores pares do array e armazena-os em um vetor
    let pares: Vec<i32> = arr.iter().cloned().filter(|&x| x % 2 == 0).collect();

    // Verifica se o vetor de números pares está vazio
    if pares.is_empty() {
        1 // Retorna 1 se não houver números pares
    } else {
        // Calcula o produto de todos os números pares no vetor
        pares.iter().product()
    }
}

fn main() {
    // Numeros fixos do vetor ('\../')
    let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];

    // Chama a função media_positivos e trata o resultado com pattern matching
    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {}", media), // Exibe a média se existir
        None => println!("Não há números positivos."), // Mensagem caso não haja números positivos
    }

    // Chama a função produto_pares e exibe o resultado
    let produto = produto_pares(numeros);
    println!("Produto dos números pares: {}", produto);
}
