#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::format;
use core::marker::PhantomData;

use frame_support::dispatch::{GetDispatchInfo, Pays, PostDispatchInfo};
use frame_system::RawOrigin;
use pallet_evm::{
    AddressMapping, BalanceConverter, ExitError, GasWeightMapping, IsPrecompileResult, Precompile,
    PrecompileFailure, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use precompile_utils::EvmResult;
use sp_core::{H160, U256, blake2_256, crypto::ByteArray};
use sp_runtime::traits::Dispatchable;
use sp_runtime::traits::StaticLookup;
use subtensor_runtime_common::ProxyType;

use pallet_admin_utils::{PrecompileEnable, PrecompileEnum};

use crate::balance_transfer::*;
use crate::ed25519::*;
use crate::extensions::*;
use crate::metagraph::*;
use crate::neuron::*;
use crate::staking::*;
use crate::subnet::*;

mod balance_transfer;
mod ed25519;
mod extensions;
mod metagraph;
mod neuron;
mod staking;
mod subnet;

pub struct Precompiles<R>(PhantomData<R>);

impl<R> Default for Precompiles<R>
where
    R: frame_system::Config
        + pallet_evm::Config
        + pallet_balances::Config
        + pallet_admin_utils::Config
        + pallet_subtensor::Config
        + pallet_proxy::Config<ProxyType = ProxyType>,
    R::AccountId: From<[u8; 32]> + ByteArray,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
        + From<pallet_proxy::Call<R>>
        + From<pallet_balances::Call<R>>
        + From<pallet_admin_utils::Call<R>>
        + GetDispatchInfo
        + Dispatchable<PostInfo = PostDispatchInfo>,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <R as pallet_balances::Config>::Balance: TryFrom<U256>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Precompiles<R>
where
    R: frame_system::Config
        + pallet_evm::Config
        + pallet_balances::Config
        + pallet_admin_utils::Config
        + pallet_subtensor::Config
        + pallet_proxy::Config<ProxyType = ProxyType>,
    R::AccountId: From<[u8; 32]> + ByteArray,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
        + From<pallet_proxy::Call<R>>
        + From<pallet_balances::Call<R>>
        + From<pallet_admin_utils::Call<R>>
        + GetDispatchInfo
        + Dispatchable<PostInfo = PostDispatchInfo>,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <R as pallet_balances::Config>::Balance: TryFrom<U256>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn used_addresses() -> [H160; 14] {
        [
            hash(1),
            hash(2),
            hash(3),
            hash(4),
            hash(5),
            hash(1024),
            hash(1025),
            hash(Ed25519Verify::<R::AccountId>::INDEX),
            hash(BalanceTransferPrecompile::<R>::INDEX),
            hash(StakingPrecompile::<R>::INDEX),
            hash(SubnetPrecompile::<R>::INDEX),
            hash(MetagraphPrecompile::<R>::INDEX),
            hash(NeuronPrecompile::<R>::INDEX),
            hash(StakingPrecompileV2::<R>::INDEX),
        ]
    }
}
impl<R> PrecompileSet for Precompiles<R>
where
    R: frame_system::Config
        + pallet_evm::Config
        + pallet_balances::Config
        + pallet_admin_utils::Config
        + pallet_subtensor::Config
        + pallet_proxy::Config<ProxyType = ProxyType>,
    R::AccountId: From<[u8; 32]> + ByteArray,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
        + From<pallet_proxy::Call<R>>
        + From<pallet_balances::Call<R>>
        + From<pallet_admin_utils::Call<R>>
        + GetDispatchInfo
        + Dispatchable<PostInfo = PostDispatchInfo>,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <R as pallet_balances::Config>::Balance: TryFrom<U256>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        match handle.code_address() {
            // Ethereum precompiles :
            a if a == hash(1) => Some(ECRecover::execute(handle)),
            a if a == hash(2) => Some(Sha256::execute(handle)),
            a if a == hash(3) => Some(Ripemd160::execute(handle)),
            a if a == hash(4) => Some(Identity::execute(handle)),
            a if a == hash(5) => Some(Modexp::execute(handle)),
            // Non-Frontier specific nor Ethereum precompiles :
            a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
            a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
            a if a == hash(Ed25519Verify::<R::AccountId>::INDEX) => {
                Some(Ed25519Verify::<R::AccountId>::execute(handle))
            }
            // Subtensor specific precompiles :
            a if a == hash(BalanceTransferPrecompile::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::BalanceTransfer) {
                    Some(BalanceTransferPrecompile::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other(
                            "Precompile Balance Transfer is disabled".into(),
                        ),
                    }))
                }
            }
            a if a == hash(StakingPrecompile::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::Staking) {
                    Some(StakingPrecompile::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Precompile Staking is disabled".into()),
                    }))
                }
            }
            a if a == hash(StakingPrecompileV2::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::Staking) {
                    Some(StakingPrecompileV2::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Precompile Staking is disabled".into()),
                    }))
                }
            }

            a if a == hash(SubnetPrecompile::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::Subnet) {
                    Some(SubnetPrecompile::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Precompile Subnet is disabled".into()),
                    }))
                }
            }
            a if a == hash(MetagraphPrecompile::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::Metagraph) {
                    Some(MetagraphPrecompile::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Precompile Metagrah is disabled".into()),
                    }))
                }
            }
            a if a == hash(NeuronPrecompile::<R>::INDEX) => {
                if PrecompileEnable::<R>::get(PrecompileEnum::Neuron) {
                    Some(NeuronPrecompile::<R>::execute(handle))
                } else {
                    Some(Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Precompile Neuron is disabled".into()),
                    }))
                }
            }
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        IsPrecompileResult::Answer {
            is_precompile: Self::used_addresses().contains(&address),
            extra_cost: 0,
        }
    }
}

fn hash(a: u64) -> H160 {
    H160::from_low_u64_be(a)
}
