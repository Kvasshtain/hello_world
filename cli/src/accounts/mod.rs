pub mod account_state;

use {
    anyhow::Result,
    std::{
        cell::Ref,
        mem::{align_of, size_of},
    },
};

pub trait Data {
    type Item<'a>;
    async fn from_arr<'a>(data: Ref<'a, &[u8]>) -> Result<Self::Item<'a>>;
}

pub async fn cast<'a, T: 'a>(data: Ref<'a, &[u8]>, offset: usize, len: usize) -> Result<Ref<'a, T>> {
    assert_eq!(align_of::<T>(), 1);

    if data.len() < offset + len {
        return Err(anyhow::Error::msg("Wrong data length"));
    }

    let data = Ref::map(data, |a| &a[offset..offset + len]);
    assert_eq!(data.len(), size_of::<T>());

    let state = Ref::map(data, |a| {
        let ptr = a.as_ptr().cast::<T>();
        unsafe { &*ptr }
    });

    Ok(state)
}

