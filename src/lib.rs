pub mod cell;
mod frame;
mod gnb;
pub mod modulation;
mod scheduler;
pub mod scrambler;
mod stub;
mod symbol;
mod ue;

use symbol::Symbol;

type BitStream = Vec<u8>;
type SymbolStream = Vec<Symbol>;
