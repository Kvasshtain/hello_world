use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn allocate<'a>(context: Context<'a>, seed: String, size: u64) -> Vec<Result<Signature>> {
    let mut data = vec![Instruction::Alloc as u8];

    data.extend(size.to_le_bytes());

    data.extend(seed.as_bytes());

    let (resized, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &context.program_id);

    let ix = context.compose_ix(&data.as_slice(), &[&resized]);

    let tx = context.compose_tx(&[ix]).await.unwrap();

    vec![Ok(context
        .client
        .send_and_confirm_transaction(&tx)
        .await
        .unwrap())]
}
