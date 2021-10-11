#![cfg_attr(not(feature = "std"), no_std)]

use darkwebb_primitives::ElementTrait;

sp_api::decl_runtime_apis! {
	pub trait MerkleTreeApi<E: ElementTrait> {
		/// Get the leaf of tree id at a given index.
		fn get_leaf(tree_id: u32, index: u32) -> Option<E>;
	}
}
