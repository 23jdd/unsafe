use std::borrow::{Borrow, ToOwned};
use std::ops::Deref;

enum MyCow<'a, B>
where
    B: ?Sized + ToOwned,
{
    Borrowed(&'a B),
    Owned(B::Owned),
}

impl<'a, B> Deref for MyCow<'a, B>
where
    B: ?Sized + ToOwned,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match self {
            MyCow::Borrowed(v) => *v,
            MyCow::Owned(v) => v.borrow(),
        }
    }
}