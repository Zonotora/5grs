use rs5g::cell::Cell;
use rs5g::modulation::Modulator;
use rs5g::scrambler::Scrambler;

use std::collections::VecDeque;

fn main() {
    let cell = Cell::new();
    let scrambler = Scrambler::default();
    let modulator = Modulator::default();

    let bits = vec![0, 1, 1, 1];
    println!("{:#?}", bits);
    let bits = scrambler.scramble(&bits);
    println!("{:#?}", bits);
    let bits = scrambler.scramble(&bits);
    println!("{:#?}", bits);

    let mut packed_bits = vec![];
    let mut bits = VecDeque::from(bits);

    // Pack 2 bits in the same u8
    if bits.len() > 1 {
        while !bits.is_empty() {
            let first = bits.pop_front().unwrap();
            let second = if bits.is_empty() {
                0
            } else {
                bits.pop_front().unwrap()
            };
            let packed = first + (second << 1);
            packed_bits.push(packed);
        }
    } else {
        packed_bits.push(bits[0]);
    }

    let symbols = modulator.modulate(&packed_bits);
    println!("{:#?}", symbols);
    let bits = modulator.demodulate(&symbols);
    println!("{:#?}", bits);
}
