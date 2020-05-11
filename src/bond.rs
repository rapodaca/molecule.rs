use crate::bond_order::BondOrder;
use crate::parity::Parity;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Bond {
    pub tid: usize,
    pub order: BondOrder,
    pub parity: Option<Parity>
}

impl Bond {
    pub fn multiplicity(&self) -> u8 {
        match self.order {
            BondOrder::Zero =>   0,
            BondOrder::Single => 1,
            BondOrder::Double => 2,
            BondOrder::Triple => 3
        }
    }
}