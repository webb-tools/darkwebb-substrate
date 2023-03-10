use super::*;
use crate::mock::*;
use crate::{
	Error, Instance2,
};

use ark_serialize::CanonicalSerialize;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok};
use crate::Instance1;

use sp_runtime::traits::Zero;

use webb_primitives::{
	types::vanchor::ProofData,
	webb_proposals::{
		FunctionSignature, ResourceId, SubstrateTargetSystem, TargetSystem, TypedChainId,
	},
};
use num_bigint::{BigInt, Sign};
use ark_bn254::{Bn254, Fr};
use std::{
	sync::Mutex,
	fs::File,
	str::FromStr,
	// default::Default,
};
use ark_circom::{WitnessCalculator, read_zkey};
use ark_groth16::ProvingKey;
use ark_relations::r1cs::ConstraintMatrices;
use circom_proving::{circom_from_folder, generate_proof, ProofError, verify_proof};

use arkworks_setups::{common::setup_params, Curve};


const SEED: u32 = 0;
const START_TIMESTAMP: u64 = 0;
const INITIAL_LIQUIDITY: u128 = 10000000;
const LIQUIDITY: u128 = 20000000;
const INITIAL_TOTAL_REWARDS_BALANCE: i128 = 30000000;
const DURATION: u64 = 31536000;

const TEST_MAX_EDGES: u32 = 100;
const TEST_TREE_DEPTH: u8 = 32;

#[test]
fn should_initialize_parameters() {
	new_test_ext().execute_with(|| {});
}

fn setup_environment() {
	for account_id in [
		account::<AccountId>("", 1, SEED),
		account::<AccountId>("", 2, SEED),
		account::<AccountId>("", 3, SEED),
		account::<AccountId>("", 4, SEED),
		account::<AccountId>("", 5, SEED),
	] {
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), account_id, 100_000_000, 0));
	}
}
fn setup_environment_with_circom(
) -> ((ProvingKey<Bn254>, ConstraintMatrices<Fr>), &'static Mutex<WitnessCalculator>) {
	let curve = Curve::Bn254;
	let params3 = setup_params::<ark_bn254::Fr>(curve, 5, 3);

	// println!("Setting up ZKey");
	// let path_2_2 = "/home/semar/Projects/protocol-substrate/pallets/anonymity-mining-rewards/solidity-fixtures/solidity-fixtures/reward_2/30/circuit_final.zkey";
	// let mut file_2_2 = File::open(path_2_2).unwrap();
	// let params_2_2 = read_zkey(&mut file_2_2).unwrap();
	//
	// let wasm_2_2_path = "/home/semar/Projects/protocol-substrate/pallets/anonymity-mining-rewards/solidity-fixtures/solidity-fixtures/reward_2/30/reward_30_2.wasm";
	//
	// let wc_2_2 = circom_from_folder(wasm_2_2_path);

	setup_environment();

	HasherPallet::force_set_parameters(
		RuntimeOrigin::root(),
		params3.to_bytes().try_into().unwrap(),
	).unwrap();

	(params_2_2, wc_2_2)
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
struct InputsRaw {
	rate: String,
	fee: String,
	#[serde(rename = "rewardNullifier")]
	reward_nullifier: String,
	#[serde(rename = "extDataHash")]
	ext_data_hash: String,
	#[serde(rename = "noteChainID")]
	note_chain_id: String,
	#[serde(rename = "noteAmount")]
	note_amount: String,
	#[serde(rename = "noteAssetID")]
	note_asset_id: String,
	#[serde(rename = "noteTokenID")]
	note_token_id: String,
	#[serde(rename = "note_ak_X")]
	note_ak_x: String,
	#[serde(rename = "note_ak_Y")]
	note_ak_y: String,
	#[serde(rename = "noteBlinding")]
	note_blinding: String,
	#[serde(rename = "notePathElements")]
	note_path_elements: Vec<String>,
	#[serde(rename = "notePathIndices")]
	note_path_indices: String,
	note_alpha: String,
	#[serde(rename = "note_ak_alpha_X")]
	note_ak_alpha_x: String,
	#[serde(rename = "note_ak_alpha_Y")]
	note_ak_alpha_y: String,
	#[serde(rename = "inputChainID")]
	input_chain_id: String,
	#[serde(rename = "inputAmount")]
	input_amount: String,
	#[serde(rename = "inputPrivateKey")]
	input_private_key: String,
	#[serde(rename = "inputBlinding")]
	input_blinding: String,
	#[serde(rename = "inputNullifier")]
	input_nullifier: String,
	#[serde(rename = "inputRoot")]
	input_root: String,
	#[serde(rename = "inputPathElements")]
	input_path_elements: Vec<String>,
	#[serde(rename = "inputPathIndices")]
	input_path_indices: String,
	#[serde(rename = "outputChainID")]
	output_chain_id: String,
	#[serde(rename = "outputAmount")]
	output_amount: String,
	#[serde(rename = "outputPrivateKey")]
	output_private_key: String,
	#[serde(rename = "outputBlinding")]
	output_blinding: String,
	#[serde(rename = "outputCommitment")]
	output_commitment: String,
	#[serde(rename = "unspentTimestamp")]
	unspent_timestamp: String,
	#[serde(rename = "unspentRoots")]
	unspent_roots: Vec<String>,
	#[serde(rename = "unspentPathIndices")]
	unspent_path_indices: String,
	#[serde(rename = "unspentPathElements")]
	unspent_path_elements: Vec<String>,
	#[serde(rename = "spentTimestamp")]
	spent_timestamp: String,
	#[serde(rename = "spentRoots")]
	spent_roots: Vec<String>,
	#[serde(rename = "spentPathIndices")]
	spent_path_indices: String,
	#[serde(rename = "spentPathElements")]
	spent_path_elements: Vec<String>,
}

#[derive(Debug)]
struct RewardCircuitInputs {
	rate: Vec<BigInt>,
	fee: Vec<BigInt>,
	reward_nullifier: Vec<BigInt>,
	ext_data_hash: Vec<BigInt>,
	note_chain_id: Vec<BigInt>,
	note_amount: Vec<BigInt>,
	note_asset_id: Vec<BigInt>,
	note_token_id: Vec<BigInt>,
	note_ak_x: Vec<BigInt>,
	note_ak_y: Vec<BigInt>,
	note_blinding: Vec<BigInt>,
	note_path_elements: Vec<BigInt>,
	note_path_indices: Vec<BigInt>,
	note_alpha: Vec<BigInt>,
	note_ak_alpha_x: Vec<BigInt>,
	note_ak_alpha_y: Vec<BigInt>,
	input_chain_id: Vec<BigInt>,
	input_amount: Vec<BigInt>,
	input_private_key: Vec<BigInt>,
	input_blinding: Vec<BigInt>,
	input_nullifier: Vec<BigInt>,
	input_root: Vec<BigInt>,
	input_path_elements: Vec<BigInt>,
	input_path_indices: Vec<BigInt>,
	output_chain_id: Vec<BigInt>,
	output_amount: Vec<BigInt>,
	output_private_key: Vec<BigInt>,
	output_blinding: Vec<BigInt>,
	output_commitment: Vec<BigInt>,
	unspent_timestamp: Vec<BigInt>,
	unspent_roots: Vec<BigInt>,
	unspent_path_indices: Vec<BigInt>,
	unspent_path_elements: Vec<BigInt>,
	spent_timestamp: Vec<BigInt>,
	spent_roots: Vec<BigInt>,
	spent_path_indices: Vec<BigInt>,
	spent_path_elements: Vec<BigInt>,
}

impl RewardCircuitInputs {
	pub fn from_raw(inputs: &InputsRaw) -> Self {
		let rate = vec![Self::to_bigint(&inputs.rate)];
		let fee = vec![Self::to_bigint(&inputs.fee)];
		let reward_nullifier = vec![Self::to_bigint(&inputs.reward_nullifier)];
		let ext_data_hash = vec![Self::to_bigint(&inputs.ext_data_hash)];
		let note_chain_id = vec![Self::to_bigint(&inputs.note_chain_id)];
		let note_amount = vec![Self::to_bigint(&inputs.note_amount)];
		let note_asset_id = vec![Self::to_bigint(&inputs.note_asset_id)];
		let note_token_id = vec![Self::to_bigint(&inputs.note_token_id)];
		let note_ak_x = vec![Self::to_bigint(&inputs.note_ak_x)];
		let note_ak_y = vec![Self::to_bigint(&inputs.note_ak_y)];
		let note_blinding = vec![Self::to_bigint(&inputs.note_blinding)];
		let note_path_elements = inputs.note_path_elements.iter().map(|val| Self::to_bigint(&val)).collect();
		let note_path_indices = vec![Self::to_bigint(&inputs.note_path_indices)];
		let note_alpha = vec![Self::to_bigint(&inputs.note_alpha)];
		let note_ak_alpha_x = vec![Self::to_bigint(&inputs.note_ak_alpha_x)];
		let note_ak_alpha_y = vec![Self::to_bigint(&inputs.note_ak_alpha_y)];
		let input_chain_id = vec![Self::to_bigint(&inputs.input_chain_id)];
		let input_amount = vec![Self::to_bigint(&inputs.input_amount)];
		let input_private_key = vec![Self::to_bigint(&inputs.input_private_key)];
		let input_blinding = vec![Self::to_bigint(&inputs.input_blinding)];
		let input_nullifier = vec![Self::to_bigint(&inputs.input_nullifier)];
		let input_root = vec![Self::to_bigint(&inputs.input_root)];
		let input_path_elements = inputs.input_path_elements.iter().map(|val| Self::to_bigint(&val)).collect();

		let input_path_indices = vec![Self::to_bigint(&inputs.input_path_indices)];
		let output_chain_id = vec![Self::to_bigint(&inputs.output_chain_id)];
		let output_amount = vec![Self::to_bigint(&inputs.output_amount)];
		let output_private_key = vec![Self::to_bigint(&inputs.output_private_key)];
		let output_blinding = vec![Self::to_bigint(&inputs.output_blinding)];
		let output_commitment = vec![Self::to_bigint(&inputs.output_commitment)];
		let unspent_timestamp = vec![Self::to_bigint(&inputs.unspent_timestamp)];
		let unspent_roots = inputs.unspent_roots.iter().map(|root|  Self::to_bigint(&root)).collect();
		let unspent_path_indices = vec![Self::to_bigint(&inputs.unspent_path_indices)];
		let unspent_path_elements = inputs.unspent_path_elements.iter().map(|val| Self::to_bigint(&val)).collect();
		let spent_timestamp = vec![Self::to_bigint(&inputs.spent_timestamp)];
		let spent_roots = inputs.spent_roots.iter().map(|val| Self::to_bigint(&val)).collect();
		let spent_path_indices = vec![Self::to_bigint(&inputs.spent_path_indices)];
		let spent_path_elements = inputs.spent_path_elements.iter().map(|val| Self::to_bigint(&val)).collect();
		Self {
			rate,
			fee,
			reward_nullifier,
			ext_data_hash,
			note_chain_id,
			note_amount,
			note_asset_id,
			note_token_id,
			note_ak_x,
			note_ak_y,
			note_blinding,
			note_path_elements,
			note_path_indices,
			note_alpha,
			note_ak_alpha_x,
			note_ak_alpha_y,
			input_chain_id,
			input_amount,
			input_private_key,
			input_blinding,
			input_nullifier,
			input_root,
			input_path_elements,
			input_path_indices,
			output_chain_id,
			output_amount,
			output_private_key,
			output_blinding,
			output_commitment,
			unspent_timestamp,
			unspent_roots,
			unspent_path_indices,
			unspent_path_elements,
			spent_timestamp,
			spent_roots,
			spent_path_indices,
			spent_path_elements,
		}
	}
	fn to_bigint(str_value: &str) -> BigInt {
		match str_value {
			hex_string if hex_string.starts_with("0x") =>
				BigInt::from_bytes_be(Sign::Plus, &hex::decode(&hex_string[2..]).unwrap()),
			decimal_string => BigInt::from_str(decimal_string).unwrap(),
		}
	}
}

#[test]
// #[ignore]
fn circom_should_complete_30x2_reward_claim_with_json_file() {
	new_test_ext().execute_with(|| {
		let (params_2_2, wc_2_2) = setup_environment_with_circom();
		let raw = include_str!("../circuitInput.json");
		let inputs_raw: InputsRaw = serde_json::from_str(raw).unwrap();
		let circuit_inputs: RewardCircuitInputs = RewardCircuitInputs::from_raw(&inputs_raw);
		// println!("inputs: {inputs_raw:?}");
		println!("circuitInputs: {circuit_inputs:?}");
		let max_edges = 2u32;
		let depth = 30u8;
		let call = AnonymityMiningClaims::create(
			None,
			depth,
			max_edges,
			0u32,
			1u32.into()
		);
		assert_ok!(call);


		let inputs_for_proof = [
			("rate", circuit_inputs.rate.clone()),
			("fee", circuit_inputs.fee.clone()),
			("rewardNullifier", circuit_inputs.reward_nullifier.clone()),
			("extDataHash", circuit_inputs.ext_data_hash.clone()),
			("noteChainID", circuit_inputs.note_chain_id.clone()),
			("noteAmount", circuit_inputs.note_amount.clone()),
			("noteAssetID", circuit_inputs.note_asset_id.clone()),
			("noteTokenID", circuit_inputs.note_token_id.clone()),
			("note_ak_X", circuit_inputs.note_ak_x.clone()),
			("note_ak_Y", circuit_inputs.note_ak_y.clone()),
			("noteBlinding", circuit_inputs.note_blinding.clone()),
			("notePathElements", circuit_inputs.note_path_elements.clone()),
			("notePathIndices", circuit_inputs.note_path_indices.clone()),
			("note_alpha", circuit_inputs.note_alpha.clone()),
			("note_ak_alpha_X", circuit_inputs.note_ak_alpha_x.clone()),
			("note_ak_alpha_Y", circuit_inputs.note_ak_alpha_y.clone()),
			("inputChainID", circuit_inputs.input_chain_id.clone()),
			("inputAmount", circuit_inputs.input_amount.clone()),
			("inputPrivateKey", circuit_inputs.input_private_key.clone()),
			("inputBlinding", circuit_inputs.input_blinding.clone()),
			("inputNullifier", circuit_inputs.input_nullifier.clone()),
			("inputRoot", circuit_inputs.input_root.clone()),
			("inputPathElements", circuit_inputs.input_path_elements.clone()),
			("inputPathIndices", circuit_inputs.input_path_indices.clone()),
			("outputChainID", circuit_inputs.output_chain_id.clone()),
			("outputAmount", circuit_inputs.output_amount.clone()),
			("outputPrivateKey", circuit_inputs.output_private_key.clone()),
			("outputBlinding", circuit_inputs.output_blinding.clone()),
			("outputCommitment", circuit_inputs.output_commitment.clone()),
			("unspentTimestamp", circuit_inputs.unspent_timestamp.clone()),
			("unspentRoots", circuit_inputs.unspent_roots.clone()),
			("unspentPathIndices", circuit_inputs.unspent_path_indices.clone()),
			("unspentPathElements", circuit_inputs.unspent_path_elements.clone()),
			("spentTimestamp", circuit_inputs.spent_timestamp.clone()),
			("spentRoots", circuit_inputs.spent_roots.clone()),
			("spentPathIndices", circuit_inputs.spent_path_indices.clone()),
			("spentPathElements", circuit_inputs.spent_path_elements.clone()),
		];
		let x = generate_proof(wc_2_2, &params_2_2, inputs_for_proof.clone());

		let num_inputs = params_2_2.1.num_instance_variables;

		let (proof, full_assignment) = x.unwrap();

		let mut proof_bytes = Vec::new();
		proof.serialize(&mut proof_bytes).unwrap();

		let reward_proof_data = RewardProofData {
			proof: proof_bytes,
			rate: BigInt::from(1000),
			fee: BigInt::from(0),
			reward_nullifier: BigInt::from(0),
			note_ak_alpha_x: BigInt::from(0),
			note_ak_alpha_y: BigInt::from(0),
			ext_data_hash: BigInt::from(0),
			input_root: BigInt::from(0),
			input_nullifier: BigInt::from(0),
			output_commitment: BigInt::from(0),
			spent_roots: vec![BigInt::from(0)],
			unspent_roots: vec![BigInt::from(0)],
		};

		let inputs_for_verification = &full_assignment[1..num_inputs];

		let did_proof_work =
			verify_proof(&params_2_2.0.vk, &proof, inputs_for_verification.to_vec()).unwrap();
		assert!(did_proof_work);
	});
}

// helper function to create anchor using Anchor pallet call
fn mock_vanchor_creation_using_pallet_call(resource_id: &ResourceId) {
	assert!(!<pallet_mt::Trees<Test, Instance1>>::contains_key(0));
	assert_ok!(VAnchor::create(RuntimeOrigin::root(), TEST_MAX_EDGES, TEST_TREE_DEPTH, 0));
	assert!(<pallet_mt::Trees<Test, Instance1>>::contains_key(0));
	assert_eq!(TEST_MAX_EDGES, <pallet_linkable_tree::MaxEdges<Test, Instance1>>::get(0));
		let max_edges = 2u32;
		let depth = 30u8;
		let call = AnonymityMiningClaims::create(
			None,
			depth,
			max_edges,
			0u32,
			1u32.into()
		);
		assert_ok!(call);
}

// AP claim tests

// Test claim_ap
// #[test]
// fn test_claim_ap() {
// 	new_test_ext().execute_with(|| {
// 		setup_environment();
//
// 		let recipient_one_account_id = account::<AccountId>("", 2, SEED);
// 		let sender_two_account_id = account::<AccountId>("", 3, SEED);
//
// 		let src_id = TypedChainId::Substrate(1);
// 		let target_id = TypedChainId::Substrate(5);
// 		let target_system =
// 			TargetSystem::Substrate(SubstrateTargetSystem { pallet_index: 11, tree_id: 0 });
// 		let r_id: ResourceId = ResourceId::new(target_system, target_id);
//
// 		let root = Element::from_bytes(&[1; 32]);
// 		let latest_leaf_index = 5;
// 		let src_target_system = target_system;
// 		let src_resource_id = ResourceId::new(src_target_system, src_id);
//
// 		let dest_target_system = target_system;
// 		let dest_resource_id = ResourceId::new(dest_target_system, target_id);
//
// 		// print out r_id
// 		println!("r_id: {:?}", r_id);
//
// 		let tree_id = 5;
//
// 		// token setup
// 		let ap_currency_id = 1;
// 		let reward_currency_id = 2;
//
// 		// add reward balance to pallet
// 		let new_reward_balance = INITIAL_TOTAL_REWARDS_BALANCE;
// 		assert_ok!(Currencies::update_balance(
// 			RuntimeOrigin::root(),
// 			AnonymityMiningClaims::account_id(),
// 			reward_currency_id,
// 			new_reward_balance,
// 		));
//
// 		// adding AP balance to pallet
// 		let new_ap_balance = 50000;
// 		assert_ok!(Currencies::update_balance(
// 			RuntimeOrigin::root(),
// 			AnonymityMiningClaims::account_id(),
// 			ap_currency_id,
// 			new_ap_balance,
// 		));
//
// 		// param setup
// 		let curve = Curve::Bn254;
// 		let params = setup_params::<ark_bn254::Fr>(curve, 5, 3);
//
// 		SignatureBridge::whitelist_chain(RuntimeOrigin::root(), src_id.chain_id());
// 		SignatureBridge::set_resource(RuntimeOrigin::root(), r_id);
// 		SignatureBridge::resource_exists(r_id);
//
// 		mock_vanchor_creation_using_pallet_call(&r_id);
//
// 		// mock proof data
// 		let proof_data = ProofData {
// 			proof: vec![],
// 			public_amount: Default::default(),
// 			roots: vec![],
// 			input_nullifiers: vec![],
// 			output_commitments: vec![],
// 			ext_data_hash: Default::default(),
// 		};
//
// 		let reward_proof_data = RewardProofData {
// 			rate: 10,
// 			fee: 1,
// 			note_ak_alpha_x: Default::default(),
// 			note_ak_alpha_y: Default::default(),
// 			ext_data_hash: Default::default(),
// 			input_root: Default::default(),
// 			input_nullifier: Default::default(),
// 			output_commitment: Default::default(),
// 			spent_roots: vec![],
// 			unspent_roots: vec![],
// 		};
//
// 		// mock roots
// 		let unspent_root = Default::default();
// 		let spent_root = Default::default();
//
// 		// mock reward_nullifier_hash
// 		let reward_nullifier_hash = Default::default();
//
// 		let claim_ap_call = AnonymityMiningClaims::claim_ap(
// 			src_resource_id,
// 			dest_resource_id,
// 			recipient_one_account_id,
// 			1000,
// 			root,
// 			latest_leaf_index,
// 			proof_data,
// 			reward_proof_data,
// 			unspent_root,
// 			spent_root,
// 			reward_nullifier_hash,
// 		);
//
// 		assert_ok!(claim_ap_call);
// 	})
// }
//
/// testing update roots
#[test]
fn should_fail_update_without_resource_id_initialization() {
	new_test_ext().execute_with(|| {
		setup_environment();
		let src_id = TypedChainId::Substrate(1);
		let target_system =
			TargetSystem::Substrate(SubstrateTargetSystem { pallet_index: 11, tree_id: 0 });
		let src_target_system = target_system;
		let src_resource_id = ResourceId::new(src_target_system, src_id);

		let raw = include_str!("../circuitInput.json");
		let inputs_raw: InputsRaw = serde_json::from_str(raw).unwrap();
		let circuit_inputs: RewardCircuitInputs = RewardCircuitInputs::from_raw(&inputs_raw);

		let unspent_update_0 = AnonymityMiningClaims::update_unspent_root(
			src_resource_id,
			Element::from_bytes(&circuit_inputs.unspent_roots[0].to_bytes_be().1)
		);
		assert_err!(
			unspent_update_0,
			Error::<Test, Instance1>::InvalidUnspentChainIds,
		);

		let unspent_update_1 = AnonymityMiningClaims::update_unspent_root(
			src_resource_id,
			Element::from_bytes(&circuit_inputs.unspent_roots[1].to_bytes_be().1)
		);
		assert_err!(
			unspent_update_1,
			Error::<Test, Instance1>::InvalidUnspentChainIds,
		);
	})
}

// fn create_claims_pallet(asset_id: u32) -> u32 {
// 	let max_edges = 2u32;
// 	let depth = 30u8;
// 	assert_ok!(AnonymityMiningClaims::create(
// 		None,
// 		depth,
// 		max_edges,
// 		asset_id,
// 		0u32.into()
// 	));
// 	MerkleTree::next_tree_id() - 1
// }
//
#[test]
fn should_create_pallet() {
	new_test_ext().execute_with(|| {
		setup_environment_with_circom();
		let max_edges = 2u32;
		let depth = 30u8;
		let call = AnonymityMiningClaims::create(
			None,
			depth,
			max_edges,
			0u32,
			1u32.into()
		);
		assert_ok!(call);
	})
}
