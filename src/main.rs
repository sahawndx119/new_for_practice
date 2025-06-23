// in the name of God 

fn main () {
    println!("hello please enter your name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("name >> {}" , name.trim());
}