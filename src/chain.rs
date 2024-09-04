//! Chain specific utils

use std::fmt::Debug;

use alloy_primitives::{B256, U256};
use alloy_rpc_types::{BlockTransactions, Header};
use revm::{
    primitives::{BlockEnv, SpecId, TxEnv},
    Handler,
};

use crate::{mv_memory::MvMemory, PevmTxExecutionResult};

/// Different chains may have varying reward policies.
/// This enum specifies which policy to follow, with optional
/// pre-calculated data to assist in reward calculations.
#[derive(Debug, Clone)]
pub enum RewardPolicy {
    /// Ethereum
    Ethereum,
    /// Optimism
    #[cfg(feature = "optimism")]
    Optimism {
        /// L1 Fee Receipient
        l1_fee_recipient_location_hash: crate::MemoryLocationHash,
        /// Base Fee Vault
        base_fee_vault_location_hash: crate::MemoryLocationHash,
    },
}

/// Custom behaviours for different chains & networks
pub trait PevmChain: Debug {
    /// The transaction type
    type Transaction: Debug + Clone + PartialEq;

    /// The error type for [Self::get_block_spec].
    type BlockSpecError: Debug + Clone + PartialEq;

    /// The error type for [Self::get_gas_price].
    type GasPriceError: Debug + Clone + PartialEq;

    /// The error type for [Self::get_tx_env].
    type TxEnvError: Debug + Clone + PartialEq;

    /// Get chain id.
    fn id(&self) -> u64;

    /// Get block's [SpecId]
    fn get_block_spec(&self, header: &Header) -> Result<SpecId, Self::BlockSpecError>;

    /// Get tx gas price.
    fn get_gas_price(&self, tx: &Self::Transaction) -> Result<U256, Self::GasPriceError>;

    /// Build [MvMemory]
    fn build_mv_memory(
        &self,
        _hasher: &ahash::RandomState,
        _block_env: &BlockEnv,
        txs: &[TxEnv],
    ) -> MvMemory {
        MvMemory::new(txs.len(), [], [])
    }

    /// Get [Handler]
    fn get_handler<'a, EXT, DB: revm::Database>(
        &self,
        spec_id: SpecId,
        with_reward_beneficiary: bool,
    ) -> Handler<'a, revm::Context<EXT, DB>, EXT, DB>;

    /// Get [RewardPolicy]
    fn get_reward_policy(&self, hasher: &ahash::RandomState) -> RewardPolicy;

    /// Calculate receipt root
    fn calculate_receipt_root(
        &self,
        spec_id: SpecId,
        txs: &BlockTransactions<Self::Transaction>,
        tx_results: &[PevmTxExecutionResult],
    ) -> B256;

    /// Get [TxEnv]
    fn get_tx_env(&self, tx: Self::Transaction) -> Result<TxEnv, Self::TxEnvError>;
}

mod ethereum;
pub use ethereum::PevmEthereum;

#[cfg(feature = "optimism")]
pub(crate) mod optimism;
#[cfg(feature = "optimism")]
pub use optimism::PevmOptimism;
