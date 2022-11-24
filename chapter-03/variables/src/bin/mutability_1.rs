fn main() {
    let x = 5;

    println!("The value of x is: {}", x);

    //| x = 6;
    //| ^^^^^ cannot assign twice to immutable variable
    //For more information about this error, try `rustc --explain E0384`.
    
    println!("The value of x is: {}", x);
}
