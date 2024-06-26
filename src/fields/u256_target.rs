use std::marker::PhantomData;

use num_bigint::BigUint;
use plonky2::{
    field::extension::Extendable,
    hash::hash_types::RichField,
    iop::{target::Target, witness::WitnessWrite},
    plonk::circuit_builder::CircuitBuilder,
};
use plonky2_crypto::u32::gadgets::arithmetic_u32::U32Target;
use plonky2_ecdsa::gadgets::biguint::BigUintTarget;

use super::{fq_target::FqTarget, fr_target::FrTarget};

#[derive(Clone, Debug)]
pub struct U256Target<F: RichField + Extendable<D>, const D: usize> {
    pub limbs: [Target; 8],
    _marker: PhantomData<F>,
}
impl<F: RichField + Extendable<D>, const D: usize> U256Target<F, D> {
    pub fn new(limbs: [Target; 8]) -> Self {
        Self {
            limbs,
            _marker: PhantomData,
        }
    }
    pub fn constant(builder: &mut CircuitBuilder<F, D>, value: &BigUint) -> Self {
        let mut limbs = value.to_u32_digits();
        assert!(limbs.len() <= 8);
        limbs.extend(vec![0; 8 - limbs.len()]);
        let limbs: [Target; 8] = limbs
            .iter()
            .map(|x| builder.constant(F::from_canonical_u32(*x)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self::new(limbs)
    }

    pub fn empty(builder: &mut CircuitBuilder<F, D>) -> Self {
        let limbs = builder.add_virtual_target_arr();
        Self::new(limbs)
    }

    pub fn connect(builder: &mut CircuitBuilder<F, D>, lhs: &Self, rhs: &Self) {
        for i in 0..8 {
            builder.connect(lhs.limbs[i], rhs.limbs[i]);
        }
    }
    pub fn to_fq(&self, builder: &mut CircuitBuilder<F, D>) -> FqTarget<F, D> {
        FqTarget::from_limbs(builder, &self.limbs)
    }
    pub fn to_fr(&self, builder: &mut CircuitBuilder<F, D>) -> FrTarget<F, D> {
        FrTarget::from_limbs(builder, &self.limbs)
    }
    pub fn to_biguint_target(&self) -> BigUintTarget {
        let limbs = self.limbs.map(|x| U32Target(x)).to_vec();
        BigUintTarget { limbs }
    }

    pub fn to_vec(&self) -> Vec<Target> {
        self.limbs.to_vec()
    }

    pub fn from_vec(input: &[Target]) -> Self {
        assert!(input.len() == 8);
        let limbs: [Target; 8] = input.try_into().unwrap();
        Self::new(limbs)
    }

    pub fn set_witness<W: WitnessWrite<F>>(&self, pw: &mut W, value: &BigUint) {
        let mut limbs = value.to_u32_digits();
        limbs.extend(vec![0; 8 - limbs.len()]);
        for i in 0..8 {
            pw.set_target(self.limbs[i], F::from_canonical_u32(limbs[i]));
        }
    }
}
