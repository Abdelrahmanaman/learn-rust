

fn main () {
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(43);
    vec.push(4);
    vec.push(3);
    vec.push(6);
    let vec2 = return_even(vec);
    println!("{:?}", vec2);
}

fn return_even(vec: Vec<i32>) -> Vec<i32> {
    let mut even_vec =  Vec::new();
    for i in vec{
        if i % 2 == 0 {
             even_vec.push(i);
        }
    }
    Vec::new()
}
