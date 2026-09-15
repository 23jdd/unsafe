use std::alloc::{alloc, Layout, dealloc};
use std::io::stdout;
use std::marker::PhantomData;
use std::ptr;
use std::ptr::null_mut;
use std::thread::sleep;

/// LinkedList
/// # EXAMPLE
/// ```rs
///  let mut l = LinkedList::new();
///  l.push_back(2);
///  l.push_back(3);
///  assert_eq!(l.get(0), Some(&2));
///  assert_eq!(l.get(1), Some(&3));
///  assert_eq!(l.front(), Some(&2));
///  assert_eq!(l.back(), Some(&3));
///  l.clear();
///  assert!(l.is_empty())
/// ```
pub struct LinkedList<T> where T:Default{
    head:*mut Node<T>,
    tail:* mut Node<T>,
    len:usize,
    _mark:PhantomData<T>
}
struct Node<T> where T:Default{
    next:* mut Node<T>,
    pre:*mut Node<T>,
    value:T,
}
impl <T> Node<T> where T:Default{
    fn new(value: T) -> Self {
        Node {
            next: null_mut(),
            pre: null_mut(),
            value
        }
    }
    fn default()->Self{
        Self::new(T::default())
    }
}
impl<T> LinkedList<T> where T:Default{
    pub fn new() -> Self{
        unsafe {
            let layout=Layout::new::<Node<T>>();
            let head= alloc(layout) as *mut Node<T>;
            let tail=alloc(layout) as *mut Node<T>;
            ptr::write(head, Node::default());
            ptr::write(tail,Node::default());
            (*head).next=tail;
            (*tail).pre=head;
            Self{
                head,
                tail,
                len:0,
                _mark:PhantomData
            }
        }

    }

    pub fn len(&self) -> usize{
        self.len
    }

    pub fn is_empty(&self) -> bool{
        self.len==0
    }

    pub fn push_front(&mut self, value: T)->bool{
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout) as *mut Node<T>;
            if ptr.is_null(){
                false
            }else{
                //  head->next ptr  head-><-ptr-><-next
                ptr::write(ptr, Node::new(value));
                let next=(*self.head).next;
                (*self.head).next=ptr;
                (*ptr).next=next;
                (*ptr).pre=self.head;
                (*next).pre=ptr;
                self.len+=1;
                true
            }
        }

    }

    pub fn push_back(&mut self, value: T)->bool{
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout) as *mut Node<T>;
            if ptr.is_null(){
                false
            }else{
                ptr::write(ptr, Node::new(value));
                //    pre-><-ptr-><-tail
                let pre=(*self.tail).pre;
                (*self.tail).pre=ptr;
                (*ptr).pre=pre;
                (*ptr).next=self.tail;
                (*pre).next=ptr;
                self.len+=1;
                true
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T>{
        if self.len<=0{
            None
        }else{
            unsafe {
                // head->ptr->next
                let ptr=(*self.head).next;
                (*self.head).next=(*ptr).next;
                (*(*ptr).next).pre=self.head;
                let raw = Box::from_raw(ptr);
                self.len-=1;
                Some(raw.value)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T>{
        if self.len<=0{
            None
        }else{
            unsafe {
                let ptr=(*self.tail).pre;
                (*self.tail).pre=(*ptr).pre;
                (*(*ptr).pre).next=self.tail;
                let raw = Box::from_raw(ptr);
                self.len-=1;
                Some(raw.value)
            }
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.len() > 0 {
            unsafe {
                Some(&(*(*self.head).next).value)
            }
        } else {
            None
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T>{
        if self.len() > 0 {
            unsafe {
                Some(& mut (*(*self.head).next).value)
            }
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&T>{
        if self.len() > 0 {
            unsafe {
                Some(&(*(*self.tail).pre).value)
            }
        } else {
            None
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T>{
        if self.len() > 0 {
            unsafe {
                Some(& mut (*(*self.tail).pre).value)
            }
        } else {
            None
        }
    }

    pub fn clear(&mut self){
        while let Some(c)=self.pop_back(){

        }
    }
    pub fn get(&self,index:usize)->Option<&T>{
        if index<self.len{
            let mut p=self.head;
            for  i in 0..=index {
                unsafe {
                    p=(*p).next;
                }
            }
            Some(unsafe{&(*p).value})
        }else{
            None
        }
    }
    pub fn get_mut(&mut self,index:usize)->Option<&mut T>{
        if index<self.len{
            let mut p=self.head;
            for  i in 0..=index {
                unsafe {
                    p=(*p).next;
                }
            }
            Some(unsafe{& mut(*p).value})
        }else{
            None
        }
    }
}
impl<T> Drop for LinkedList<T>where T:Default{
    fn drop(&mut self) {
        self.clear();
        unsafe{
             ptr::drop_in_place(self.head);
             ptr::drop_in_place(self.tail);
             dealloc(self.head as *mut u8, Layout::new::<Node<T>>());
             dealloc(self.tail as *mut u8, Layout::new::<Node<T>>());
        }
    }
}