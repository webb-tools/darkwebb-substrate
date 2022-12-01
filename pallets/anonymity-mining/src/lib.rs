// This file is part of Webb.

// Copyright (C) 2021 Webb Technologies Inc.
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

//! # Anonymity Mining Module
//!
//! ## Overview
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

use frame_support::{
	pallet_prelude::{ensure, DispatchError},
	sp_runtime::traits::AccountIdConversion,
	traits::Get,
	PalletId,
};
use sp_std::{convert::TryInto, prelude::*, vec};

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Account Identifier from which the internal Pot is generated.
		type PotId: Get<PalletId>;

		/// The origin which may forcibly reset parameters or otherwise alter
		/// privileged attributes.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weightinfo for pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {}
	}

	#[pallet::storage]
	#[pallet::getter(fn parameters)]
	/// Details of the module's parameters
	pub(super) type Parameters<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Parameters haven't been initialized
		ParametersNotInitialized,
		/// Error during hashing
		HashError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn force_set_parameters(
			origin: OriginFor<T>,
			parameters: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			T::ForceOrigin::ensure_origin(origin)?;
			Parameters::<T>::try_mutate(|params| {
				*params = parameters.clone();
				Ok(().into())
			})
		}

		#[pallet::weight(0)]
		pub fn swap(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Get a unique, inaccessible account id from the `PotId`.
	pub fn account_id() -> T::AccountId {
		T::PotId::get().into_account_truncating()
	}
}