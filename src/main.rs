mod once;

use once::Once;


fn main() {
    let once = Once::new();
    once.call_one(||{
         println!("Once called");
    });
    once.call_one(||{
         println!("Once called");
    })
}