use crate::{Packet, Result};

use super::NodeId;

pub trait PublicKey {
    type Signature;

    fn verify(&self, sign: Self::Signature) -> Result<()>;
}

pub type SignatureByPublicKey<P> = <P as PublicKey>::Signature;

pub trait SecretKey<N: NodeId, I> {
    type PublicKey: PublicKey;

    fn public(&self) -> Self::PublicKey;

    fn sign(&self, packet: Packet<N, I>) -> Result<SignatureByPublicKey<Self::PublicKey>>;
}

