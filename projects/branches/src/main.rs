fn main() {
    let numero = 6;

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    lacos();
    laco_for();
}

fn lacos() {
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }
}

fn laco_for() {
    let a = [11, 22, 33, 44, 55];

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }
}
