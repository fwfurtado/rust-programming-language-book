fn main() {
    let x = 5;

    println!("The value of x is: {}", x);

    let x = x + 1;
    
    println!("The value of x is: {}", x);

    {
        let x = 7 * x;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {}", x);

    // We can change data type of a variable using shadowing
    let spacing  = "   ";
    let spacing = spacing.len();
    println!("The value of spacing is: {}", spacing);

}