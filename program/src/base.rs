use solana_pubkey::Pubkey;

pub struct Base<'a> {
    pub program_id: &'a Pubkey,
}

impl<'a> Base<'a> {
    pub fn new(program_id: &'a Pubkey) -> Self {
        Self { program_id }
    }
}
