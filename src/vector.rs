use std::alloc::{
    alloc, dealloc, handle_alloc_error, realloc, Layout,
};
use std::marker::PhantomData;
use std::mem::{size_of, ManuallyDrop};
use std::ptr::{self, NonNull};

const INIT_CAP: usize = 16;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,

    // 表示 Vec 逻辑上拥有 T
    _mark: PhantomData<T>,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        // ZST，例如 ()
        if size_of::<T>() == 0 {
            return Self {
                ptr: NonNull::<T>::dangling().as_ptr(),
                len: 0,
                cap: usize::MAX,
                _mark: PhantomData,
            };
        }

        let layout = Layout::array::<T>(INIT_CAP)
            .expect("layout overflow");

        unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            Self {
                ptr,
                len: 0,
                cap: INIT_CAP,
                _mark: PhantomData,
            }
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            /*
                ptr.add(self.len)
                找到下一个未初始化的位置。

                write(value)
                把 value 移动进去，不会尝试 drop 原位置。
            */
            self.ptr.add(self.len).write(value);
        }

        self.len += 1;
    }

    fn grow(&mut self) {
        // ZST 不需要真正分配内存
        if size_of::<T>() == 0 {
            panic!("capacity overflow");
        }

        let old_cap = self.cap;

        let new_cap = old_cap
            .checked_mul(2)
            .expect("capacity overflow");

        let old_layout =
            Layout::array::<T>(old_cap).expect("layout overflow");

        let new_layout =
            Layout::array::<T>(new_cap).expect("layout overflow");

        unsafe {
            let new_ptr = realloc(
                self.ptr as *mut u8,
                old_layout,
                new_layout.size(),
            ) as *mut T;

            if new_ptr.is_null() {
                handle_alloc_error(new_layout);
            }

            self.ptr = new_ptr;
            self.cap = new_cap;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            Some(&*self.ptr.add(index))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            Some(&mut *self.ptr.add(index))
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {

            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }

            if size_of::<T>() != 0 {
                let layout =
                    Layout::array::<T>(self.cap)
                        .expect("layout overflow");

                dealloc(
                    self.ptr as *mut u8,
                    layout,
                );
            }
        }
    }
}


pub struct Iter<'a, T> {
    ptr: *const T,

    // 当前下标
    index: usize,

    // 一共有多少个元素
    len: usize,

    /*
        让编译器知道：

        Iter<'a, T>

        逻辑上持有生命周期为 'a 的 &T。
    */
    _mark: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        unsafe {
            let ptr = self.ptr.add(self.index);

            self.index += 1;

            Some(&*ptr)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;

        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            ptr: self.ptr,
            index: 0,
            len: self.len,
            _mark: PhantomData,
        }
    }
}


pub struct IntoIter<T> {
    // allocation 最初的位置
    ptr: *mut T,

    // 下一个需要取出的元素
    index: usize,

    // 原来的 len
    len: usize,

    // 原来的 capacity
    cap: usize,

    _mark: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        unsafe {
            let ptr = self.ptr.add(self.index);

            self.index += 1;


            Some(ptr::read(ptr))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;

        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        unsafe {

            for i in self.index..self.len {
                ptr::drop_in_place(
                    self.ptr.add(i),
                );
            }

            if size_of::<T>() != 0 {
                let layout =
                    Layout::array::<T>(self.cap)
                        .expect("layout overflow");

                dealloc(
                    self.ptr as *mut u8,
                    layout,
                );
            }
        }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let vec = ManuallyDrop::new(self);

        IntoIter {
            ptr: vec.ptr,
            index: 0,
            len: vec.len,
            cap: vec.cap,
            _mark: PhantomData,
        }
    }
}

