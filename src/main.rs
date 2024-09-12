use rs5g::cell::Cell;
use rs5g::modulation::Modulator;

fn main() {
    let cell = Cell::new();
    let modulator = Modulator::new();

    let bits = vec![0x03];
    let symbols = modulator.modulate(&bits);
    println!("{:#?}", symbols);
}
