//! Compact implementation for [`AlloyTxExtended`]

use crate::{Compact, FixedBytes};
use alloy_consensus::{TxExtended as AlloyTxExtended, transaction::EXTENDED_COMMITMENT_BYTES};
use alloy_primitives::{Bytes, ChainId, TxKind, U256};

/// Extended transaction.
#[derive(Debug, Clone, PartialEq, Eq, Default, Compact)]
#[reth_codecs(crate = "crate")]
#[cfg_attr(
    any(test, feature = "test-utils"),
    derive(arbitrary::Arbitrary, serde::Serialize, serde::Deserialize),
    crate::add_arbitrary_tests(crate, compact)
)]
#[cfg_attr(feature = "test-utils", allow(unreachable_pub), visibility::make(pub))]
pub(crate) struct TxExtended {
    /// Added as EIP-155: Simple replay attack protection
    chain_id: Option<ChainId>,
    /// A scalar value equal to the number of transactions sent by the sender; formally Tn.
    nonce: u64,
    /// A scalar value equal to the number of
    /// Wei to be paid per unit of gas for all computation
    /// costs incurred as a result of the execution of this transaction; formally Tp.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max number is:
    /// 340282366920938463463374607431768211455
    gas_price: u128,
    /// A scalar value equal to the number of Wei to
    /// be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment
    /// to the newly created account; formally Tv.
    value: U256,
    // NP TODO doc
    /// NP TODO doc
    // NP TODO name? Coin? NoteCommitment?
    pub commitment: FixedBytes<EXTENDED_COMMITMENT_BYTES>,
}

impl Compact for AlloyTxExtended {
    fn to_compact<B>(&self, buf: &mut B) -> usize
    where
        B: bytes::BufMut + AsMut<[u8]>,
    {
        let tx = TxExtended {
            chain_id: self.chain_id,
            nonce: self.nonce,
            gas_price: self.gas_price,
            value: self.value,
            commitment: self.commitment.clone(),
        };

        tx.to_compact(buf)
    }

    fn from_compact(buf: &[u8], len: usize) -> (Self, &[u8]) {
        let (tx, _) = TxExtended::from_compact(buf, len);

        let alloy_tx = Self {
            chain_id: tx.chain_id,
            nonce: tx.nonce,
            gas_price: tx.gas_price,
            value: tx.value,
            commitment: tx.commitment
        };

        (alloy_tx, buf)
    }
}
