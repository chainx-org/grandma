// --- crates.io ---
use parity_scale_codec::Decode;
// --- grandma ---
use crate::primitives::AccountId;

pub type QueuedKeys<SK> = Vec<(AccountId, SK)>;

pub trait SessionKeys: Decode {
	fn grandpa(&self) -> &AccountId;
}

#[derive(Debug, Decode)]
pub struct PolkadotSessionKeys {
	pub grandpa: AccountId,
	pub babe: AccountId,
	pub im_online: AccountId,
	pub para_validator: AccountId,
	pub para_assignment: AccountId,
	pub authority_discovery: AccountId,
}
impl SessionKeys for PolkadotSessionKeys {
	fn grandpa(&self) -> &AccountId {
		&self.grandpa
	}
}

#[derive(Debug, Decode)]
pub struct ChainXSessionKeys {
	pub babe: AccountId,
	pub grandpa: AccountId,
	pub im_online: AccountId,
	pub authority_discovery: AccountId,
}
impl SessionKeys for ChainXSessionKeys {
	fn grandpa(&self) -> &AccountId {
		&self.grandpa
	}
}


#[derive(Debug, Decode)]
pub struct ChainXValidator {
	pub registered_at: u32,
	pub is_chilled: bool,
	pub last_chilled: Option<u32>,
	pub referral_id: Vec<u8>,
}
