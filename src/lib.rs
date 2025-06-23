//in the name of God

pub struct Person<'a> {
    pub name : &'a str,
    pub age : u32,
    pub city : &'a str,
}


impl<'a> Person<'a> {
    pub fn new(name : &'a str , age : u32 , city : &'a str) -> Self {
        Person {
            name,
            age,
            city,
        }
    }
}