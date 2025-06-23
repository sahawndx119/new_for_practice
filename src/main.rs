// in the name of God 

use practice::Person;

fn main () {
    println!("hello please enter your name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();


    println!("please enter your age");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age = age.trim().parse::<u32>().unwrap();



    println!("please enter your city");
    let mut city = String::new();
    std::io::stdin().read_line(&mut city).unwrap();


    let person = Person::new(name.trim(), age, city.trim());
    println!("created a struct type named person");

    println!("name >> {}" , person.name);
    println!("age >> {}" , person.age);
    println!("city >> {}" , person.city);

}