//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-31, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE:
//! `[]` EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("statemine-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/statemint
// benchmark
// --chain=statemine-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_proxy
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./runtime/statemine/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_proxy.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	fn proxy(p: u32) -> Weight {
		27_318_000_u64
			// Standard Error: 1_000
			.saturating_add(208_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}

	fn proxy_announced(a: u32, p: u32) -> Weight {
		60_665_000_u64
			// Standard Error: 2_000
			.saturating_add(677_000_u64.saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add(197_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn remove_announcement(a: u32, p: u32) -> Weight {
		39_455_000_u64
			// Standard Error: 2_000
			.saturating_add(687_000_u64.saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add(3_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn reject_announcement(a: u32, p: u32) -> Weight {
		39_411_000_u64
			// Standard Error: 2_000
			.saturating_add(686_000_u64.saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add(3_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn announce(a: u32, p: u32) -> Weight {
		54_386_000_u64
			// Standard Error: 2_000
			.saturating_add(677_000_u64.saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add(194_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn add_proxy(p: u32) -> Weight {
		37_411_000_u64
			// Standard Error: 2_000
			.saturating_add(298_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn remove_proxy(p: u32) -> Weight {
		36_658_000_u64
			// Standard Error: 2_000
			.saturating_add(332_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn remove_proxies(p: u32) -> Weight {
		34_893_000_u64
			// Standard Error: 1_000
			.saturating_add(209_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn anonymous(p: u32) -> Weight {
		51_243_000_u64
			// Standard Error: 1_000
			.saturating_add(44_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn kill_anonymous(p: u32) -> Weight {
		37_188_000_u64
			// Standard Error: 1_000
			.saturating_add(208_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}