fn main() {
    let greeting = String::from("hello world");

let char1 = greeting.chars().nth(1);

match char1{
    Some(c)=> println!("{}",c),
    None=> println!("Character not found")
}
}
