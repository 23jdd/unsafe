use std::alloc::{alloc, Layout, handle_alloc_error, dealloc};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;
pub struct Refcell<T>{
      ptr :*mut T,
      _mark:PhantomData<T>,
      count:*mut u64, // `Ref` count
      m_count:*mut u64, // `RefMut` count
}
impl<T> Refcell<T> {
     pub fn new(val:T)->Self{
              let layout=Layout::new::<T>();
              let count_layout=Layout::new::<u64>();
              unsafe{
                  let ptr = alloc(layout) as *mut T;
                  let count=alloc(count_layout) as *mut u64;
                  let m_count=alloc(count_layout) as *mut u64;
                  if ptr.is_null() || count.is_null() ||m_count.is_null(){
                       handle_alloc_error(layout);
                  }
                  ptr::write(ptr, val);
                  ptr::write(count, 0);
                  ptr::write(m_count, 0);
                  Self{
                      ptr,count,m_count,
                      _mark:PhantomData,
                  }
              }
     }
     pub fn borrow(&self) ->Ref<T> {
          unsafe {
              if *self.m_count>0{
                  panic!("cannot borrow mutably borrowed value");
              }
              *self.count+=1;
              Ref{
                  ptr:self.ptr,
                  count:& mut *(self.count),
                  _mark:PhantomData
              }
          }

     }
     pub fn borrow_mut(& self)->RefMut<T>{
         unsafe {
             if *self.count>0 || *self.m_count!=0{
                  panic!("cannot mutably borrowed value");
             }
             *self.m_count+=1;
             RefMut{
                 ptr:self.ptr,
                 count:& mut *(self.m_count),
                 _mark:PhantomData
             }
         }
     }
}
impl<T> Drop for Refcell<T>{
     fn drop(&mut self){
             unsafe {
                    ptr::drop_in_place(self.ptr);
                    dealloc(self.ptr as *mut u8,Layout::new::<T>());
                    dealloc(self.count as *mut u8,Layout::new::<u64>());
                    dealloc(self.m_count as *mut u8,Layout::new::<u64>());
             }
     }
}
pub struct Ref<'a,T> {
    ptr: *const T,
    count:&'a mut u64,
    _mark: PhantomData<T>
}
impl<'a,T> Deref for Ref<'a,T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
           unsafe {
                 &(*self.ptr)
           }
    }
}
impl<'a,T> Drop for Ref<'a,T>{
    fn drop(&mut self) {
        *self.count-=1
    }
}
pub struct RefMut<'a,T> {
    ptr: *mut T,
    count: &'a mut u64,
    _mark: PhantomData<T>
}
impl<'a,T> Deref for RefMut<'a,T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.ptr)
        }
    }
}
impl <'a,T>DerefMut for RefMut<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            & mut (*self.ptr)
        }
    }
}
impl<'a,T> Drop for RefMut<'a,T>{
    fn drop(&mut self) {
          *self.count-=1
    }
}