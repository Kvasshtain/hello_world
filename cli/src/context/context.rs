use {
    anyhow::Result,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signature::Signer,
        transaction::Transaction,
    },
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
};

pub struct Context<'a> {
    pub program_id: Pubkey,
    pub keypair: Keypair,
    pub client: &'a RpcClient,
}

impl<'a> Context<'a> {
    pub fn new(program_id: Pubkey, keypair: Keypair, client: &'a RpcClient) -> Result<Self> {
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
}
