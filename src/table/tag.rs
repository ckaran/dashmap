use flize::{
    generic_array::{typenum::U2, GenericArray},
    Tag,
};

#[derive(Clone, Copy)]
pub struct BTag {
    pub tombstone: bool,
    pub resize:    bool,
}

impl Tag for BTag {
    type Size = U2;

    fn deserialize(bits: GenericArray<bool, Self::Size>) -> Self {
        Self {
            tombstone: bits[0],
            resize:    bits[1],
        }
    }

    fn serialize(self) -> GenericArray<bool, Self::Size> {
        GenericArray::from([self.tombstone, self.resize])
    }
}
