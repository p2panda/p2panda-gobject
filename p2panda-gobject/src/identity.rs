use p2panda::node;

use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaPrivateKey", nullable)]
pub struct PrivateKey(pub(crate) node::PrivateKey);

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivateKey {
    pub fn new() -> Self {
        PrivateKey(node::PrivateKey::new())
    }

    pub fn from_data(bytes: [u8; 32]) -> Self {
        Self(p2panda::node::PrivateKey::from(bytes))
    }

    pub fn from_hex(_str: &str) -> Self {
        todo!("p2panda doesn't implement this");
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    pub fn to_data(&self) -> [u8; 32] {
        *self.0.as_bytes()
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaPublicKey", nullable)]
pub struct PublicKey(pub(crate) p2panda::node::PublicKey);

impl PublicKey {
    pub fn from_data(bytes: [u8; 32]) -> Result<Self, glib::Error> {
        Ok(Self(p2panda::node::PublicKey::try_from(bytes).map_err(
            |error| glib::Error::new(Error::Signature, &error.to_string()),
        )?))
    }

    pub fn from_hex(_str: &str) -> Self {
        todo!("p2panda doesn't implement this");
    }

    pub fn to_data(&self) -> [u8; 32] {
        *self.0.as_bytes()
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}
