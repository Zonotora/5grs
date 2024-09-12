use crate::symbol::Symbol;

const N_OFDM_SYMBOLS_IN_SLOT: usize = 14;
const N_SLOTS_FOR_NUMEROLOGY_0: usize = 10;

type SymbolSet = [Symbol; N_OFDM_SYMBOLS_IN_SLOT];
struct Frame<const N: usize> {
    numerology: u8,
    slots: [SymbolSet; N],
}

impl Frame<N_SLOTS_FOR_NUMEROLOGY_0> {
    pub fn new() -> Self {
        Self {
            numerology: 0,
            slots: [[Symbol::default(); N_OFDM_SYMBOLS_IN_SLOT]; N_SLOTS_FOR_NUMEROLOGY_0]
        }
    }
}
