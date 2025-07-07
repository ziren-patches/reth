//! Compact implementation for [`AlloyTxGoat`]

use crate::Compact;
use alloy_consensus::TxGoat as AlloyTxGoat;
use alloy_primitives::Bytes;
///
/// This is a helper type to use derive on it instead of manually managing `bitfield`.
///
/// By deriving `Compact` here, any future changes or enhancements to the `Compact` derive
/// will automatically apply to this type.
///
/// Notice: Make sure this struct is 1:1 with [`alloy_consensus::TxGoat`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
// #[reth_codecs(crate = "crate")]
#[cfg_attr(
    any(test, feature = "test-utils"),
    derive(arbitrary::Arbitrary, serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(test, feature = "test-utils"),)]
#[cfg_attr(feature = "test-utils", allow(unreachable_pub), visibility::make(pub))]
pub(crate) struct TxGoat {
    module: u8,
    action: u8,
    nonce: u64,
    input: Bytes,
}

impl Compact for AlloyTxGoat {
    fn to_compact<B>(&self, buf: &mut B) -> usize
    where
        B: bytes::BufMut + AsMut<[u8]>,
    {
        todo!()
    }

    fn from_compact(_buf: &[u8], _len: usize) -> (Self, &[u8]) {
        todo!()
    }
}
