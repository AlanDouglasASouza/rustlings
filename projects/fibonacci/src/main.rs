fn main() {
    println!("O vigésimo termo da sequência de Fibonacci é {}.", fibonacci(20));
}

fn fibonacci(sequence: u32) -> u32 {

    let mut terms = [0, 1];
    let mut i = 1;   

    if sequence == 1 {
        return 0;
    }

    if sequence == 2 {
        return 1;
    }

    while i < sequence {      

        if i > 2 {           

            if i % 2 == 0 {
                terms[1] = terms[0] + terms[1];
            } else {
                terms[0] = terms[0] + terms[1];
            }
        }

        i = i + 1;
       
    }

    return terms[0] + terms[1];
}