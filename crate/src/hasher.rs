// Pedersen hash implementation of the Hasher trait

use bellman::pairing::ff::{Field, PrimeField, PrimeFieldRepr};
use sapling_crypto::pedersen_hash::{baby_pedersen_hash, Personalization};
use bellman::pairing::bn256::Bn256;
use sapling_crypto::alt_babyjubjub::{JubjubEngine, AltJubjubBn256};
use crate::bit_iterator::BitIteratorLe;

pub struct PedersenHasher<E: JubjubEngine> {
    params: E::Params,
}

impl<E: JubjubEngine> PedersenHasher<E> {
    pub fn hash_bits<I: IntoIterator<Item=bool>>(&self, input: I) -> E::Fr {
        baby_pedersen_hash::<E, _>(Personalization::NoteCommitment, input, &self.params).into_xy().0
    }

    pub fn hash(&self, data: E::Fr) -> E::Fr {
        self.hash_bits(self.get_bits_le_fixed(data, E::Fr::NUM_BITS as usize))
    }

    fn get_bits_le_fixed(&self, data: E::Fr, n: usize) -> Vec<bool> {
        let mut r: Vec<bool> = Vec::with_capacity(n);
        r.extend(BitIteratorLe::new(data.into_repr()).take(n));
        let len = r.len();
        r.extend((len..n).map(|_| false));
        r
    }
}

pub type BabyPedersenHasher = PedersenHasher<Bn256>;

impl Default for PedersenHasher<Bn256> {
    fn default() -> Self {
        Self{
            params: AltJubjubBn256::new(),
        }
    }
}