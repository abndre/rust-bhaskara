// Função para calcular o delta
pub fn calcular_delta(a: f64, b: f64, c: f64) -> f64 {
    b * b - 4.0 * a * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_delta() {
        assert_eq!(calcular_delta(1.0, -3.0, 2.0), 1.0);
        assert_eq!(calcular_delta(1.0, 2.0, 1.0), 0.0);
        assert_eq!(calcular_delta(2.0, -7.0, 6.0), 1.0);
    }
}