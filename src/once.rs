use std::sync::atomic::{AtomicBool, Ordering};

pub struct Once{
    called:AtomicBool
}

impl Once {
    pub fn new()->Self{
           Self{
                called: AtomicBool::new(false)
           }  
    }   
    pub fn call_one<T>(&self,f:T)where T:FnOnce(){
        if let Ok(_)=self.called.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst){
               f()   
        } 
    }
}