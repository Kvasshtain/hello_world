pub mod account_state;
pub mod holder_data;
pub mod holder_lock;
pub mod ver;
pub mod account_type;
pub mod account_lock;

use std::ptr;
use {
    crate::error::Error::InvalidDataLength,
    solana_program::account_info::AccountInfo,
    std::{
        cell::{Ref, RefMut},
        mem::{align_of, size_of},
    },
};

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub trait Data {
    type Item<'a>;
    type ItemMut<'a>;
    fn from_account<'a>(info: &'a AccountInfo) -> Result<Self::Item<'a>>;
    fn from_account_mut<'a>(info: &'a AccountInfo) -> Result<Self::ItemMut<'a>>;
    fn size(info: &AccountInfo) -> usize;
    fn offset(info: &AccountInfo) -> usize;
}

pub fn cast<'a, T>(info: &'a AccountInfo, offset: usize, len: usize) -> Result<Ref<'a, T>> {
    assert_eq!(align_of::<T>(), 1);

    let data = info.data.borrow();

    if data.len() < offset + len {
        return Err(InvalidDataLength(*info.key, data.len(), offset + len));
    }

    let data = Ref::map(data, |a| &a[offset..offset + len]);
    assert_eq!(data.len(), size_of::<T>());

    let state = Ref::map(data, |a| {
        let ptr = a.as_ptr().cast::<T>();
        unsafe { &*ptr }
    });

    Ok(state)
}

pub fn cast_mut<'a, T>(info: &'a AccountInfo, offset: usize, len: usize) -> Result<RefMut<'a, T>> {
    assert_eq!(align_of::<T>(), 1);

    let data = info.data.borrow_mut();

    if data.len() < offset + len {
        return Err(InvalidDataLength(*info.key, data.len(), offset + len));
    }

    let data = RefMut::map(data, |a| &mut a[offset..offset + len]);
    assert_eq!(data.len(), size_of::<T>());

    let state = RefMut::map(data, |a| {
        let ptr = a.as_mut_ptr().cast::<T>();
        unsafe { &mut *ptr }
    });

    Ok(state)
}

fn cast_slice<'a, T>(info: &'a AccountInfo, offset: usize, count: usize) -> Result<Ref<'a, [T]>> {
    assert_eq!(align_of::<T>(), 1);

    let data = info.data.borrow();

    let len = count * size_of::<T>();
    if data.len() < offset + len {
        return Err(InvalidDataLength(*info.key, data.len(), offset + len));
    }

    let data = Ref::map(data, |a| &a[offset..offset + len]);
    assert_eq!(data.len() % size_of::<T>(), 0);

    let slice = Ref::map(data, |a| {
        let ptr = a.as_ptr().cast::<T>();
        unsafe { &*ptr::slice_from_raw_parts(ptr, count) }
    });

    Ok(slice)
}

fn cast_slice_mut<'a, T>(
    info: &'a AccountInfo,
    offset: usize,
    count: usize,
) -> Result<RefMut<'a, [T]>> {
    assert_eq!(align_of::<T>(), 1);

    let data = info.data.borrow_mut();

    let len = count * size_of::<T>();
    if data.len() < offset + len {
        return Err(InvalidDataLength(*info.key, data.len(), offset + len));
    }

    let data = RefMut::map(data, |a| &mut a[offset..offset + len]);
    assert_eq!(data.len() % size_of::<T>(), 0);

    let slice = RefMut::map(data, |a| {
        let ptr = a.as_mut_ptr().cast::<T>();
        unsafe { &mut *ptr::slice_from_raw_parts_mut(ptr, count) }
    });

    Ok(slice)
}
