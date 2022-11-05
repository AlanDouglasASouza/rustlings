
    // Testando variaveis e funções
fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
    //let espacos = "   ";
    //let espacos = espacos.len();
    outra_funcao();

    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {}", y);

    let a = cinco();

    println!("O valor de a é: {}", a);

    let som = soma_um(9);

    println!("O valor da soma é {}", som)
}

fn outra_funcao() {
    println!("Outra função.");
}

//CINCO
fn cinco() -> i32 {
    5
}

// Função que incrementa um número
fn soma_um(x: i32) -> i32 {
    x + 1
}