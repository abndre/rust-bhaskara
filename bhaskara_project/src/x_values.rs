// Função para calcular os valores de x
pub fn calcular_valores_de_x(a: f64, b: f64, delta: f64) -> Option<(f64, f64)> {
    if delta >= 0.0 {
        let x1 = (-b + delta.sqrt()) / (2.0 * a);
        let x2 = (-b - delta.sqrt()) / (2.0 * a);
        Some((x1, x2))
    } else {
        None // Delta negativo não tem solução real
    }
}
