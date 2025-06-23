// in the name of God 

fn main () {
    println!("hello please enter your name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("name >> {}" , name.trim());

    println!("please enter your age");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age = age.trim().parse::<u32>().unwrap();
    

    println!("age >> {age}");



    let mut city = String::new();
    std::io::stdin().read_line(&mut city).unwrap();
    let city = city.trim();

    println!("city >> {city}");
}