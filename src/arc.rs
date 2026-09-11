use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{fence, AtomicUsize, Ordering};

struct ArcInner<T> {
    count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: *mut ArcInner<T>,
    _marker: PhantomData<ArcInner<T>>,
}

impl<T> Arc<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<ArcInner<T>>();

        unsafe {
            let ptr = alloc(layout) as *mut ArcInner<T>;

            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            ptr::write(
                ptr,
                ArcInner {
                    count: AtomicUsize::new(1),
                    data: value,
                },
            );

            Self {
                ptr,
                _marker: PhantomData,
            }
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr)
                .count
                .fetch_add(1, Ordering::Relaxed);
        }

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.ptr).data
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        unsafe {
            // fetch_sub 返回减之前的值
            if (*self.ptr)
                .count
                .fetch_sub(1, Ordering::Release)
                != 1
            {
                return;
            }

            // 我们是最后一个 Arc
            fence(Ordering::Acquire);

            let layout = Layout::new::<ArcInner<T>>();

            // 先执行 ArcInner<T> 的析构
            ptr::drop_in_place(self.ptr);

            // 再释放内存
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}