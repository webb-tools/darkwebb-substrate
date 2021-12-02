use super::{IntoAbiToken, Token};
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct VAnchorMetadata<AccountId, AssetId> {
	/// Creator account
	pub creator: AccountId,
	/// Option of specifying a fungible asset. When None, the asset is the
	/// native currency.
	pub asset: AssetId,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, TypeInfo)]
pub struct ProofData<Element, Balance> {
	pub proof: Vec<u8>,
	pub roots: Vec<Element>,
	pub input_nullifiers: Vec<Element>,
	pub output_commitments: Vec<Element>,
	pub public_amount: Balance,
	pub ext_data_hash: Element,
}

impl<E, B> ProofData<E, B> {
	pub fn new(
		proof: Vec<u8>,
		roots: Vec<E>,
		input_nullifiers: Vec<E>,
		output_commitments: Vec<E>,
		public_amount: B,
		ext_data_hash: E,
	) -> Self {
		Self {
			proof,
			roots,
			input_nullifiers,
			output_commitments,
			public_amount,
			ext_data_hash,
		}
	}
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, TypeInfo)]
pub struct ExtData<AccountId: Encode, Amount: Encode, Balance: Encode, Element: Encode> {
	pub recipient: AccountId,
	pub relayer: AccountId,
	pub ext_amount: Amount,
	pub fee: Balance,
	pub encrypted_output1: Element,
	pub encrypted_output2: Element,
}

impl<I: Encode, A: Encode, B: Encode, E: Encode> ExtData<I, A, B, E> {
	pub fn new(recipient: I, relayer: I, ext_amount: A, fee: B, encrypted_output1: E, encrypted_output2: E) -> Self {
		Self {
			recipient,
			relayer,
			ext_amount,
			fee,
			encrypted_output1,
			encrypted_output2
		}
	}
}

impl<I: Encode, A: Encode, B: Encode, E: Encode> IntoAbiToken for ExtData<I, A, B, E> {
	fn into_abi(&self) -> Token {
		let recipient = Token::Bytes(self.recipient.encode());
		let ext_amount = Token::Bytes(self.ext_amount.encode());
		let relayer = Token::Bytes(self.relayer.encode());
		let fee = Token::Bytes(self.fee.encode());
		let encrypted_output1 = Token::Bytes(self.encrypted_output1.encode());
		let encrypted_output2 = Token::Bytes(self.encrypted_output2.encode());
		Token::Tuple(vec![
			recipient,
			relayer,
			ext_amount,
			fee,
			encrypted_output1,
			encrypted_output2,
		])
	}
}