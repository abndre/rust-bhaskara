mod delta;
mod x_values;



fn main() {
    println!("Hello, world - Lets Math!");
    println!("Calculadora de Bhaskara");

    // coeficientes
    let a: f64 = 1.0;
    let b: f64 = -5.0;
    let c: f64 = 6.0;

    // Calcule o delta usando a função do módulo 'delta'
    let delta: f64 = delta::calcular_delta(a, b, c);

    // Calcule os valores de x usando a função do módulo 'valores_de_x'
    match x_values::calcular_valores_de_x(a, b, delta) {
        Some((x1, x2)) => println!("As raízes são: x1 = {}, x2 = {}", x1, x2),
        None => println!("Delta negativo. Não há solução real."),
    }
}
