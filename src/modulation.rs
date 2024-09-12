use num::Complex;

use crate::symbol::Symbol;

type BitStream = Vec<u8>;
type SymbolStream = Vec<Symbol>;

trait Modulate {
    fn modulate(bit_stream: &BitStream) -> SymbolStream;
}

trait Demodulate {
    fn demodulate(symbol_stream: &SymbolStream) -> BitStream;
}

enum ModulationScheme {
    Qpsk,   // 2 bits per symbol
    Qam16,  // 4 bits per symbol
    Qam64,  // 6 bits per symbol
    Qam256, // 8 bits per symbol
}

struct Qpsk;

impl Modulate for Qpsk {
    fn modulate(bit_stream: &BitStream) -> SymbolStream {
        // d(i)=1√2[(1−2b(2i))+j(1−2b(2i+1))]
        let mut symbols: SymbolStream = vec![];
        let masks = [0xc0, 0x30, 0x0c, 0x03];
        let sqrt: f32 = 1.0 / (2.0_f32).sqrt();
        for bits in bit_stream {
            for (i, mask) in masks.iter().enumerate() {
                let value = (bits & mask) >> ((3 - i) * 2);
                let first = value & 0x1;
                let second = (value >> 1) & 0x1;
                let re = sqrt * (1 - 2 * first as i8) as f32;
                let im = sqrt * (1 - 2 * second as i8) as f32;
                let complex = Complex::new(re, im);
                let symbol = Symbol::new(complex);
                symbols.push(symbol)
            }
        }
        symbols
    }
}

impl Demodulate for Qpsk {
    fn demodulate(symbols: &SymbolStream) -> BitStream {
        let a = 0.70710677;
        let b = [(a, a), (a, -a), (-a, a), (-a, -a)];
        let c = [0x0, 0x1, 0x2, 0x3];

        vec![]
    }
}

pub struct Modulator {
    scheme: ModulationScheme,
}

impl Modulator {
    pub fn new() -> Self {
        Modulator {
            scheme: ModulationScheme::Qpsk,
        }
    }

    pub fn modulate(&self, bits: &BitStream) -> SymbolStream {
        match self.scheme {
            ModulationScheme::Qpsk => Qpsk::modulate(bits),
            _ => unimplemented!(),
        }
    }

    pub fn demodulate(&self, symbols: &SymbolStream) -> BitStream {
        match self.scheme {
            ModulationScheme::Qpsk => Qpsk::demodulate(symbols),
            _ => unimplemented!(),
        }
    }
}
