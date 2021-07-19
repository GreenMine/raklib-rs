#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Reliability {
    Unreliable,
    UnreliableSequenced,
    Reliable,
    ReliableOrdered,
    ReliableSequenced,
    UnreliableWithAckReceipt,
    ReliableWithAckReceipt,
    ReliableOrderedWithAckReceipt,
}

use Reliability::*;

impl Reliability {
    pub fn from_u8(reliability: u8) -> Self {
        unsafe { *(&reliability as *const _ as *const Self) }
    }
    pub fn is_reliable(self) -> bool {
        self == Reliable
            || self == ReliableOrdered
            || self == ReliableSequenced
            || self == ReliableWithAckReceipt
            || self == ReliableOrderedWithAckReceipt
    }

    pub fn is_sequenced(self) -> bool {
        self == UnreliableSequenced || self == ReliableSequenced
    }

    pub fn is_ordered(self) -> bool {
        self == ReliableOrdered || self == ReliableOrderedWithAckReceipt
    }
}
