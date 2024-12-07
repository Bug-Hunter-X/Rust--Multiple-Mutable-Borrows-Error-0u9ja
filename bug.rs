fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; //This line is correct
    let z = y; // This line causes the error
    *z = 20;// This line causes the error
    println!("x = {}", x);
}