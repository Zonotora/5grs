use num::Complex;

#[derive(Debug, Default, Copy, Clone)]
pub struct Symbol {
    pub complex: Complex<f32>,
}

impl Symbol {
    pub fn new(complex: Complex<f32>) -> Self {
        Symbol { complex }
    }
}
