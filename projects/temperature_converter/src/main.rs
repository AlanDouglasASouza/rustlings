fn main() {    
    println!("Olá 84 °F convertido é {0:.1} °C!", converter(83.0));
}

fn converter(temp: f64) -> f64 {
    let celsius = (temp - 32.0) / 1.8;
    return celsius;
}