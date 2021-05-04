fn main() 
{
    println!("Hello, world!");
    let x = another_function(5, 6);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) -> i32
{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y // Sem ;
}