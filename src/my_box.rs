use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::marker::PhantomData;
use std::mem::{forget, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

pub struct Box<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

impl<T> Box<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();

        unsafe {
            let ptr = if layout.size() == 0 {
                NonNull::<T>::dangling().as_ptr()
            } else {
                let ptr = alloc(layout) as *mut T;

                if ptr.is_null() {
                    handle_alloc_error(layout);
                }

                ptr
            };

            ptr::write(ptr, value);

            Self {
                ptr,
                _marker: PhantomData,
            }
        }
    }
    pub fn into_raw(self)->*mut T{
        let this = ManuallyDrop::new(self);
        this.ptr
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            // ① drop T
            ptr::drop_in_place(self.ptr);

            // ② release allocation
            let layout = Layout::new::<T>();

            if layout.size() != 0 {
                dealloc(self.ptr.cast::<u8>(), layout);
            }
        }
    }
}