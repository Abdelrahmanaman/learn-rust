use std::fs;


fn main () {
    let res =read_file_unsafe("test.txt".to_string());
    println!("{}", res);
    
    print!("Hello, world!");
}


fn read_file_unsafe(file_content: String) -> String {
    let res = fs::read_to_string(file_content);
    return res.unwrap()
}