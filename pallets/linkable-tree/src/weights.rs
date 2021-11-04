//! Autogenerated weights for `pallet_anchor`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_anchor
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --raw
// --output
// ./pallets/anchor/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create(i: u32, d:u32) -> Weight;
	fn set_maintainer() -> Weight;
	fn force_set_maintainer() -> Weight;
}

/// Weight functions for `pallet_anchor`.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	// Storage: MerkleTree NextTreeId (r:1 w:1)
	// Storage: MerkleTree DefaultHashes (r:1 w:0)
	// Storage: Mixer Mixers (r:0 w:1)
	// Storage: MerkleTree Trees (r:0 w:1)
	// Storage: Anchor MaxEdges (r:0 w:1)
	fn create(_i: u32, _d: u32, ) -> Weight {
		(31_320_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Anchor Maintainer (r:1 w:1)
	fn set_maintainer() -> Weight {
		(23_195_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Anchor Maintainer (r:1 w:1)
	fn force_set_maintainer() -> Weight {
		(19_648_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}


impl WeightInfo for  () {
	fn create(_i: u32, _d:u32) -> Weight {
		0
	}
	fn set_maintainer() -> Weight {
		0
	}
	fn force_set_maintainer() -> Weight {
		0
	}
}