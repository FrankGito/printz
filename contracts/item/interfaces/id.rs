use ink::prelude::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}
impl Default for Id {
    fn default() -> Self {
        Id::U128(0u128)
    }
}
