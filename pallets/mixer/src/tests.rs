use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_std::test_rng;
use arkworks_utils::utils::common::{setup_params_x5_3, setup_params_x5_5, Curve};
use codec::Encode;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok, traits::OnInitialize};
use sp_runtime::traits::{One, Zero};

use darkwebb_primitives::{merkle_tree::TreeInspector, AccountId, ElementTrait};
use orml_traits::MultiCurrency;
use pallet_asset_registry::AssetType;
use std::time::{Duration, Instant};

use crate::{mock::*, test_utils::*};

const SEED: u32 = 0;

fn hasher_params() -> Vec<u8> {
	let curve = Curve::Bn254;
	let params = setup_params_x5_3::<ark_bn254::Fr>(curve);
	params.to_bytes()
}

fn setup_environment(curve: Curve) -> Vec<u8> {
	let rng = &mut test_rng();
	let curve = Curve::Bn254;
	let params3 = setup_params_x5_3::<Bn254Fr>(curve);
	let params5 = setup_params_x5_5::<Bn254Fr>(curve);
	let prover = MixerProverSetupBn254_30::new(params3.clone(), params5);

	// 1. Setup The Hasher Pallet.
	assert_ok!(HasherPallet::force_set_parameters(Origin::root(), params3.to_bytes()));
	// 2. Initialize MerkleTree pallet.
	<MerkleTree as OnInitialize<u64>>::on_initialize(1);
	// 3. Setup the VerifierPallet
	//    but to do so, we need to have a VerifyingKey
	let (circuit, .., public_inputs) = prover.setup_random_circuit(rng);
	let (pk, vk) = MixerProverSetupBn254_30::setup_keys::<Bn254, _>(circuit.clone(), rng);

	assert_ok!(VerifierPallet::force_set_parameters(Origin::root(), vk));

	// finally return the provingkey bytes
	pk
}

#[test]
fn should_create_new_mixer() {
	new_test_ext().execute_with(|| {
		// init hasher pallet first.
		assert_ok!(HasherPallet::force_set_parameters(Origin::root(), hasher_params()));
		// then the merkle tree.
		<MerkleTree as OnInitialize<u64>>::on_initialize(1);
		assert_ok!(Mixer::create(Origin::root(), One::one(), 3, 0));
	});
}
#[test]
fn should_be_able_to_deposit() {
	new_test_ext().execute_with(|| {
		// init hasher pallet first.
		assert_ok!(HasherPallet::force_set_parameters(Origin::root(), hasher_params()));
		// then the merkle tree.
		<MerkleTree as OnInitialize<u64>>::on_initialize(1);
		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 3, 0));
		let tree_id = MerkleTree::next_tree_id() - 1;
		let account_id = account::<AccountId>("", 1, SEED);
		let leaf = Element::from_bytes(&[1u8; 32]);
		// check the balance before the deposit.
		let balance_before = Balances::free_balance(account_id.clone());
		// and we do the deposit
		assert_ok!(Mixer::deposit(Origin::signed(account_id.clone()), tree_id, leaf));
		// now we check the balance after the deposit.
		let balance_after = Balances::free_balance(account_id);
		// the balance should be less now with `deposit_size`
		assert_eq!(balance_after, balance_before - deposit_size);
		// now we need also to check if the state got updated.
		let tree = MerkleTree::trees(tree_id);
		assert_eq!(tree.leaf_count, 1);
	});
}
#[test]
fn should_be_able_to_change_the_maintainer() {
	new_test_ext().execute_with(|| {
		assert_ok!(Mixer::create(Origin::root(), One::one(), 3, 0));
		let default_maintainer_account_id = AccountId::default();
		let current_maintainer_account_id = Mixer::maintainer();
		assert_eq!(current_maintainer_account_id, default_maintainer_account_id);
		let new_maintainer_account_id = account::<AccountId>("", 1, SEED);
		assert_ok!(Mixer::force_set_maintainer(
			Origin::root(),
			new_maintainer_account_id.clone()
		));
		let current_maintainer_account_id = Mixer::maintainer();
		assert_eq!(current_maintainer_account_id, new_maintainer_account_id);
	});
}
#[test]
// fn should_have_same_root_as_wasm_utils() {
// 	new_test_ext().execute_with(|| {

// 		let curve = Curve::Bn254;
// 		let pk_bytes = setup_environment(curve);
// 		// now let's create the mixer.
// 		let deposit_size = One::one();
// 		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
// 		// now with mixer created, we should setup the circuit.
// 		let tree_id = MerkleTree::next_tree_id() - 1;
// 		let sender_account_id = account::<AccountId>("", 1, SEED);
// 		let recipient_account_id = account::<AccountId>("", 2, SEED);
// 		let relayer_account_id = account::<AccountId>("", 0, SEED);
// 		let fee_value = 0;
// 		let refund_value = 0;

// 		// inputs
// 		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
// 		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

// 		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
// 			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

// 		assert_ok!(Mixer::deposit(
// 			Origin::signed(sender_account_id.clone()),
// 			tree_id,
// 			leaf_element.clone(),
// 		));

// 		let note_str =
// "webb.bridge:v1:3:2:Arkworks:Bn254:Poseidon:EDG:18:0:5:5:
// 7e0f4bfa263d8b93854772c94851c04b3a9aba38ab808a8d081f6f5be9758110b7147c395ee9bf495734e4703b1f622009c81712520de0bbd5e7a10237c7d829bf6bd6d0729cca778ed9b6fb172bbb12b01927258aca7e0a66fd5691548f8717"
// ; 		let note = wasm_utils::note::JsNote::deserialize(&note_str).unwrap();
// 		let leaf  = leaf_element.to_vec();
// 		let mut  leaf_slice:[u8;32] = [0u8;32];
// 		leaf_slice.copy_from_slice(&leaf[..32]);

// 		let mut proof_builder = wasm_utils::proof::mixer::JsProofInputBuilder::new();
// 		proof_builder.fee =Some(0);
// 		proof_builder.refund  = Some(0);
// 		proof_builder.leaf_index = Some(0);

// 		proof_builder.recipient = Some(recipient_account_id.encode());
// 		proof_builder.relayer = Some(relayer_account_id.encode());
// 		proof_builder.leaves = Some(vec![leaf_slice]);
// 		let proof_input = proof_builder.build().unwrap();
// 		let proof  = wasm_utils::proof::mixer::generate_proof_js(note ,proof_input).unwrap();

// 		// check the balance before the withdraw.
// 		let balance_before = Balances::free_balance(recipient_account_id.clone());

// 		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
// 		assert_eq!(roots_element[0], mixer_tree_root);
// 		let root_from_node = roots_element[0].to_vec();
// 		let nullifier_hash_from_node = nullifier_hash_element.to_vec();

// 		assert_eq!(root_from_node , proof.root);
// 		assert_eq!(nullifier_hash_from_node , proof.nullifier_hash);

// 		assert_ok!(Mixer::withdraw(
// 			Origin::signed(sender_account_id),
// 			tree_id,
// 			proof_bytes,
// 			roots_element[0],
// 			nullifier_hash_element,
// 			recipient_account_id.clone(),
// 			relayer_account_id,
// 			fee_value.into(),
// 			refund_value.into(),
// 		));
// 		// now we check the recipient balance again.
// 		let balance_after = Balances::free_balance(recipient_account_id);
// 		assert_eq!(balance_after, balance_before + deposit_size);
// 		// perfect
// 	});
// }
#[test]
fn mixer_works() {
	new_test_ext().execute_with(|| {
		let now = Instant::now();
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);
		// now let's create the mixer.
		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));
		// check the balance before the withdraw.
		let balance_before = Balances::free_balance(recipient_account_id.clone());

		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(roots_element[0], mixer_tree_root);

		assert_ok!(Mixer::withdraw(
			Origin::signed(sender_account_id),
			tree_id,
			proof_bytes,
			roots_element[0],
			nullifier_hash_element,
			recipient_account_id.clone(),
			relayer_account_id,
			fee_value.into(),
			refund_value.into(),
		));
		// now we check the recipient balance again.
		let balance_after = Balances::free_balance(recipient_account_id);
		assert_eq!(balance_after, balance_before + deposit_size);
		// perfect
		println!("{:?}", now.elapsed());
	});
}

#[test]
fn mixer_should_fail_with_when_proof_when_any_byte_is_changed_in_proof() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (mut proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let root_element = roots_element[0];
		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(root_element, mixer_tree_root);

		let a = proof_bytes[0];
		let b = proof_bytes[1];
		proof_bytes[0] = b;
		proof_bytes[1] = a;

		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				relayer_account_id,
				fee_value.into(),
				refund_value.into(),
			),
			crate::Error::<Test>::InvalidWithdrawProof
		);
	});
}

#[test]
fn mixer_should_fail_when_invalid_merkle_roots() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;

		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let mut root_element_bytes = roots_element[0].to_bytes().to_vec();
		let a = root_element_bytes[0];
		let b = root_element_bytes[1];
		root_element_bytes[0] = b;
		root_element_bytes[1] = a;
		let root_element = Element::from_bytes(&root_element_bytes[..]);

		// now we start to generate the proof.
		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				relayer_account_id,
				fee_value.into(),
				refund_value.into(),
			),
			crate::Error::<Test>::UnknownRoot
		);
	});
}

#[test]
fn mixer_should_fail_when_relayer_id_is_different_from_that_in_proof_generation() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let root_element = roots_element[0];
		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(root_element, mixer_tree_root);

		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id.clone()),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				sender_account_id,
				fee_value.into(),
				refund_value.into(),
			),
			crate::Error::<Test>::InvalidWithdrawProof
		);
	});
}

#[test]
fn mixer_should_fail_with_when_fee_submitted_is_changed() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let root_element = roots_element[0];
		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(root_element, mixer_tree_root);

		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				relayer_account_id,
				100u128,
				refund_value.into(),
			),
			crate::Error::<Test>::InvalidWithdrawProof
		);
	});
}

#[test]
fn mixer_should_fail_with_invalid_proof_when_account_ids_are_truncated_in_reverse() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = truncate_and_pad_reverse(&recipient_account_id.encode()[..]);
		let relayer_bytes = truncate_and_pad_reverse(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let root_element = roots_element[0];
		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(root_element, mixer_tree_root);

		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				relayer_account_id,
				fee_value.into(),
				refund_value.into(),
			),
			crate::Error::<Test>::InvalidWithdrawProof
		);
	});
}

#[test]
fn double_spending_should_fail() {
	new_test_ext().execute_with(|| {
		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, 0));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (proof_bytes, roots_element, nullifier_hash_element, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);

		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));

		let root_element = roots_element[0];
		let mixer_tree_root = MerkleTree::get_root(tree_id).unwrap();
		assert_eq!(root_element, mixer_tree_root);

		let balance_before = Balances::free_balance(recipient_account_id.clone());

		assert_ok!(Mixer::withdraw(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			proof_bytes.clone(),
			root_element,
			nullifier_hash_element,
			recipient_account_id.clone(),
			relayer_account_id.clone(),
			fee_value.into(),
			refund_value.into(),
		));
		// now we check the recipient balance again.
		let balance_after = Balances::free_balance(recipient_account_id.clone());
		assert_eq!(balance_after, balance_before + deposit_size);
		// perfect

		assert_err!(
			Mixer::withdraw(
				Origin::signed(sender_account_id),
				tree_id,
				proof_bytes,
				root_element,
				nullifier_hash_element,
				recipient_account_id,
				relayer_account_id,
				fee_value.into(),
				refund_value.into(),
			),
			crate::Error::<Test>::AlreadyRevealedNullifier
		);
	});
}

#[test]
fn deposit_with_non_native_asset_should_work() {
	new_test_ext().execute_with(|| {
		// create an Asset first
		assert_ok!(
			AssetRegistry::get_or_create_asset(String::from("ETH").into(), AssetType::Token, Zero::zero()),
			1
		);

		let currency_id = AssetRegistry::next_asset_id() - 1;

		let curve = Curve::Bn254;
		let pk_bytes = setup_environment(curve);

		let deposit_size = One::one();
		assert_ok!(Mixer::create(Origin::root(), deposit_size, 30, currency_id));
		// now with mixer created, we should setup the circuit.
		let tree_id = MerkleTree::next_tree_id() - 1;
		let sender_account_id = account::<AccountId>("", 1, SEED);
		let recipient_account_id = account::<AccountId>("", 2, SEED);
		let relayer_account_id = account::<AccountId>("", 0, SEED);
		let fee_value = 0;
		let refund_value = 0;

		// inputs
		let recipient_bytes = crate::truncate_and_pad(&recipient_account_id.encode()[..]);
		let relayer_bytes = crate::truncate_and_pad(&relayer_account_id.encode()[..]);

		let (_, _, _, leaf_element) =
			setup_zk_circuit(curve, recipient_bytes, relayer_bytes, pk_bytes, fee_value, refund_value);
		// check my balance first, before sending the deposit
		assert_eq!(Currencies::free_balance(currency_id, &sender_account_id), Zero::zero());
		// now we add some balance
		let new_balance = 100;
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			sender_account_id.clone(),
			currency_id,
			new_balance,
		));
		// now we do check the balance again, it should be updated
		assert_eq!(
			Currencies::free_balance(currency_id, &sender_account_id),
			new_balance as _
		);
		// and then we do the deposit
		assert_ok!(Mixer::deposit(
			Origin::signed(sender_account_id.clone()),
			tree_id,
			leaf_element,
		));
		// our balance should be updated again
		assert_eq!(
			Currencies::free_balance(currency_id, &sender_account_id),
			(new_balance - deposit_size as i128) as _
		);
	});
}
