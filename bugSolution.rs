fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and preferred way to modify the vector
    println!("First element: {}", v[0]);
}