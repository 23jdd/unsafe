use super::vector::Vec;
pub type Str=Vec<char>;
impl Str{
     pub fn from(s:&str)->Self{
         let mut str = Str::new(); 
         for ch in s.chars() {
              str.push(ch)
         }
         str
     }
}