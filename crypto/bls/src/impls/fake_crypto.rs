use crate::{
    aggregate_public_key::TAggregatePublicKey,
    aggregate_signature::TAggregateSignature,
    public_key::{GenericPublicKey, TPublicKey, PUBLIC_KEY_BYTES_LEN},
    secret_key::{TSecretKey, SECRET_KEY_BYTES_LEN},
    signature::{TSignature, SIGNATURE_BYTES_LEN},
    Error, Hash256, SecretHash,
};
/// Provides the externally-facing, core BLS types.
pub mod types {
    pub use super::verify_signature_sets;
    pub use super::AggregatePublicKey;
    pub use super::AggregateSignature;
    pub use super::PublicKey;
    pub use super::SecretKey;
    pub use super::Signature;
    pub use super::SignatureSet;
}

pub type SignatureSet<'a> = crate::signature_set::SignatureSet<
    'a,
    PublicKey,
    AggregatePublicKey,
    Signature,
    AggregateSignature,
>;

pub fn verify_signature_sets<'a>(
    _signature_sets: impl ExactSizeIterator<Item = &'a SignatureSet<'a>>,
) -> bool {
    true
}

#[derive(Clone)]
pub struct PublicKey([u8; PUBLIC_KEY_BYTES_LEN]);

impl PublicKey {
    fn zero() -> Self {
        Self([0; PUBLIC_KEY_BYTES_LEN])
    }
}

impl TPublicKey for PublicKey {
    fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN] {
        self.0.clone()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        let mut pubkey = Self::zero();
        pubkey.0[..].copy_from_slice(&bytes[0..PUBLIC_KEY_BYTES_LEN]);
        Ok(pubkey)
    }
}

impl Eq for PublicKey {}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

#[derive(Clone)]
pub struct AggregatePublicKey([u8; PUBLIC_KEY_BYTES_LEN]);

impl TAggregatePublicKey for AggregatePublicKey {
    fn zero() -> Self {
        Self([0; PUBLIC_KEY_BYTES_LEN])
    }

    fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN] {
        let mut bytes = [0; PUBLIC_KEY_BYTES_LEN];
        bytes[..].copy_from_slice(&self.0);
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        let mut key = [0; PUBLIC_KEY_BYTES_LEN];

        key[..].copy_from_slice(&bytes);

        Ok(Self(key))
    }
}

impl Eq for AggregatePublicKey {}

impl PartialEq for AggregatePublicKey {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

#[derive(Clone)]
pub struct Signature([u8; SIGNATURE_BYTES_LEN]);

impl Signature {
    fn zero() -> Self {
        Self([0; SIGNATURE_BYTES_LEN])
    }
}

impl TSignature<PublicKey> for Signature {
    fn serialize(&self) -> [u8; SIGNATURE_BYTES_LEN] {
        self.0.clone()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        let mut signature = Self::zero();
        signature.0[..].copy_from_slice(&bytes[0..SIGNATURE_BYTES_LEN]);
        Ok(signature)
    }

    fn verify(&self, _pubkey: &PublicKey, _msg: Hash256) -> bool {
        true
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

#[derive(Clone)]
pub struct AggregateSignature([u8; SIGNATURE_BYTES_LEN]);

impl AggregateSignature {
    fn zero() -> Self {
        Self([0; SIGNATURE_BYTES_LEN])
    }
}

impl TAggregateSignature<PublicKey, AggregatePublicKey, Signature> for AggregateSignature {
    fn zero() -> Self {
        Self::zero()
    }

    fn add_assign(&mut self, _other: &Signature) {
        // Do nothing.
    }

    fn add_assign_aggregate(&mut self, _other: &Self) {
        // Do nothing.
    }

    fn serialize(&self) -> [u8; SIGNATURE_BYTES_LEN] {
        let mut bytes = [0; SIGNATURE_BYTES_LEN];

        bytes[..].copy_from_slice(&self.0);

        bytes
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        let mut key = [0; SIGNATURE_BYTES_LEN];

        key[..].copy_from_slice(&bytes);

        Ok(Self(key))
    }

    fn fast_aggregate_verify(
        &self,
        _msg: Hash256,
        _pubkeys: &[&GenericPublicKey<PublicKey>],
    ) -> bool {
        true
    }

    fn aggregate_verify(
        &self,
        _msgs: &[Hash256],
        _pubkeys: &[&GenericPublicKey<PublicKey>],
    ) -> bool {
        true
    }
}

impl Eq for AggregateSignature {}

impl PartialEq for AggregateSignature {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

#[derive(Clone)]
pub struct SecretKey([u8; SECRET_KEY_BYTES_LEN]);

impl TSecretKey<Signature, PublicKey> for SecretKey {
    fn random() -> Self {
        Self([0; SECRET_KEY_BYTES_LEN])
    }

    fn public_key(&self) -> PublicKey {
        PublicKey::zero()
    }

    fn sign(&self, _msg: Hash256) -> Signature {
        Signature::zero()
    }

    fn serialize(&self) -> SecretHash {
        let mut bytes = [0; SECRET_KEY_BYTES_LEN];
        bytes[..].copy_from_slice(&self.0[..]);
        bytes.into()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        let mut sk = Self::random();
        sk.0[..].copy_from_slice(&bytes[0..SECRET_KEY_BYTES_LEN]);
        Ok(sk)
    }
}
