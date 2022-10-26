use crate::{ElementHasher, HashFn};

mod rpo;
pub use rpo::Rpo256 as Hasher;
pub use rpo::{INV_MDS, MDS};

// TYPE ALIASES
// ================================================================================================

pub type Digest = <Hasher as HashFn>::Digest;

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
pub fn merge(values: &[Digest; 2]) -> Digest {
    Hasher::merge(values)
}