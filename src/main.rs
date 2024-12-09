struct Point<T> {
    x: T,
    y: T,
}
fn main () {
    let integer_point = Point{x:5, y:5};
    println!("{}", integer_point.x)
}
