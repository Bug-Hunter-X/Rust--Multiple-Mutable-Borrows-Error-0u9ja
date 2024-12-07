fn main() {
    let mut x = 5;
    { // Creating a block scope will solves this error.
        let y = &mut x;
        *y = 10; 
        println!("x = {}", x);
    }
    let mut z = 20;
    let w = &mut z; 
    *w = 30; 
    println!("z = {}", z);
}