use std::collections::HashMap;

fn main () {

    println!("Hello App")



}


/*
Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a Hashmaps where the keys are the unique keys from the
input tuples and the values are vectors of all corresponding values associated
with each key.
*/

fn return_tuples(tuples: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for t in tuples {

            map.insert(t.0, t.1);

    }
    return map;
}
