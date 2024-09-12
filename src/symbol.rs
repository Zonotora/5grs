use num::Complex;

#[derive(Debug)]
pub struct Symbol {
    complex: Complex<f32>,
}

impl Symbol {
    pub fn new(complex: Complex<f32>) -> Self {
        Symbol { complex }
    }
}
