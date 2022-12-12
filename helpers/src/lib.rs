use std::collections::*;

pub trait CollectVec: Iterator + Sized {
    fn collectvec(self) -> Vec<Self::Item> {
        self.collect()
    }
    fn collectset(self) -> BTreeSet<Self::Item>
    where
        Self::Item: Ord,
    {
        self.collect()
    }
}
impl<I: Iterator> CollectVec for I {}
