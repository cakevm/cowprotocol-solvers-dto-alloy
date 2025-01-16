use alloy_primitives::{Address, FixedBytes, B256};
use serde::{Deserialize, Serialize};

// uid as 56 bytes: 32 for orderDigest, 20 for ownerAddress and 4 for validTo
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrderUid(pub FixedBytes<56>);

impl OrderUid {
    /// Create a UID from its parts.
    pub fn from_parts(hash: B256, owner: Address, valid_to: u32) -> Self {
        let mut uid: FixedBytes<56> = Default::default();
        uid[0..32].copy_from_slice(hash.as_slice());
        uid[32..52].copy_from_slice(owner.as_slice());
        uid[52..56].copy_from_slice(&valid_to.to_be_bytes());
        Self(uid)
    }

    /// Splits an order UID into its parts.
    pub fn parts(&self) -> (B256, Address, u32) {
        (B256::from_slice(&self.0[0..32]), Address::from_slice(&self.0[32..52]), u32::from_be_bytes(self.0[52..56].try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_uid_parts() {
        let uid = OrderUid::from_parts(B256::repeat_byte(0x01), Address::repeat_byte(0x02), 123);
        assert_eq!(uid.parts(), (B256::repeat_byte(0x01), Address::repeat_byte(0x02), 123));
    }
}
