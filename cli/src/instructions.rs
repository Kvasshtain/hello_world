use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn build_ix(
    program_id: Pubkey,
    data: &[u8],
    payer: Pubkey,
    pubkeys: &[&Pubkey],
) -> Instruction {
    let mut accounts: Vec<AccountMeta> = pubkeys
        .iter()
        .map(|&pubkey| AccountMeta::new(*pubkey, false))
        .collect();

    accounts.insert(0, AccountMeta::new(payer, true));
    accounts.push(AccountMeta::new_readonly(
        solana_sdk::system_program::id(),
        false,
    ));

    Instruction {
        program_id,
        accounts,
        data: data.to_vec(),
    }
}
