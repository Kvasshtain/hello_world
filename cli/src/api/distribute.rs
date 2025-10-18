use {
    crate::{context::Context, api::transfer_from, api::{create, transfer}},
    solana_sdk::{
        pubkey::Pubkey,
        signer::Signer,
        signature::Signature,
    },
};

pub async fn batch<'a>(
    seed_prefix: String,
    seeds: Vec<String>,
    context: &Context<'a>,
    generation: u64,
    rent: u64,
) {
    println!("ENTER");

    let mut stop_flag = true;

    let mut index: u64 = 0;

    let mut create_futures: Vec<_> = vec![];
    let mut transfer_futures: Vec<_> = vec![];

    let mut next_seeds: Vec<String> = seeds.clone();

    for from_seed in seeds.clone() {

        let (from, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[&*from_seed.as_bytes()], &context.program_id);

        println!("from pubkey: {}", from);

        let balance = context.client.get_balance(&from).await.unwrap() - rent;

        if balance == 1 {
            continue;
        } else {
            stop_flag = false;
        }

        let half = balance as f64 / 2.0;

        let to_seed: String = seed_prefix.clone() + "-" + &generation.to_string() + "-" + index.to_string().as_str();

        println!("to seed: {}", to_seed);

        let (to, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[to_seed.as_bytes()], &context.program_id);

        println!("to pubkey: {}", to);

        create_futures.push(create(context, to_seed.clone(), 0, context.program_id));

        transfer_futures.push(transfer_from(context, half.ceil() as u64, from_seed.clone(), from, to));

        next_seeds.push(to_seed);

        index = index + 1;
    }

    let create_results = futures_util::future::join_all(create_futures).await;
    let transfer_results = futures_util::future::join_all(transfer_futures).await;

    for result in create_results {
        println!("create signature: {}", result.unwrap());
    }

    for result in transfer_results {
        println!("transfer signature: {}", result.unwrap());
    }

    if stop_flag {
        println!("STOP");
        return;
    }

    println!("CONTINUE");

    Box::pin(batch(seed_prefix, next_seeds, context, generation + 1, rent)).await;
}

pub async fn distribute<'a>(
    context: &Context<'a>,
    seed_prefix: String,
    amount: u64,
) -> anyhow::Result<Signature> {
    let index: u64 = 0;

    let generation: u64 = 0;

    let seed: String = seed_prefix.clone() + "-" + &generation.to_string() + "-" + index.to_string().as_str();

    println!("user pubkey: {}", context.keypair.pubkey());

    println!("genesis_account seed: {}", seed);

    let (new, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[&*seed.as_bytes()], &context.program_id);

    println!("genesis_account pubkey: {}", new);

    let next_seeds: Vec<String> = vec![seed];

    println!("first transfer amount: {}", amount);

    println!("CREATE");

    let _create_result = create(context, seed_prefix.clone() + "-" + &generation.to_string() + "-" + index.to_string().as_str(), 0, context.program_id).await?;

    let transfer_result = transfer(context, amount, new).await?;

    let rent = context.client.get_minimum_balance_for_rent_exemption(0).await?;

    batch(seed_prefix, next_seeds, context, generation + 1, rent).await;

    Ok(transfer_result)
}