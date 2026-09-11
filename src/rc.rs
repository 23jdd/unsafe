use std::alloc::{alloc, Layout, handle_alloc_error, dealloc};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr;
use lazy_static::lazy_static;

pub struct Rc<T>{
    ptr: *mut T,
    count:*mut u64,
    _marker: PhantomData<T>,
}
impl <T> Rc<T>{
    pub fn new(val:T)->Self{
        let layout=Layout::new::<T>();
        let count_layout=Layout::new::<u64>();
        unsafe {
            let ptr=alloc(layout) as *mut T;
            if ptr.is_null(){
                 handle_alloc_error(layout);
            }
            let count=alloc(count_layout) as *mut u64;
            if count.is_null(){
                handle_alloc_error(count_layout);
            }
            ptr.write(val);
            count.write(1);
            Rc{
                ptr,
                count,
                _marker:PhantomData,
            }
        }
    }
    pub fn clone(&self)->Self{
          unsafe {
               *self.count+=1
          }
          Self{
              ptr:self.ptr,
              _marker:PhantomData,
              count:self.count,
          }
    }
    pub fn strong_count(&self)->u64{
              unsafe{
                    *self.count
              }
    }
    pub fn downgrade(this:&Self){
         
    }
}
impl<T> Drop for Rc<T>{
    fn drop(&mut self) {
         unsafe {
             *self.count -= 1;
             if *self.count == 0 {
                  ptr::drop_in_place(self.ptr);
                  ptr::drop_in_place(self.count);
                  dealloc(self.ptr as *mut u8,Layout::new::<T>());
                  dealloc(self.count as *mut u8,Layout::new::<u64>());
             }
         }
    }
}
impl<T> Deref for Rc<T>{
    type Target =T;
    fn deref(&self) -> &Self::Target {
         unsafe {
               & (*self.ptr)
         }
    }
}
