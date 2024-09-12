use crate::gnb::Gnb;
use crate::ue::Ue;

pub struct Cell {
    gnb: Gnb,
    ues: Vec<Ue>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            gnb: Gnb::default(),
            ues: vec![],
        }
    }
}
