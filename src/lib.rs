pub mod once;
pub mod arc;
pub mod cell;
pub mod my_box;
pub mod refcell;
pub mod rc;
pub mod string;
pub mod vector;
pub mod linked_list;
pub mod pin;
mod cow;
#[cfg(test)]
mod test {
    use std::arch::x86_64::_mm256_maddubs_epi16;
    use std::borrow::Cow;
    use std::cell::OnceCell;
    use std::marker::PhantomPinned;
    use std::ops::{Add, Deref};
    use std::pin::{pin, Pin};
    use std::sync::{Once, OnceLock};
    use rand::distributions::uniform::SampleBorrow;
    use crate::linked_list::LinkedList;

    #[test]
    #[allow(unused_assignments)]
    fn test() {

    }
}
