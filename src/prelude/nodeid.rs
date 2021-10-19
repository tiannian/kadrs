use core::ops::{BitXor, BitXorAssign};

use crate::Result;

pub trait NodeId: PartialEq + Eq + PartialOrd + Ord + BitXor + BitXorAssign + Sized {
    type PublicKey;

    fn verify(&self, pk: Self::PublicKey) -> Result<()>;
}
