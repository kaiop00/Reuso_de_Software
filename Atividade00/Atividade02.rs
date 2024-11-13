use std::io;

// Função para ler um número inteiro do usuário
fn ler_inteiro() -> i32 {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");
    entrada.trim().parse::<i32>().expect("Digite um número inteiro válido")
}

// Função que analisa a tupla e retorna uma tupla com a soma, o maior e o menor valor
fn analisar_tupla(valores: (i32, i32, i32)) -> (i32, i32, i32) {
    let soma = valores.0 + valores.1 + valores.2;
    let maior = valores.0.max(valores.1).max(valores.2);
    let menor = valores.0.min(valores.1).min(valores.2);
    (soma, maior, menor)
}

fn main() {
    println!("Digite o primeiro número inteiro:");
    let num1 = ler_inteiro();
    println!("Digite o segundo número inteiro:");
    let num2 = ler_inteiro();
    println!("Digite o terceiro número inteiro:");
    let num3 = ler_inteiro();

    let tupla = (num1, num2, num3);
    let resultado = analisar_tupla(tupla);

    println!("Resultados:");
    println!("Soma dos números: {}", resultado.0);
    println!("Maior número: {}", resultado.1);
    println!("Menor número: {}", resultado.2);
}
