use std::alloc::{alloc, Layout, handle_alloc_error, dealloc};
use std::marker::PhantomData;
use std::ptr;

pub struct Cell<T>{
    ptr: *mut T,
    _marker: PhantomData<T>,
}
impl<T> Cell<T> {
    pub fn new(val:T)->Self{
         let layout=Layout::new::<T>();
         unsafe{
             let ptr = alloc(layout) as *mut T;
             if ptr.is_null(){
                  handle_alloc_error(layout);
             }
             ptr::write(ptr,val);
             Self{
                 ptr,
                 _marker: PhantomData,
             }
         }
     }
     pub fn set(&self,val:T){
           unsafe {
                ptr::drop_in_place(self.ptr);
                ptr::replace(self.ptr,val);
           }
     }
     pub fn replace(&self,val:T)->T{
         unsafe {
             ptr::drop_in_place(self.ptr);
             ptr::replace(self.ptr,val)
         }
     }
    pub fn get_mut(&mut self)->&mut T {
        unsafe {
            &mut (*self.ptr)
        }
    }
}
impl<T> Cell<T> where T:Copy{
     pub fn get(&self)->T{
            unsafe {
                *(self.ptr).clone()
            }
      }
}
impl<T> Drop for Cell<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }
}