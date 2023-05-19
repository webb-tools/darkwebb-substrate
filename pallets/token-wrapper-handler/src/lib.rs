// This file is part of Webb.

// Copyright (C) 2021-2023 Webb Technologies Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Token Wrapper Handler Module
//!
//! A module for executing wrapping fee, add/remove token proposals.
//! These functions can only be called by the bridge.
//!
//! ## Overview
//!
//! The Token Wrapper Handler module provides functionality for token wrapping
//! management including:
//!
//! * Executing proposal to change the wrapping fee
//! * Executing a proposal to add a token to the asset registry
//! * Executing a proposal to remove a token from the asset registry
//!
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Dispatchable Functions
//! * execute_wrapping_fee_proposal
//! * execute_add_token_to_pool_share
//! * execute_remove_token_from_pool_share
//!
//! ## Interface
//!
//! ## Related Modules
//!
//! * Token-Wrapper Pallet
//! * Bridge Pallet

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock_signature_bridge;
#[cfg(test)]
mod tests_signature_bridge;

use frame_support::traits::EnsureOrigin;
use frame_system::pallet_prelude::OriginFor;
use sp_std::{convert::TryInto, vec::Vec};

use pallet_token_wrapper::{traits::TokenWrapperInterface, BalanceOf};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_token_wrapper::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type BridgeOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// TokenWrapper Interface
		type TokenWrapper: TokenWrapperInterface<
			Self::AccountId,
			Self::AssetId,
			BalanceOf<Self>,
			Self::ProposalNonce,
		>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {
		UpdatedWrappingFeePercent { wrapping_fee_percent: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Access violation.
		InvalidPermissions,
		// Token Wrapper Handler already exists for specified resource Id.
		ResourceIsAlreadyAnchored,
		// Token Wrapper Handler doesn't exist for specified resource Id.
		TokenWrapperHandlerNotFound,
		/// Storage overflowed.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Execute the wrapping fee proposal by calling the update_wrapping_fee
		/// method. Ensures that only the bridge can call this function.
		#[pallet::weight(195_000_000)]
		#[pallet::call_index(0)]
		pub fn execute_wrapping_fee_proposal(
			origin: OriginFor<T>,
			wrapping_fee_percent: BalanceOf<T>,
			into_pool_share_id: T::AssetId,
			nonce: T::ProposalNonce,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			// Set the wrapping fee
			T::TokenWrapper::set_wrapping_fee(into_pool_share_id, wrapping_fee_percent, nonce)?;
			Ok(().into())
		}

		#[pallet::weight(195_000_000)]
		#[pallet::call_index(1)]
		pub fn execute_add_token_to_pool_share(
			origin: OriginFor<T>,
			name: Vec<u8>,
			asset_id: T::AssetId,
			nonce: T::ProposalNonce,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			// Add asset to the pool share
			T::TokenWrapper::add_asset_to_existing_pool(&name, asset_id, nonce)?;
			Ok(().into())
		}

		#[pallet::weight(195_000_000)]
		#[pallet::call_index(2)]
		pub fn execute_remove_token_from_pool_share(
			origin: OriginFor<T>,
			name: Vec<u8>,
			asset_id: T::AssetId,
			nonce: T::ProposalNonce,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			// Remove asset from the pool share
			T::TokenWrapper::delete_asset_from_existing_pool(&name, asset_id, nonce)?;
			Ok(().into())
		}
		/// Executes `SetFeeRecipient` proposal which will set new fee recipient,
		/// who will be receiving wrapping fee cost for given asset.
		/// Ensures that only the bridge can call this function.
		#[pallet::weight(195_000_000)]
		#[pallet::call_index(3)]
		pub fn execute_set_fee_recipient_proposal(
			origin: OriginFor<T>,
			pool_share_id: T::AssetId,
			fee_recipient: T::AccountId,
			nonce: T::ProposalNonce,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			T::TokenWrapper::set_fee_recipient(pool_share_id, fee_recipient, nonce)?;
			Ok(().into())
		}

		/// Executes `RescueTokenProposal`, which will transfer tokens from
		/// fee recipient to given recipient address from given asset.
		/// Ensures that only the bridge can call this function.
		#[pallet::weight(195_000_000)]
		#[pallet::call_index(4)]
		pub fn execute_rescue_tokens_proposal(
			origin: OriginFor<T>,
			pool_share_id: T::AssetId,
			asset_id: T::AssetId,
			amount: BalanceOf<T>,
			recipient: T::AccountId,
			nonce: T::ProposalNonce,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			T::TokenWrapper::rescue_tokens(pool_share_id, asset_id, amount, recipient, nonce)?;
			Ok(().into())
		}
	}
}
