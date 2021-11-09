
//! Autogenerated weights for `pallet_mixer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/darkwebb-standalone-node
// benchmark
// --chain=dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_mixer
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --raw
// --output
// ./pallets/mixer/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create(d:u32) -> Weight;
	fn deposit() -> Weight;
	fn set_maintainer() -> Weight;
	fn force_set_maintainer() -> Weight;
	fn withdraw() -> Weight;
}

/// Weight functions for `pallet_mixer`.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	// Storage: MerkleTree NextTreeId (r:1 w:1)
	// Storage: MerkleTree DefaultHashes (r:1 w:0)
	// Storage: Mixer Mixers (r:0 w:1)
	// Storage: MerkleTree Trees (r:0 w:1)
	fn create(d: u32, ) -> Weight {
		34_099_000_u64
			// Standard Error: 5_000
			.saturating_add(43_000_u64.saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: MerkleTree Trees (r:1 w:1)
	// Storage: MerkleTree DefaultHashes (r:1 w:0)
	// Storage: BN254CircomPoseidon3x5Hasher Parameters (r:1 w:0)
	// Storage: MerkleTree NextRootIndex (r:1 w:1)
	// Storage: MerkleTree NextLeafIndex (r:1 w:1)
	// Storage: Mixer Mixers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: MerkleTree Leaves (r:0 w:1)
	// Storage: MerkleTree CachedRoots (r:0 w:1)
	fn deposit() -> Weight {
		9_847_660_000_u64
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Mixer Maintainer (r:1 w:1)
	fn set_maintainer() -> Weight {
		22_199_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Mixer Maintainer (r:1 w:1)
	fn force_set_maintainer() -> Weight {
		19_758_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Mixer Mixers (r:1 w:0)
	// Storage: MerkleTree Trees (r:1 w:0)
	// Storage: MerkleTree CachedRoots (r:1 w:0)
	// Storage: Mixer NullifierHashes (r:1 w:1)
	// Storage: MixerVerifier Parameters (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn withdraw() -> Weight {
		35_264_966_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

impl WeightInfo for () {
	fn create(_d:u32) -> Weight {
		0
	}
	fn deposit() -> Weight{
		0
	}
	fn set_maintainer() -> Weight {
		0
	}
	fn force_set_maintainer() -> Weight {
		0
	}
	fn withdraw() -> Weight{
		0
	}
}
