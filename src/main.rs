use rs5g::cell::Cell;
use rs5g::modulation::Modulator;

fn main() {
    let cell = Cell::new();
    let modulator = Modulator::default();

    for bits in 0..0b100 {
        println!("{:#?}", bits);
        let bits = vec![bits];
        let symbols = modulator.modulate(&bits);
        println!("{:#?}", symbols);
        let bits = modulator.demodulate(&symbols);
        println!("{:#?}", bits);
    }
}
