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
        //   (imag)
        //     ^
        //  10 | 00
        //  ---|---> (real)
        //  11 | 01
        let mut symbols: SymbolStream = vec![];
        let sqrt: f32 = 1.0 / (2.0_f32).sqrt();
        let re_map = [sqrt, -sqrt, -sqrt, sqrt];
        let im_map = [sqrt, sqrt, -sqrt, -sqrt];
        for bits in bit_stream {
            let re = re_map[*bits as usize];
            let im = im_map[*bits as usize];
            let complex = Complex::new(re, im);
            let symbol = Symbol::new(complex);
            symbols.push(symbol)
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
