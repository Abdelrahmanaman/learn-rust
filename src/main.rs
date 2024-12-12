use std::collections::HashMap;

fn main () {
   let mut users: HashMap<String, i32> = HashMap::new();

   users.insert(String::from("Abdelrahman"), 22);
   users.insert(String::from("Ab"), 25);

   let user1 = users.get("Abdelrahman");

   match user1 {
       Some (age) => println!("user is {}", age),
       None => println!("User not found")
   }


}
