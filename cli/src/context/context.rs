use {
    anyhow::Result,
    hello_world::{
        accounts::{account_state::AccountState, Data},
        State,
    },
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_associated_token_account::solana_program,
};

#[derive(Clone)]
pub struct Context<'a> {
    pub program_id: Pubkey,
    pub keypair: &'a Keypair,
    pub client: &'a RpcClient,
}

impl<'a> Context<'a> {
    pub fn new(program_id: Pubkey, keypair: &'a Keypair, client: &'a RpcClient) -> Result<Self> {
        Ok(Self {
            program_id,
            keypair,
            client,
        })
    }

    pub fn compose_ix(&self, data: &[u8], pubkeys: &[&Pubkey]) -> Instruction {
        let mut accounts: Vec<AccountMeta> = pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new(*pubkey, false))
            .collect();

        accounts.insert(0, AccountMeta::new(self.keypair.pubkey(), true));
        accounts.push(AccountMeta::new_readonly(
            solana_sdk::system_program::id(),
            false,
        ));

        Instruction {
            program_id: self.program_id,
            accounts,
            data: data.to_vec(),
        }
    }

    pub async fn compose_tx(&self, ixs: &[Instruction]) -> Result<Transaction> {
        let payer_key = self.keypair.pubkey();

        let blockhash = self.client.get_latest_blockhash().await?;

        let mut tx = Transaction::new_with_payer(ixs, Some(&payer_key));

        tx.sign(&[&self.keypair], blockhash);

        Ok(tx)
    }

    pub async fn get_balance(context: Context<'a>, mint: Pubkey) -> Result<u64> {
        let (pubkey, _bump, _seeds) =
            State::balance_pubkey_bump(&context.program_id, &context.keypair.pubkey(), &mint);

        use solana_program::account_info::IntoAccountInfo;

        let acc = context.client.get_account(&pubkey).await?;

        let mut bind = (pubkey, acc);
        let info = bind.into_account_info();

        let account_state = AccountState::from_account_mut(&info)?;

        Ok(account_state.balance)
    }
}
