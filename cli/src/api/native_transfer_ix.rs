use {
    crate::context::Context, anyhow::Result, hello_world::Instruction, solana_sdk::pubkey::Pubkey,
};

pub async fn native_transfer_ix<'a>(
    context: &Context<'a>,
    amount: u64,
    to: Pubkey,
) -> Result<solana_sdk::instruction::Instruction> {
    let mut data = vec![Instruction::Transfer as u8];

    data.extend(to.to_bytes());

    data.extend(amount.to_le_bytes());

    Ok(context.compose_ix(&data.as_slice(), &[&to]))
}
