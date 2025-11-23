use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Default, Clone)]
pub struct Seed {
    pub items: Vec<Vec<u8>>,
}
impl Seed {
    pub fn cast(&self) -> Vec<&[u8]> {
        self.items
            .iter()
            .map(|a| a.as_slice())
            .collect::<Vec<&[u8]>>()
    }
    pub fn add(&mut self, bump_seed: u8) {
        self.items.push(vec![bump_seed]);
    }
    pub fn from_vec(slice: Vec<&[u8]>) -> Self {
        let items = slice.iter().map(|&a| a.to_vec()).collect::<Vec<_>>();

        Self { items }
    }
}
