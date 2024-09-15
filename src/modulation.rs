use crate::{BitStream, SymbolStream};
use num::Complex;

use crate::symbol::Symbol;

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
        bit_stream
            .iter()
            .map(|bits| {
                let re = if bits & 0b10 == 0 { 1.0 } else { -1.0 };
                let im = if bits & 0b01 == 0 { 1.0 } else { -1.0 };
                Symbol::new(Complex::new(re, im))
            })
            .collect()
    }
}

impl Demodulate for Qpsk {
    fn demodulate(symbols: &SymbolStream) -> BitStream {
        symbols
            .iter()
            .map(|symbol| {
                let mut bits = 0u8;
                bits |= if symbol.complex.re <= 0.0 { 0b10 } else { 0 };
                bits |= if symbol.complex.im <= 0.0 { 0b01 } else { 0 };
                bits
            })
            .collect()
    }
}

pub struct Modulator {
    scheme: ModulationScheme,
}

impl Modulator {
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

impl Default for Modulator {
    fn default() -> Self {
        Self {
            scheme: ModulationScheme::Qpsk,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_modulate_demodulate() {
        let modulator = Modulator::default();
        for bits in 0..0b100 {
            let symbols = modulator.modulate(&vec![bits]);
            let modulated_bits = modulator.demodulate(&symbols)[0];
            assert_eq!(bits, modulated_bits);
        }
    }
}
