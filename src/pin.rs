use std::ops::{Deref, DerefMut};

struct Pin<P>{
    ptr:P
}
impl<P> Pin<P> {
    // 安全构造：只有 Target: Unpin 才允许
    pub fn new(ptr: P) -> Self
    where
        P: Deref,
        P::Target: Unpin,
    {
        Self { ptr }
    }

    // 非安全构造：调用者保证以后不会移动 Target
    pub unsafe fn new_unchecked(ptr: P) -> Self {
        Self { ptr }
    }
}
impl<P: Deref> Pin<P> {
    // Pin<&i32> -> Pin<&i32>
    // Pin<Box<i32>>->Pin<&i32>
    // let a=Pin::new(&1)
    //
    pub fn as_ref(& self) -> Pin<&P::Target> {
        unsafe {
            Pin::new_unchecked(&*self.ptr)
        }
    }
}

impl<P: DerefMut> Pin<P> {
    pub fn as_mut(&mut self) -> Pin<&mut P::Target> {
        unsafe {
            Pin::new_unchecked(&mut *self.ptr)
        }
    }
}
impl<'a, T: ?Sized> Pin<&'a T> {
    pub fn get_ref(self) -> &'a T {
        self.ptr
    }
}

impl<'a, T: ?Sized> Pin<&'a mut T> {
    pub fn get_mut(self) -> &'a mut T
    where
        T: Unpin,
    {
        self.ptr
    }

    pub unsafe fn get_unchecked_mut(self) -> &'a mut T {
        self.ptr
    }
}
impl<P: Deref> Pin<P> {
    pub fn into_inner(self) -> P
    where
        P::Target: Unpin,
    {
         self.ptr
    }

    pub unsafe fn into_inner_unchecked(self) -> P {
        self.ptr
    }
}
impl<P> Deref for Pin<P>
where
    P: Deref,
{
    type Target = P::Target;

    fn deref(&self) -> &Self::Target {
        &*self.ptr
    }
}

impl<P> DerefMut for Pin<P>
where
    P: DerefMut,
    P::Target: Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.ptr
    }
}