use cli::MetaMerkleSnapshot;
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;

pub struct ProgramTestContext {
    pub payer: Keypair,
    pub program_config_pda: Pubkey,
    pub operators: Vec<Keypair>,
    pub meta_merkle_snapshot: MetaMerkleSnapshot,
    pub snapshot_slot: u64,
}
