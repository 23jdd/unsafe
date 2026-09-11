
mod my_box;
mod vector;
mod string;
mod rc;
mod arc;
mod cell;
mod refcell;
use refcell::Refcell;
struct User{

}
impl User{
     fn new()->Self{
           Self{}
     }
     fn check(&self){
         println!("check");
     }
     fn check_mut(& mut self){
          println!("check_mut");
     }
}
impl Drop for User{
    fn drop(&mut self){
         println!("drop");
    }
}
fn main() {
    {
        let refcell = Refcell::new(User::new());
        let mut r= refcell.borrow_mut();
        let mut r1=refcell.borrow();
        r.check();
        r.check_mut()
    }
}