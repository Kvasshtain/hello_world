use {crate::context::Context, anyhow::Result, solana_sdk::signature::Signature};

pub async fn create_send_tx<'a>(
    context: &Context<'a>,
    ixs: &[solana_sdk::instruction::Instruction],
) -> Result<Signature> {
    let tx = context.compose_tx(ixs).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
